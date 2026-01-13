//! Voxel system for 3D spatial representation
//!
//! This module is only available when the "voxel" feature is enabled.

use crate::errors::SpatialError;

/// Voxel data structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Voxel {
    Empty,
    Solid { material: u8, density: u8 },
    Fluid { fluid_type: u8, level: u8 },
}

impl Default for Voxel {
    fn default() -> Self {
        Voxel::Empty
    }
}

/// Voxel chunk
#[derive(Debug, Clone)]
pub struct VoxelChunk {
    pub coord: (u32, u32, u32),
    pub voxels: Vec<Voxel>,
}

impl VoxelChunk {
    /// Create a new voxel chunk
    pub fn new(coord: (u32, u32, u32)) -> Self {
        Self {
            coord,
            voxels: vec![Voxel::default(); 16 * 16 * 16], // 16x16x16 voxels
        }
    }

    /// Get voxel at position
    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&Voxel> {
        if x < 16 && y < 16 && z < 16 {
            self.voxels.get(x + y * 16 + z * 16 * 16)
        } else {
            None
        }
    }

    /// Set voxel at position
    pub fn set(&mut self, x: usize, y: usize, z: usize, voxel: Voxel) {
        if x < 16 && y < 16 && z < 16 {
            self.voxels[x + y * 16 + z * 16 * 16] = voxel;
        }
    }

    /// Serialize voxel chunk
    pub fn serialize(&self) -> Result<Vec<u8>, SpatialError> {
        bincode::serialize(self)
            .map_err(|e| SpatialError::SerializationError {
                message: e.to_string(),
            })
    }

    /// Deserialize voxel chunk
    pub fn deserialize(data: &[u8]) -> Result<Self, SpatialError> {
        bincode::deserialize(data)
            .map_err(|e| SpatialError::DeserializationError {
                message: e.to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voxel_chunk_creation() {
        let chunk = VoxelChunk::new((0, 0, 0));
        assert_eq!(chunk.coord, (0, 0, 0));
        assert_eq!(chunk.voxels.len(), 16 * 16 * 16);
    }

    #[test]
    fn test_voxel_set_get() {
        let mut chunk = VoxelChunk::new((0, 0, 0));
        let voxel = Voxel::Solid {
            material: 1,
            density: 255,
        };

        chunk.set(5, 5, 5, voxel);
        let retrieved = chunk.get(5, 5, 5);

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), &voxel);
    }

    #[test]
    fn test_voxel_out_of_bounds() {
        let chunk = VoxelChunk::new((0, 0, 0));
        assert!(chunk.get(20, 5, 5).is_none());
    }
}
