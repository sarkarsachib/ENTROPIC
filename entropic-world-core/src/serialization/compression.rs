use crate::spatial::chunk::Chunk;

pub fn compress_chunk_elevation(chunk: &Chunk) -> Vec<u8> {
    let mut result = Vec::new();
    
    for &elevation in &chunk.elevation {
        let bytes = elevation.to_le_bytes();
        result.extend_from_slice(&bytes);
    }
    
    result
}

pub fn decompress_chunk_elevation(bytes: &[u8]) -> Vec<f32> {
    let mut result = Vec::new();
    
    for chunk in bytes.chunks(4) {
        if chunk.len() == 4 {
            let mut array = [0u8; 4];
            array.copy_from_slice(chunk);
            result.push(f32::from_le_bytes(array));
        }
    }
    
    result
}

pub fn compress_chunk_vegetation(chunk: &Chunk) -> Vec<u8> {
    chunk.vegetation.clone()
}

pub fn decompress_chunk_vegetation(bytes: &[u8]) -> Vec<u8> {
    bytes.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spatial::coordinates::ChunkCoord;

    #[test]
    fn test_elevation_compression() {
        let chunk = Chunk::new(ChunkCoord::new(0, 0));
        let compressed = compress_chunk_elevation(&chunk);
        let decompressed = decompress_chunk_elevation(&compressed);

        assert_eq!(chunk.elevation.len(), decompressed.len());
        for (original, decompressed_val) in chunk.elevation.iter().zip(decompressed.iter()) {
            assert_eq!(original, decompressed_val);
        }
    }

    #[test]
    fn test_vegetation_compression() {
        let chunk = Chunk::new(ChunkCoord::new(0, 0));
        let compressed = compress_chunk_vegetation(&chunk);
        let decompressed = decompress_chunk_vegetation(&compressed);

        assert_eq!(chunk.vegetation, decompressed);
    }
}
