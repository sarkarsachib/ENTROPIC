use crate::errors::SpatialError;
use crate::{Chunk, ChunkCoord};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Chunk serialization format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedChunk {
    pub coord: (u32, u32),
    pub biome: String,
    pub elevation: Vec<f32>,
    pub vegetation: Vec<u8>,
    pub water_level: f32,
    pub entities: Vec<String>,
    pub structures: Vec<SerializedStructure>,
}

/// Structure serialization format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedStructure {
    pub id: String,
    pub structure_type: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Chunk persistence manager
pub struct ChunkSerializer;

impl ChunkSerializer {
    /// Serialize chunk to bytes
    pub fn serialize_chunk(chunk: &Chunk) -> Result<Vec<u8>, SpatialError> {
        bincode::serialize(chunk)
            .map_err(|e| SpatialError::SerializationError {
                message: e.to_string(),
            })
    }

    /// Deserialize chunk from bytes
    pub fn deserialize_chunk(data: &[u8]) -> Result<Chunk, SpatialError> {
        bincode::deserialize(data)
            .map_err(|e| SpatialError::DeserializationError {
                message: e.to_string(),
            })
    }

    /// Serialize chunk to JSON
    pub fn serialize_chunk_json(chunk: &Chunk) -> Result<String, SpatialError> {
        let serialized = SerializedChunk::from_chunk(chunk);
        serde_json::to_string_pretty(&serialized)
            .map_err(|e| SpatialError::SerializationError {
                message: e.to_string(),
            })
    }

    /// Deserialize chunk from JSON
    pub fn deserialize_chunk_json(json: &str) -> Result<Chunk, SpatialError> {
        let serialized: SerializedChunk = serde_json::from_str(json)
            .map_err(|e| SpatialError::DeserializationError {
                message: e.to_string(),
            })?;

        serialized.to_chunk()
    }

    /// Save chunk to file (binary)
    pub fn save_chunk<P: AsRef<Path>>(
        chunk: &Chunk,
        path: P,
    ) -> Result<(), SpatialError> {
        let data = Self::serialize_chunk(chunk)?;
        fs::write(path, data)
            .map_err(|e| SpatialError::IoError {
                message: e.to_string(),
            })
    }

    /// Load chunk from file (binary)
    pub fn load_chunk<P: AsRef<Path>>(path: P) -> Result<Chunk, SpatialError> {
        let data = fs::read(path)
            .map_err(|e| SpatialError::IoError {
                message: e.to_string(),
            })?;

        Self::deserialize_chunk(&data)
    }

    /// Save chunk to file (JSON)
    pub fn save_chunk_json<P: AsRef<Path>>(
        chunk: &Chunk,
        path: P,
    ) -> Result<(), SpatialError> {
        let json = Self::serialize_chunk_json(chunk)?;
        fs::write(path, json)
            .map_err(|e| SpatialError::IoError {
                message: e.to_string(),
            })
    }

    /// Load chunk from file (JSON)
    pub fn load_chunk_json<P: AsRef<Path>>(path: P) -> Result<Chunk, SpatialError> {
        let json = fs::read_to_string(path)
            .map_err(|e| SpatialError::IoError {
                message: e.to_string(),
            })?;

        Self::deserialize_chunk_json(&json)
    }

    /// Serialize multiple chunks to bytes
    pub fn serialize_chunks(chunks: &[Chunk]) -> Result<Vec<u8>, SpatialError> {
        bincode::serialize(chunks)
            .map_err(|e| SpatialError::SerializationError {
                message: e.to_string(),
            })
    }

    /// Deserialize multiple chunks from bytes
    pub fn deserialize_chunks(data: &[u8]) -> Result<Vec<Chunk>, SpatialError> {
        bincode::deserialize(data)
            .map_err(|e| SpatialError::DeserializationError {
                message: e.to_string(),
            })
    }

    /// Calculate compressed size of serialized chunk
    pub fn compressed_size(chunk: &Chunk) -> Result<usize, SpatialError> {
        let data = Self::serialize_chunk(chunk)?;
        Ok(data.len())
    }
}

impl SerializedChunk {
    /// Convert from Chunk
    fn from_chunk(chunk: &Chunk) -> Self {
        Self {
            coord: (chunk.coord.x, chunk.coord.y),
            biome: format!("{:?}", chunk.biome),
            elevation: chunk.elevation.clone(),
            vegetation: chunk.vegetation.clone(),
            water_level: chunk.water_level,
            entities: chunk.entities.clone(),
            structures: chunk
                .structures
                .iter()
                .map(|s| SerializedStructure {
                    id: s.id.clone(),
                    structure_type: format!("{:?}", s.structure_type),
                    x: s.x,
                    y: s.y,
                    z: s.z,
                })
                .collect(),
        }
    }

    /// Convert to Chunk
    fn to_chunk(&self) -> Result<Chunk, SpatialError> {
        let mut chunk = Chunk::new(ChunkCoord::new(self.coord.0, self.coord.1));

        chunk.elevation = self.elevation.clone();
        chunk.vegetation = self.vegetation.clone();
        chunk.water_level = self.water_level;
        chunk.entities = self.entities.clone();

        // Note: Biome and structures are not fully deserialized here
        // because we need to parse the string representation

        Ok(chunk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_chunk() -> Chunk {
        let mut chunk = Chunk::new(ChunkCoord::new(5, 10));
        chunk.elevation[100] = 50.5;
        chunk.vegetation[200] = 128;
        chunk.load();
        chunk
    }

    #[test]
    fn test_serialize_deserialize_binary() {
        let chunk = create_test_chunk();
        let data = ChunkSerializer::serialize_chunk(&chunk).unwrap();
        let deserialized = ChunkSerializer::deserialize_chunk(&data).unwrap();

        assert_eq!(chunk.coord, deserialized.coord);
        assert_eq!(chunk.elevation[100], deserialized.elevation[100]);
    }

    #[test]
    fn test_serialize_deserialize_json() {
        let chunk = create_test_chunk();
        let json = ChunkSerializer::serialize_chunk_json(&chunk).unwrap();
        let deserialized = ChunkSerializer::deserialize_chunk_json(&json).unwrap();

        assert_eq!(chunk.coord, deserialized.coord);
        assert_eq!(chunk.elevation[100], deserialized.elevation[100]);
    }

    #[test]
    fn test_compressed_size() {
        let chunk = create_test_chunk();
        let size = ChunkSerializer::compressed_size(&chunk).unwrap();

        assert!(size > 0);
    }

    #[test]
    fn test_serialize_chunks() {
        let chunks = vec![
            Chunk::new(ChunkCoord::new(0, 0)),
            Chunk::new(ChunkCoord::new(1, 1)),
        ];

        let data = ChunkSerializer::serialize_chunks(&chunks).unwrap();
        let deserialized = ChunkSerializer::deserialize_chunks(&data).unwrap();

        assert_eq!(deserialized.len(), 2);
    }
}
