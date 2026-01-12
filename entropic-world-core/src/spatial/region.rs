use serde::{Deserialize, Serialize};
use crate::spatial::coordinates::ChunkCoord;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegionCoord {
    pub x: u32,
    pub y: u32,
}

impl RegionCoord {
    /// Constructs a `RegionCoord` from the given x and y coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let coord = crate::spatial::region::RegionCoord::new(1, 2);
    /// assert_eq!(coord.x, 1);
    /// assert_eq!(coord.y, 2);
    /// ```
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Computes the region coordinate that contains the given chunk.
    ///
    /// The returned `RegionCoord`'s `x` and `y` are the integer quotients of the chunk's
    /// `x` and `y` divided by `region_size`.
    ///
    /// # Examples
    ///
    /// ```
    /// let chunk = ChunkCoord { x: 15, y: 20 };
    /// let region = RegionCoord::from_chunk_coord(&chunk, 8);
    /// assert_eq!(region.x, 1);
    /// assert_eq!(region.y, 2);
    /// ```
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
    /// Creates a Region for the given `RegionCoord` with an empty chunk list and no name or description.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::spatial::region::{Region, RegionCoord};
    ///
    /// let coord = RegionCoord::new(0, 0);
    /// let region = Region::new(coord);
    /// assert!(region.chunks.is_empty());
    /// assert!(region.name.is_none());
    /// assert!(region.description.is_none());
    /// ```
    pub fn new(coord: RegionCoord) -> Self {
        Self {
            coord,
            chunks: Vec::new(),
            name: None,
            description: None,
        }
    }

    /// Sets the region's name and returns the modified `Region` for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// let region = Region::new(RegionCoord::new(0, 0)).with_name("Spawn".to_string());
    /// assert_eq!(region.name.as_deref(), Some("Spawn"));
    /// ```
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Adds the given chunk to the region if it is not already present.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut region = Region::new(RegionCoord::new(0, 0));
    /// let chunk = ChunkCoord::new(1, 2);
    /// region.add_chunk(chunk.clone());
    /// region.add_chunk(chunk);
    /// assert_eq!(region.chunks.len(), 1);
    /// ```
    pub fn add_chunk(&mut self, chunk: ChunkCoord) {
        if !self.chunks.contains(&chunk) {
            self.chunks.push(chunk);
        }
    }

    /// Checks whether the region contains the given chunk coordinate.
    ///
    /// # Returns
    ///
    /// `true` if the chunk is present in the region's chunk list, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::spatial::region::{Region, RegionCoord};
    /// use crate::spatial::coordinates::ChunkCoord;
    ///
    /// let mut region = Region::new(RegionCoord::new(0, 0));
    /// let chunk = ChunkCoord::new(1, 1);
    /// region.add_chunk(chunk.clone());
    /// assert!(region.contains_chunk(&chunk));
    /// ```
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