use serde::{Deserialize, Serialize};
use crate::spatial::coordinates::ChunkCoord;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegionCoord {
    pub x: u32,
    pub y: u32,
}

impl RegionCoord {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn from_chunk_coord(chunk: &ChunkCoord, region_size: u32) -> Self {
        Self {
            x: chunk.x / region_size,
            y: chunk.y / region_size,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Region {
    pub coord: RegionCoord,
    pub chunks: Vec<ChunkCoord>,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Region {
    pub fn new(coord: RegionCoord) -> Self {
        Self {
            coord,
            chunks: Vec::new(),
            name: None,
            description: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn add_chunk(&mut self, chunk: ChunkCoord) {
        if !self.chunks.contains(&chunk) {
            self.chunks.push(chunk);
        }
    }

    pub fn contains_chunk(&self, chunk: &ChunkCoord) -> bool {
        self.chunks.contains(chunk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_from_chunk() {
        let chunk = ChunkCoord::new(15, 20);
        let region = RegionCoord::from_chunk_coord(&chunk, 8);
        assert_eq!(region.x, 1);
        assert_eq!(region.y, 2);
    }

    #[test]
    fn test_region_chunk_management() {
        let mut region = Region::new(RegionCoord::new(0, 0));
        let chunk1 = ChunkCoord::new(0, 0);
        let chunk2 = ChunkCoord::new(1, 1);

        region.add_chunk(chunk1);
        region.add_chunk(chunk2);

        assert_eq!(region.chunks.len(), 2);
        assert!(region.contains_chunk(&chunk1));
    }
}
