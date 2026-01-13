use crate::constants::*;
use crate::errors::SpatialError;
use crate::noise::PerlinNoise;
use crate::{Biome, Chunk, ChunkCoord};
use entropic_world_core::constants::HEIGHTMAP_RESOLUTION;
use std::sync::Arc;

/// Procedurally generates terrain using noise functions
pub struct TerrainGenerator {
    perlin: Arc<PerlinNoise>,
    seed: u32,
}

impl TerrainGenerator {
    /// Create a new terrain generator with default seed
    pub fn new() -> Self {
        Self::with_seed(12345)
    }

    /// Create a new terrain generator with a specific seed
    pub fn with_seed(seed: u32) -> Self {
        Self {
            perlin: Arc::new(PerlinNoise::with_seed(seed)),
            seed,
        }
    }

    /// Get the seed used for generation
    pub fn seed(&self) -> u32 {
        self.seed
    }

    /// Generate complete chunk with terrain
    pub fn generate_chunk(&self, coord: ChunkCoord) -> Result<Chunk, SpatialError> {
        let mut chunk = Chunk::new(coord);

        // Generate heightmap
        let heightmap = self.generate_heightmap(coord)?;
        chunk.elevation = heightmap;

        // Generate biome
        chunk.biome = self.determine_biome(coord)?;

        // Generate vegetation
        let vegetation = self.generate_vegetation(coord)?;
        chunk.vegetation = vegetation;

        // Set water level based on biome
        chunk.water_level = match chunk.biome {
            Biome::Ocean => 128.0,
            Biome::Swamp => 100.0,
            _ => 50.0,
        };

        // Mark as loaded
        chunk.load();

        Ok(chunk)
    }

    /// Generate heightmap for a chunk
    fn generate_heightmap(&self, coord: ChunkCoord) -> Result<Vec<f32>, SpatialError> {
        let mut heights = vec![0.0; HEIGHTMAP_RESOLUTION * HEIGHTMAP_RESOLUTION];

        let chunk_x = coord.x as f64 * CHUNK_SIZE as f64;
        let chunk_y = coord.y as f64 * CHUNK_SIZE as f64;

        for i in 0..HEIGHTMAP_RESOLUTION {
            for j in 0..HEIGHTMAP_RESOLUTION {
                let world_x = chunk_x
                    + (i as f64 * CHUNK_SIZE as f64 / HEIGHTMAP_RESOLUTION as f64);
                let world_y = chunk_y
                    + (j as f64 * CHUNK_SIZE as f64 / HEIGHTMAP_RESOLUTION as f64);

                // Multi-octave Perlin noise for natural terrain
                let height = self.perlin.fbm(
                    world_x * TERRAIN_NOISE_FREQUENCY,
                    world_y * TERRAIN_NOISE_FREQUENCY,
                    TERRAIN_NOISE_OCTAVES,
                    0.5,
                    2.0,
                );

                // Normalize from [-1, 1] to [0, 255]
                let normalized = (height + 1.0) / 2.0;
                heights[i * HEIGHTMAP_RESOLUTION + j] = normalized * 255.0;
            }
        }

        Ok(heights)
    }

    /// Determine biome for a chunk based on temperature and moisture
    fn determine_biome(&self, coord: ChunkCoord) -> Result<Biome, SpatialError> {
        let chunk_x = coord.x as f64;
        let chunk_y = coord.y as f64;

        let temp = self.perlin.get(
            chunk_x * BIOME_NOISE_FREQUENCY,
            chunk_y * BIOME_NOISE_FREQUENCY,
        );

        let moisture = self.perlin.get(
            chunk_x * BIOME_NOISE_FREQUENCY + 1000.0,
            chunk_y * BIOME_NOISE_FREQUENCY + 1000.0,
        );

        let biome = match (temp, moisture) {
            (t, _) if t < -0.3 => Biome::Tundra,
            (t, m) if t < 0.0 && m > 0.5 => Biome::Swamp,
            (t, m) if t < 0.0 => Biome::Forest,
            (_, m) if m > 0.6 => Biome::Swamp,
            (_, m) if m > 0.4 => Biome::Grassland,
            (_, m) if m > 0.2 => Biome::Plains,
            _ => Biome::Desert,
        };

        Ok(biome)
    }

    /// Generate vegetation density map for a chunk
    fn generate_vegetation(&self, coord: ChunkCoord) -> Result<Vec<u8>, SpatialError> {
        let mut vegetation = vec![0u8; HEIGHTMAP_RESOLUTION * HEIGHTMAP_RESOLUTION];

        for i in 0..HEIGHTMAP_RESOLUTION {
            for j in 0..HEIGHTMAP_RESOLUTION {
                let val = self.perlin.get(
                    coord.x as f64 + i as f64 * 0.1,
                    coord.y as f64 + j as f64 * 0.1,
                );

                // Normalize to 0-255
                vegetation[i * HEIGHTMAP_RESOLUTION + j] =
                    ((val + 1.0) / 2.0 * 255.0) as u8;
            }
        }

        Ok(vegetation)
    }
}

impl Default for TerrainGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_generator_creation() {
        let generator = TerrainGenerator::new();
        assert_eq!(generator.seed(), 12345);
    }

    #[test]
    fn test_terrain_generator_with_seed() {
        let generator = TerrainGenerator::with_seed(42);
        assert_eq!(generator.seed(), 42);
    }

    #[test]
    fn test_chunk_generation() {
        let generator = TerrainGenerator::new();
        let coord = ChunkCoord::new(0, 0);
        let chunk = generator.generate_chunk(coord).unwrap();

        assert_eq!(chunk.coord, coord);
        assert!(chunk.loaded);
        assert_eq!(chunk.elevation.len(), HEIGHTMAP_RESOLUTION * HEIGHTMAP_RESOLUTION);
        assert_eq!(chunk.vegetation.len(), HEIGHTMAP_RESOLUTION * HEIGHTMAP_RESOLUTION);
    }

    #[test]
    fn test_deterministic_generation() {
        let generator1 = TerrainGenerator::with_seed(42);
        let generator2 = TerrainGenerator::with_seed(42);

        let coord = ChunkCoord::new(5, 5);
        let chunk1 = generator1.generate_chunk(coord).unwrap();
        let chunk2 = generator2.generate_chunk(coord).unwrap();

        assert_eq!(chunk1.biome, chunk2.biome);
        assert_eq!(chunk1.elevation, chunk2.elevation);
        assert_eq!(chunk1.vegetation, chunk2.vegetation);
    }

    #[test]
    fn test_different_seeds_produce_different_chunks() {
        let generator1 = TerrainGenerator::with_seed(1);
        let generator2 = TerrainGenerator::with_seed(2);

        let coord = ChunkCoord::new(0, 0);
        let chunk1 = generator1.generate_chunk(coord).unwrap();
        let chunk2 = generator2.generate_chunk(coord).unwrap();

        // At least one difference should exist
        let same_biome = chunk1.biome == chunk2.biome;
        let same_elevation = chunk1.elevation == chunk2.elevation;
        assert!(!(same_biome && same_elevation));
    }

    #[test]
    fn test_heightmap_range() {
        let generator = TerrainGenerator::new();
        let chunk = generator.generate_chunk(ChunkCoord::new(0, 0)).unwrap();

        for &height in &chunk.elevation {
            assert!(height >= 0.0 && height <= 255.0);
        }
    }

    #[test]
    fn test_vegetation_range() {
        let generator = TerrainGenerator::new();
        let chunk = generator.generate_chunk(ChunkCoord::new(0, 0)).unwrap();

        for &veg in &chunk.vegetation {
            assert!(veg <= 255);
        }
    }
}
