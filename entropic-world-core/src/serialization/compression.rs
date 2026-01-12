use crate::spatial::chunk::Chunk;

/// Serializes a chunk's elevation values into a contiguous little-endian byte stream.
///
/// Each f32 elevation in `chunk.elevation` is converted to its 4-byte little-endian
/// representation and appended to the returned vector.
///
/// # Returns
///
/// A `Vec<u8>` containing the concatenated little-endian bytes of all elevation values.
///
/// # Examples
///
/// ```no_run
/// // `chunk` is an instance of `Chunk` with populated `elevation: Vec<f32>`.
/// let bytes = compress_chunk_elevation(&chunk);
/// // Each elevation occupies 4 bytes
/// assert_eq!(bytes.len() % 4, 0);
/// ```
pub fn compress_chunk_elevation(chunk: &Chunk) -> Vec<u8> {
    let mut result = Vec::new();
    
    for &elevation in &chunk.elevation {
        let bytes = elevation.to_le_bytes();
        result.extend_from_slice(&bytes);
    }
    
    result
}

/// Deserializes a byte slice into elevation values by interpreting each consecutive 4 bytes as a little-endian IEEE-754 `f32`.
///
/// Trailing bytes fewer than 4 are ignored; each complete 4-byte group is converted to one `f32`.
///
/// # Examples
///
/// ```
/// let values = [1.0f32, -2.5];
/// let mut bytes = Vec::new();
/// for v in &values {
///     bytes.extend_from_slice(&v.to_le_bytes());
/// }
///
/// let decoded = decompress_chunk_elevation(&bytes);
/// assert_eq!(decoded, values.to_vec());
/// ```
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

/// Obtain the raw vegetation byte sequence for a chunk.
///
/// This returns a copy of the chunk's `vegetation` data as a `Vec<u8>`.
///
/// # Examples
///
/// ```
/// let chunk = Chunk::new(ChunkCoord::new(0, 0));
/// let veg_bytes = compress_chunk_vegetation(&chunk);
/// assert_eq!(veg_bytes, chunk.vegetation);
/// ```
pub fn
pub fn compress_chunk_vegetation(chunk: &Chunk) -> Vec<u8> {
    chunk.vegetation.clone()
}

/// Copies a vegetation byte slice into an owned `Vec<u8>`.
///
/// This creates a new vector containing the same bytes as the provided slice,
/// suitable for storing or returning chunk vegetation data.
///
/// # Examples
///
/// ```
/// let src = &[1u8, 2, 3];
/// let v = decompress_chunk_vegetation(src);
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
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