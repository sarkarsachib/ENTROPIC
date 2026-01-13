use crate::constants::*;
use crate::errors::SpatialError;
use crate::terrain_generator::TerrainGenerator;
use crate::{Chunk, ChunkCoord, World};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

/// Priority levels for chunk loading
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Critical = 3, // Player is in/near chunk
    High = 2,     // Visible from player
    Normal = 1,   // Adjacent to visible
    Low = 0,      // Distant
}

/// Manages chunk lifecycle: generation, loading, caching, unloading
#[derive(Clone)]
pub struct ChunkManager {
    world: Arc<RwLock<World>>,
    loaded_chunks: Arc<RwLock<HashMap<ChunkCoord, Arc<Chunk>>>>,
    load_queue: Arc<RwLock<VecDeque<(ChunkCoord, Priority)>>>,
    unload_candidates: Arc<RwLock<Vec<ChunkCoord>>>,
    generator: TerrainGenerator,
    max_loaded_chunks: usize,
    view_distance: u32,
}

impl ChunkManager {
    /// Create a new chunk manager
    pub fn new(world: Arc<RwLock<World>>, view_distance: u32) -> Self {
        let max_loaded = ((view_distance as usize + 1).pow(2) * 2).min(MAX_LOADED_CHUNKS);

        Self {
            world,
            loaded_chunks: Arc::new(RwLock::new(HashMap::new())),
            load_queue: Arc::new(RwLock::new(VecDeque::new())),
            unload_candidates: Arc::new(RwLock::new(Vec::new())),
            generator: TerrainGenerator::new(),
            max_loaded_chunks: max_loaded,
            view_distance,
        }
    }

    /// Create a new chunk manager with custom terrain generator seed
    pub fn with_seed(world: Arc<RwLock<World>>, view_distance: u32, seed: u32) -> Self {
        let max_loaded = ((view_distance as usize + 1).pow(2) * 2).min(MAX_LOADED_CHUNKS);

        Self {
            world,
            loaded_chunks: Arc::new(RwLock::new(HashMap::new())),
            load_queue: Arc::new(RwLock::new(VecDeque::new())),
            unload_candidates: Arc::new(RwLock::new(Vec::new())),
            generator: TerrainGenerator::with_seed(seed),
            max_loaded_chunks: max_loaded,
            view_distance,
        }
    }

    /// Get the terrain generator
    pub fn generator(&self) -> &TerrainGenerator {
        &self.generator
    }

    /// Get current view distance
    pub fn view_distance(&self) -> u32 {
        self.view_distance
    }

    /// Get number of loaded chunks
    pub fn loaded_chunk_count(&self) -> usize {
        self.loaded_chunks.read().unwrap().len()
    }

    /// Get size of load queue
    pub fn load_queue_size(&self) -> usize {
        self.load_queue.read().unwrap().len()
    }

    /// Update visible chunks based on player position
    pub async fn update_visible_chunks(
        &self,
        player_x: f32,
        player_y: f32,
    ) -> Result<(), SpatialError> {
        let player_chunk_x = (player_x / CHUNK_SIZE).floor() as i32;
        let player_chunk_y = (player_y / CHUNK_SIZE).floor() as i32;

        let mut load_queue = self.load_queue.write().unwrap();
        let loaded = self.loaded_chunks.read().unwrap();

        // Queue chunks in view distance with priority
        for dx in -(self.view_distance as i32)..=(self.view_distance as i32) {
            for dy in -(self.view_distance as i32)..=(self.view_distance as i32) {
                let chunk_x = player_chunk_x + dx;
                let chunk_y = player_chunk_y + dy;

                // Skip negative coordinates for now
                if chunk_x < 0 || chunk_y < 0 {
                    continue;
                }

                let coord = ChunkCoord {
                    x: chunk_x as u32,
                    y: chunk_y as u32,
                };

                // Skip already loaded
                if loaded.contains_key(&coord) {
                    continue;
                }

                // Skip if already in queue
                if load_queue.iter().any(|(c, _)| c == &coord) {
                    continue;
                }

                // Calculate priority based on distance
                let distance = (dx.abs().max(dy.abs())) as u32;
                let priority = if distance == 0 {
                    Priority::Critical
                } else if distance <= 1 {
                    Priority::High
                } else if distance <= 2 {
                    Priority::Normal
                } else {
                    Priority::Low
                };

                load_queue.push_back((coord, priority));
            }
        }

        // Sort by priority
        let mut queue: Vec<_> = load_queue.drain(..).collect();
        queue.sort_by_key(|e| std::cmp::Reverse(e.1));
        queue.into_iter().for_each(|e| load_queue.push_back(e));

        Ok(())
    }

    /// Load next chunk from queue
    pub async fn process_load_queue(&self) -> Result<Option<ChunkCoord>, SpatialError> {
        let mut queue = self.load_queue.write().unwrap();

        if let Some((coord, _)) = queue.pop_front() {
            drop(queue); // Release lock before loading

            let chunk = self.load_or_generate_chunk(coord).await?;
            self.loaded_chunks.write().unwrap().insert(coord, Arc::new(chunk));

            return Ok(Some(coord));
        }

        Ok(None)
    }

    /// Load multiple chunks from queue (parallel)
    pub async fn process_load_queue_batch(&self, count: usize) -> Result<Vec<ChunkCoord>, SpatialError> {
        let mut loaded = Vec::new();

        for _ in 0..count {
            if let Some(coord) = self.process_load_queue().await? {
                loaded.push(coord);
            } else {
                break;
            }
        }

        Ok(loaded)
    }

    /// Load chunk from disk or generate if missing
    async fn load_or_generate_chunk(&self, coord: ChunkCoord) -> Result<Chunk, SpatialError> {
        let world = self.world.read().unwrap();

        // Try to load from world
        if let Some(chunk) = world.chunks.get(&coord) {
            return Ok(chunk.clone());
        }

        drop(world);

        // Generate if not found
        let chunk = self.generator.generate_chunk(coord)?;
        Ok(chunk)
    }

    /// Unload chunks exceeding max loaded chunks
    pub fn process_unload_queue(&self) -> Result<Vec<ChunkCoord>, SpatialError> {
        let loaded = self.loaded_chunks.read().unwrap();

        // Mark for unload if count exceeds max
        let mut to_unload = Vec::new();
        if loaded.len() > self.max_loaded_chunks {
            let excess = loaded.len() - self.max_loaded_chunks;

            // Unload least recently used chunks (simplified: oldest entries)
            for (coord, _) in loaded.iter().take(excess) {
                to_unload.push(*coord);
            }
        }

        drop(loaded);

        // Actually unload
        let mut loaded = self.loaded_chunks.write().unwrap();
        for coord in &to_unload {
            loaded.remove(coord);
        }

        Ok(to_unload)
    }

    /// Get loaded chunk by coordinate
    pub fn get_chunk(&self, coord: ChunkCoord) -> Option<Arc<Chunk>> {
        self.loaded_chunks.read().unwrap().get(&coord).cloned()
    }

    /// Get all loaded chunks
    pub fn get_loaded_chunks(&self) -> Vec<Arc<Chunk>> {
        self.loaded_chunks.read().unwrap().values().cloned().collect()
    }

    /// Preload chunks synchronously (for critical areas)
    pub fn preload_chunks(&self, coords: Vec<ChunkCoord>) -> Result<(), SpatialError> {
        for coord in coords {
            // Skip if already loaded
            if self.get_chunk(coord).is_some() {
                continue;
            }

            let chunk = self.generator.generate_chunk(coord)?;
            self.loaded_chunks.write().unwrap().insert(coord, Arc::new(chunk));
        }
        Ok(())
    }

    /// Unload a specific chunk
    pub fn unload_chunk(&self, coord: ChunkCoord) -> Result<(), SpatialError> {
        self.loaded_chunks.write().unwrap().remove(&coord);
        Ok(())
    }

    /// Check if a chunk is currently loaded
    pub fn is_chunk_loaded(&self, coord: ChunkCoord) -> bool {
        self.loaded_chunks.read().unwrap().contains_key(&coord)
    }

    /// Force unload all chunks (for cleanup)
    pub fn unload_all(&self) {
        self.loaded_chunks.write().unwrap().clear();
        self.load_queue.write().unwrap().clear();
    }

    /// Add chunk to load queue with priority
    pub fn queue_chunk(&self, coord: ChunkCoord, priority: Priority) -> Result<(), SpatialError> {
        // Skip if already loaded
        if self.is_chunk_loaded(coord) {
            return Ok(());
        }

        let mut queue = self.load_queue.write().unwrap();
        queue.push_back((coord, priority));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_world() -> Arc<RwLock<World>> {
        Arc::new(RwLock::new(World::new(
            "Test World".to_string(),
            "game1".to_string(),
            10,
            10,
        )))
    }

    #[tokio::test]
    async fn test_chunk_manager_creation() {
        let world = create_test_world();
        let manager = ChunkManager::new(world, 5);

        assert_eq!(manager.view_distance(), 5);
        assert_eq!(manager.loaded_chunk_count(), 0);
        assert_eq!(manager.load_queue_size(), 0);
    }

    #[tokio::test]
    async fn test_chunk_manager_with_seed() {
        let world = create_test_world();
        let manager = ChunkManager::with_seed(world, 5, 42);

        assert_eq!(manager.generator().seed(), 42);
    }

    #[tokio::test]
    async fn test_update_visible_chunks() {
        let world = create_test_world();
        let manager = ChunkManager::new(world, 2);

        manager.update_visible_chunks(100.0, 100.0).await.unwrap();

        // Should queue chunks around (0, 0)
        assert!(manager.load_queue_size() > 0);
    }

    #[tokio::test]
    async fn test_process_load_queue() {
        let world = create_test_world();
        let manager = ChunkManager::new(world, 2);

        manager.update_visible_chunks(100.0, 100.0).await.unwrap();

        let loaded = manager.process_load_queue().await.unwrap();
        assert!(loaded.is_some());
        assert_eq!(manager.loaded_chunk_count(), 1);
    }

    #[tokio::test]
    async fn test_get_chunk() {
        let world = create_test_world();
        let manager = ChunkManager::new(world, 2);

        let coord = ChunkCoord::new(0, 0);
        manager.preload_chunks(vec![coord]).unwrap();

        let chunk = manager.get_chunk(coord);
        assert!(chunk.is_some());
        assert_eq!(chunk.unwrap().coord, coord);
    }

    #[tokio::test]
    async fn test_preload_chunks() {
        let world = create_test_world();
        let manager = ChunkManager::new(world, 2);

        let coords = vec![
            ChunkCoord::new(0, 0),
            ChunkCoord::new(1, 0),
            ChunkCoord::new(0, 1),
        ];

        manager.preload_chunks(coords).unwrap();

        assert_eq!(manager.loaded_chunk_count(), 3);
    }

    #[tokio::test]
    async fn test_unload_chunk() {
        let world = create_test_world();
        let manager = ChunkManager::new(world, 2);

        let coord = ChunkCoord::new(0, 0);
        manager.preload_chunks(vec![coord]).unwrap();

        assert_eq!(manager.loaded_chunk_count(), 1);
        manager.unload_chunk(coord).unwrap();
        assert_eq!(manager.loaded_chunk_count(), 0);
    }

    #[tokio::test]
    async fn test_unload_all() {
        let world = create_test_world();
        let manager = ChunkManager::new(world, 2);

        let coords = vec![
            ChunkCoord::new(0, 0),
            ChunkCoord::new(1, 0),
            ChunkCoord::new(0, 1),
        ];

        manager.preload_chunks(coords).unwrap();
        assert_eq!(manager.loaded_chunk_count(), 3);

        manager.unload_all();
        assert_eq!(manager.loaded_chunk_count(), 0);
    }

    #[tokio::test]
    async fn test_process_unload_queue() {
        let world = create_test_world();
        let manager = ChunkManager::new(world, 1); // Small view distance

        // Preload more than max
        let coords = vec![
            ChunkCoord::new(0, 0),
            ChunkCoord::new(1, 0),
            ChunkCoord::new(2, 0),
        ];

        manager.preload_chunks(coords).unwrap();
        assert_eq!(manager.loaded_chunk_count(), 3);

        let unloaded = manager.process_unload_queue().unwrap();
        assert!(unloaded.len() > 0);
        assert!(manager.loaded_chunk_count() <= manager.max_loaded_chunks);
    }
}
