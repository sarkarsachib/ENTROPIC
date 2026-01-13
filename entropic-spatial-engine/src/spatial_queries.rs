use crate::constants::*;
use crate::errors::SpatialError;
use crate::{Chunk, ChunkCoord, Entity, World};
use std::sync::Arc;

/// High-performance spatial query engine
pub struct SpatialQueries;

impl SpatialQueries {
    /// Find all entities within radius of point
    pub fn query_radius(world: &World, x: f32, y: f32, radius: f32) -> Vec<Arc<Entity>> {
        let mut results = Vec::new();

        // Get affected chunks
        let chunk_x = (x / CHUNK_SIZE).floor() as i32;
        let chunk_y = (y / CHUNK_SIZE).floor() as i32;
        let chunk_radius = (radius / CHUNK_SIZE).ceil() as i32;

        for dx in -chunk_radius..=chunk_radius {
            for dy in -chunk_radius..=chunk_radius {
                let coord = ChunkCoord {
                    x: (chunk_x + dx).max(0) as u32,
                    y: (chunk_y + dy).max(0) as u32,
                };

                if let Some(chunk) = world.chunks.get(&coord) {
                    // Check entities in chunk
                    for entity_id in &chunk.entities {
                        if let Some(entity) = world.entities.get(entity_id) {
                            let dist = ((entity.x - x).powi(2) + (entity.y - y).powi(2)).sqrt();
                            if dist <= radius {
                                results.push(Arc::new(entity.clone()));
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// Find nearest entity to point
    pub fn nearest_entity(
        world: &World,
        x: f32,
        y: f32,
        max_distance: f32,
    ) -> Option<Arc<Entity>> {
        Self::query_radius(world, x, y, max_distance)
            .into_iter()
            .min_by_key(|e| ((e.x - x).powi(2) + (e.y - y).powi(2)) as i32)
    }

    /// Find all entities within radius, sorted by distance
    pub fn query_radius_sorted(
        world: &World,
        x: f32,
        y: f32,
        radius: f32,
    ) -> Vec<Arc<Entity>> {
        let mut results = Self::query_radius(world, x, y, radius);
        results.sort_by_key(|e| ((e.x - x).powi(2) + (e.y - y).powi(2)) as i32);
        results
    }

    /// Get all entities in chunk
    pub fn query_chunk(world: &World, coord: ChunkCoord) -> Vec<Arc<Entity>> {
        world
            .chunks
            .get(&coord)
            .map(|chunk| {
                chunk
                    .entities
                    .iter()
                    .filter_map(|id| world.entities.get(id).cloned().map(Arc::new))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Raycast: find first entity hit by ray
    pub fn raycast(
        world: &World,
        start_x: f32,
        start_y: f32,
        dir_x: f32,
        dir_y: f32,
        max_distance: f32,
    ) -> Option<Arc<Entity>> {
        let mut hits = Vec::new();

        for entity in world.entities.values() {
            // Simple circle collision
            let dx = entity.x - start_x;
            let dy = entity.y - start_y;
            let dist_along_ray = dx * dir_x + dy * dir_y;

            if dist_along_ray > 0.0 && dist_along_ray < max_distance {
                let closest_x = start_x + dir_x * dist_along_ray;
                let closest_y = start_y + dir_y * dist_along_ray;
                let dist_to_ray =
                    ((entity.x - closest_x).powi(2) + (entity.y - closest_y).powi(2)).sqrt();

                if dist_to_ray < ENTITY_RADIUS {
                    hits.push((dist_along_ray, Arc::new(entity.clone())));
                }
            }
        }

        hits.into_iter()
            .min_by_key(|h| h.0 as i32)
            .map(|h| h.1)
    }

    /// Raycast and find all entities hit by ray (sorted by distance)
    pub fn raycast_all(
        world: &World,
        start_x: f32,
        start_y: f32,
        dir_x: f32,
        dir_y: f32,
        max_distance: f32,
    ) -> Vec<Arc<Entity>> {
        let mut hits = Vec::new();

        for entity in world.entities.values() {
            let dx = entity.x - start_x;
            let dy = entity.y - start_y;
            let dist_along_ray = dx * dir_x + dy * dir_y;

            if dist_along_ray > 0.0 && dist_along_ray < max_distance {
                let closest_x = start_x + dir_x * dist_along_ray;
                let closest_y = start_y + dir_y * dist_along_ray;
                let dist_to_ray =
                    ((entity.x - closest_x).powi(2) + (entity.y - closest_y).powi(2)).sqrt();

                if dist_to_ray < ENTITY_RADIUS {
                    hits.push((dist_along_ray, Arc::new(entity.clone())));
                }
            }
        }

        hits.sort_by_key(|h| h.0 as i32);
        hits.into_iter().map(|h| h.1).collect()
    }

    /// Count entities within radius
    pub fn count_entities_in_radius(world: &World, x: f32, y: f32, radius: f32) -> usize {
        Self::query_radius(world, x, y, radius).len()
    }

    /// Find entities within rectangular bounds
    pub fn query_aabb(
        world: &World,
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
    ) -> Vec<Arc<Entity>> {
        let mut results = Vec::new();

        // Get affected chunks
        let chunk_min_x = (min_x / CHUNK_SIZE).floor() as i32;
        let chunk_min_y = (min_y / CHUNK_SIZE).floor() as i32;
        let chunk_max_x = (max_x / CHUNK_SIZE).floor() as i32;
        let chunk_max_y = (max_y / CHUNK_SIZE).floor() as i32;

        for dx in chunk_min_x..=chunk_max_x {
            for dy in chunk_min_y..=chunk_max_y {
                let coord = ChunkCoord {
                    x: dx.max(0) as u32,
                    y: dy.max(0) as u32,
                };

                if let Some(chunk = world.chunks.get(&coord) {
                    for entity_id in &chunk.entities {
                        if let Some(entity) = world.entities.get(entity_id) {
                            if entity.x >= min_x
                                && entity.x <= max_x
                                && entity.y >= min_y
                                && entity.y <= max_y
                            {
                                results.push(Arc::new(entity.clone()));
                            }
                        }
                    }
                }
            }
        }

        results
    }

    /// Find entities by type within radius
    pub fn query_by_type(
        world: &World,
        x: f32,
        y: f32,
        radius: f32,
        entity_type: &str,
    ) -> Vec<Arc<Entity>> {
        Self::query_radius(world, x, y, radius)
            .into_iter()
            .filter(|e| e.entity_type == entity_type)
            .collect()
    }

    /// Find entities by faction within radius
    pub fn query_by_faction(
        world: &World,
        x: f32,
        y: f32,
        radius: f32,
        faction_id: &str,
    ) -> Vec<Arc<Entity>> {
        Self::query_radius(world, x, y, radius)
            .into_iter()
            .filter(|e| e.faction.as_deref() == Some(faction_id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use entropic_world_core::EntityId;

    fn create_test_world() -> World {
        let mut world = World::new("Test".to_string(), "game1".to_string(), 2, 2);
        world.initialize_chunks();

        // Add test entities
        let entity1 = Entity {
            id: "entity1".to_string(),
            x: 100.0,
            y: 100.0,
            entity_type: "player".to_string(),
            faction: Some("faction1".to_string()),
            ..Default::default()
        };

        let entity2 = Entity {
            id: "entity2".to_string(),
            x: 150.0,
            y: 100.0,
            entity_type: "npc".to_string(),
            faction: Some("faction1".to_string()),
            ..Default::default()
        };

        let entity3 = Entity {
            id: "entity3".to_string(),
            x: 500.0,
            y: 500.0,
            entity_type: "player".to_string(),
            faction: Some("faction2".to_string()),
            ..Default::default()
        };

        world.entities.insert("entity1".to_string(), entity1);
        world.entities.insert("entity2".to_string(), entity2);
        world.entities.insert("entity3".to_string(), entity3);

        // Add entities to chunks
        if let Some(chunk) = world.chunks.get_mut(&ChunkCoord::new(0, 0)) {
            chunk.add_entity("entity1".to_string());
            chunk.add_entity("entity2".to_string());
        }

        if let Some(chunk) = world.chunks.get_mut(&ChunkCoord::new(1, 1)) {
            chunk.add_entity("entity3".to_string());
        }

        world
    }

    #[test]
    fn test_query_radius() {
        let world = create_test_world();
        let entities = SpatialQueries::query_radius(&world, 100.0, 100.0, 100.0);

        assert!(entities.len() >= 2);
    }

    #[test]
    fn test_nearest_entity() {
        let world = create_test_world();
        let nearest = SpatialQueries::nearest_entity(&world, 100.0, 100.0, 200.0);

        assert!(nearest.is_some());
        assert_eq!(nearest.unwrap().id, "entity1");
    }

    #[test]
    fn test_query_chunk() {
        let world = create_test_world();
        let entities = SpatialQueries::query_chunk(&world, ChunkCoord::new(0, 0));

        assert_eq!(entities.len(), 2);
    }

    #[test]
    fn test_raycast() {
        let world = create_test_world();
        let hit = SpatialQueries::raycast(&world, 0.0, 100.0, 1.0, 0.0, 500.0);

        assert!(hit.is_some());
    }

    #[test]
    fn test_query_radius_sorted() {
        let world = create_test_world();
        let entities = SpatialQueries::query_radius_sorted(&world, 100.0, 100.0, 100.0);

        assert!(entities.len() >= 2);
        // First should be closest to (100, 100)
        let first = &entities[0];
        assert_eq!(first.id, "entity1");
    }

    #[test]
    fn test_query_aabb() {
        let world = create_test_world();
        let entities = SpatialQueries::query_aabb(&world, 0.0, 0.0, 200.0, 200.0);

        assert!(entities.len() >= 2);
    }

    #[test]
    fn test_query_by_type() {
        let world = create_test_world();
        let players = SpatialQueries::query_by_type(&world, 0.0, 0.0, 1000.0, "player");

        assert!(players.len() >= 1);
        assert_eq!(players[0].entity_type, "player");
    }

    #[test]
    fn test_query_by_faction() {
        let world = create_test_world();
        let faction1 = SpatialQueries::query_by_faction(&world, 0.0, 0.0, 1000.0, "faction1");

        assert!(faction1.len() >= 2);
        assert_eq!(faction1[0].faction.as_deref(), Some("faction1"));
    }

    #[test]
    fn test_count_entities_in_radius() {
        let world = create_test_world();
        let count = SpatialQueries::count_entities_in_radius(&world, 100.0, 100.0, 100.0);

        assert!(count >= 2);
    }
}
