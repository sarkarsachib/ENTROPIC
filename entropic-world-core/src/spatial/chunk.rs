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

    pub fn with_biome(mut self, biome: Biome) -> Self {
        self.biome = biome;
        self
    }

    pub fn load(&mut self) {
        self.loaded = true;
    }

    pub fn unload(&mut self) {
        self.loaded = false;
    }

    pub fn add_entity(&mut self, entity_id: EntityId) {
        if !self.entities.contains(&entity_id) {
            self.entities.push(entity_id);
        }
    }

    pub fn remove_entity(&mut self, entity_id: &EntityId) {
        self.entities.retain(|e| e != entity_id);
    }

    pub fn add_structure(&mut self, structure: Structure) {
        self.structures.push(structure);
    }

    pub fn get_elevation_at(&self, x: usize, y: usize) -> Option<f32> {
        if x < HEIGHTMAP_RESOLUTION && y < HEIGHTMAP_RESOLUTION {
            Some(self.elevation[y * HEIGHTMAP_RESOLUTION + x])
        } else {
            None
        }
    }

    pub fn set_elevation_at(&mut self, x: usize, y: usize, elevation: f32) {
        if x < HEIGHTMAP_RESOLUTION && y < HEIGHTMAP_RESOLUTION {
            self.elevation[y * HEIGHTMAP_RESOLUTION + x] = elevation;
        }
    }

    pub fn get_vegetation_at(&self, x: usize, y: usize) -> Option<u8> {
        if x < HEIGHTMAP_RESOLUTION && y < HEIGHTMAP_RESOLUTION {
            Some(self.vegetation[y * HEIGHTMAP_RESOLUTION + x])
        } else {
            None
        }
    }

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
