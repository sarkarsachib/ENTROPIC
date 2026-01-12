use serde::{Deserialize, Serialize};
use crate::spatial::coordinates::ChunkCoord;
use crate::spatial::terrain::{Biome, Structure};
use crate::spatial::spatial_index::EntityId;
use crate::temporal::weather::Weather;
use crate::constants::HEIGHTMAP_RESOLUTION;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub biome: Biome,
    pub elevation: Vec<f32>,
    pub vegetation: Vec<u8>,
    pub water_level: f32,
    pub entities: Vec<EntityId>,
    pub structures: Vec<Structure>,
    pub weather: Weather,
    pub loaded: bool,
}

impl Chunk {
    /// Creates a new Chunk for the given coordinates with default biome, a zeroed heightmap and vegetation map, default weather, empty entities and structures, water level 0.0, and marked as not loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// let chunk = Chunk::new(ChunkCoord(5, 10));
    /// assert_eq!(chunk.coord, ChunkCoord(5, 10));
    /// assert_eq!(chunk.biome, Biome::default());
    /// assert_eq!(chunk.loaded, false);
    /// assert_eq!(chunk.elevation.len(), HEIGHTMAP_RESOLUTION * HEIGHTMAP_RESOLUTION);
    /// ```
    pub fn new(coord: ChunkCoord) -> Self {
        Self {
            coord,
            biome: Biome::default(),
            elevation: vec![0.0; HEIGHTMAP_RESOLUTION * HEIGHTMAP_RESOLUTION],
            vegetation: vec![0; HEIGHTMAP_RESOLUTION * HEIGHTMAP_RESOLUTION],
            water_level: 0.0,
            entities: Vec::new(),
            structures: Vec::new(),
            weather: Weather::default(),
            loaded: false,
        }
    }

    /// Set the chunk's biome, consuming the chunk and enabling builder-style chaining.
    ///
    /// # Parameters
    ///
    /// - `biome`: Biome to assign to the chunk.
    ///
    /// # Returns
    ///
    /// `Self` with its `biome` field set to the provided value.
    ///
    /// # Examples
    ///
    /// ```
    /// let coord = ChunkCoord(0, 0);
    /// let chunk = Chunk::new(coord).with_biome(Biome::Plains);
    /// assert_eq!(chunk.biome, Biome::Plains);
    /// ```
    pub fn with_biome(mut self, biome: Biome) -> Self {
        self.biome = biome;
        self
    }

    /// Marks the chunk as loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chunk = Chunk::new(ChunkCoord(0, 0));
    /// assert!(!chunk.loaded);
    /// chunk.load();
    /// assert!(chunk.loaded);
    /// ```
    pub fn load(&mut self) {
        self.loaded = true;
    }

    /// Marks the chunk as not loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chunk = Chunk::new(ChunkCoord(0, 0));
    /// chunk.load();
    /// chunk.unload();
    /// assert!(!chunk.loaded);
    /// ```
    pub fn unload(&mut self) {
        self.loaded = false;
    }

    /// Adds an entity identifier to the chunk, ignoring the insert if the identifier is already present.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chunk = Chunk::new(ChunkCoord(0, 0));
    /// chunk.add_entity("entity1".into());
    /// assert_eq!(chunk.entities.len(), 1);
    /// // duplicate insert is ignored
    /// chunk.add_entity("entity1".into());
    /// assert_eq!(chunk.entities.len(), 1);
    /// ```
    pub fn add_entity(&mut self, entity_id: EntityId) {
        if !self.entities.contains(&entity_id) {
            self.entities.push(entity_id);
        }
    }

    /// Removes all occurrences of the specified entity identifier from the chunk's entity list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chunk = Chunk::new(ChunkCoord(0, 0));
    /// chunk.add_entity("entity1".to_string());
    /// chunk.add_entity("entity1".to_string());
    /// chunk.add_entity("entity2".to_string());
    /// chunk.remove_entity(&"entity1".to_string());
    /// assert!(!chunk.entities.contains(&"entity1".to_string()));
    /// assert!(chunk.entities.contains(&"entity2".to_string()));
    /// ```
    pub fn remove_entity(&mut self, entity_id: &EntityId) {
        self.entities.retain(|e| e != entity_id);
    }

    /// Appends a structure to this chunk's list of structures.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chunk = Chunk::new(ChunkCoord(0, 0));
    /// let structure = Structure::default();
    /// chunk.add_structure(structure);
    /// assert_eq!(chunk.structures.len(), 1);
    /// ```
    pub fn add_structure(&mut self, structure: Structure) {
        self.structures.push(structure);
    }

    /// Retrieve the elevation value at the given heightmap coordinates.
    ///
    /// Coordinates are zero-based and must be less than `HEIGHTMAP_RESOLUTION` in both axes; otherwise no value is returned.
    ///
    /// # Returns
    ///
    /// `Some(elevation)` with the height at `(x, y)` if the coordinates are within bounds, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut c = Chunk::new(ChunkCoord(0, 0));
    /// c.set_elevation_at(10, 20, 50.5);
    /// assert_eq!(c.get_elevation_at(10, 20), Some(50.5));
    /// assert_eq!(c.get_elevation_at(9999, 9999), None);
    /// ```
    pub fn get_elevation_at(&self, x: usize, y: usize) -> Option<f32> {
        if x < HEIGHTMAP_RESOLUTION && y < HEIGHTMAP_RESOLUTION {
            Some(self.elevation[y * HEIGHTMAP_RESOLUTION + x])
        } else {
            None
        }
    }

    /// Sets the heightmap elevation at the given (x, y) coordinates.
    ///
    /// If (x, y) are outside the heightmap resolution, the function does nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chunk = Chunk::new(ChunkCoord(0, 0));
    /// chunk.set_elevation_at(2, 3, 42.0);
    /// assert_eq!(chunk.get_elevation_at(2, 3), Some(42.0));
    /// // out-of-bounds writes are ignored
    /// chunk.set_elevation_at(999, 999, 7.0);
    /// assert_eq!(chunk.get_elevation_at(999, 999), None);
    /// ```
    pub fn set_elevation_at(&mut self, x: usize, y: usize, elevation: f32) {
        if x < HEIGHTMAP_RESOLUTION && y < HEIGHTMAP_RESOLUTION {
            self.elevation[y * HEIGHTMAP_RESOLUTION + x] = elevation;
        }
    }

    /// Gets the vegetation density at the given heightmap coordinates.
    ///
    /// Returns `Some(density)` when `x` and `y` are within the heightmap bounds,
    /// `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chunk = Chunk::new(ChunkCoord(0, 0));
    /// chunk.set_vegetation_at(1, 2, 42);
    /// assert_eq!(chunk.get_vegetation_at(1, 2), Some(42));
    /// ```
    pub fn get_vegetation_at(&self, x: usize, y: usize) -> Option<u8> {
        if x < HEIGHTMAP_RESOLUTION && y < HEIGHTMAP_RESOLUTION {
            Some(self.vegetation[y * HEIGHTMAP_RESOLUTION + x])
        } else {
            None
        }
    }

    /// Sets the vegetation density at the given (x, y) heightmap coordinates if they are within bounds.
    ///
    /// If `x` or `y` is outside the heightmap resolution this function does nothing.
    ///
    /// # Parameters
    ///
    /// - `x`: The x coordinate within the heightmap (0-based).
    /// - `y`: The y coordinate within the heightmap (0-based).
    /// - `density`: Vegetation density value (0–255).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chunk = Chunk::new(ChunkCoord(0, 0));
    /// chunk.set_vegetation_at(2, 3, 128);
    /// assert_eq!(chunk.get_vegetation_at(2, 3), Some(128));
    /// ```
    pub fn set_vegetation_at(&mut self, x: usize, y: usize, density: u8) {
        if x < HEIGHTMAP_RESOLUTION && y < HEIGHTMAP_RESOLUTION {
            self.vegetation[y * HEIGHTMAP_RESOLUTION + x] = density;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_creation() {
        let coord = ChunkCoord::new(5, 10);
        let chunk = Chunk::new(coord);
        assert_eq!(chunk.coord, coord);
        assert_eq!(chunk.biome, Biome::Plains);
        assert!(!chunk.loaded);
    }

    #[test]
    fn test_chunk_entity_management() {
        let mut chunk = Chunk::new(ChunkCoord::new(0, 0));
        chunk.add_entity("entity1".to_string());
        chunk.add_entity("entity2".to_string());
        assert_eq!(chunk.entities.len(), 2);

        chunk.remove_entity(&"entity1".to_string());
        assert_eq!(chunk.entities.len(), 1);
        assert!(!chunk.entities.contains(&"entity1".to_string()));
    }

    #[test]
    fn test_chunk_elevation() {
        let mut chunk = Chunk::new(ChunkCoord::new(0, 0));
        chunk.set_elevation_at(10, 20, 50.5);
        assert_eq!(chunk.get_elevation_at(10, 20), Some(50.5));
    }
}