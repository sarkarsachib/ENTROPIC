use entropic_spatial_engine::TerrainGenerator;
use entropic_world_core::Biome;

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
    let coord = entropic_spatial_engine::ChunkCoord::new(0, 0);
    let chunk = generator.generate_chunk(coord).unwrap();

    assert_eq!(chunk.coord, coord);
    assert!(chunk.loaded);
    assert_eq!(chunk.elevation.len(), 256 * 256);
    assert_eq!(chunk.vegetation.len(), 256 * 256);
}

#[test]
fn test_deterministic_generation() {
    let generator1 = TerrainGenerator::with_seed(42);
    let generator2 = TerrainGenerator::with_seed(42);

    let coord = entropic_spatial_engine::ChunkCoord::new(5, 5);
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

    let coord = entropic_spatial_engine::ChunkCoord::new(0, 0);
    let chunk1 = generator1.generate_chunk(coord).unwrap();
    let chunk2 = generator2.generate_chunk(coord).unwrap();

    let same_biome = chunk1.biome == chunk2.biome;
    let same_elevation = chunk1.elevation == chunk2.elevation;
    assert!(!(same_biome && same_elevation));
}

#[test]
fn test_heightmap_range() {
    let generator = TerrainGenerator::new();
    let chunk = generator.generate_chunk(entropic_spatial_engine::ChunkCoord::new(0, 0)).unwrap();

    for &height in &chunk.elevation {
        assert!(height >= 0.0 && height <= 255.0);
    }
}

#[test]
fn test_vegetation_range() {
    let generator = TerrainGenerator::new();
    let chunk = generator.generate_chunk(entropic_spatial_engine::ChunkCoord::new(0, 0)).unwrap();

    for &veg in &chunk.vegetation {
        assert!(veg <= 255);
    }
}

#[test]
fn test_biome_generation() {
    let generator = TerrainGenerator::with_seed(999);

    // Generate multiple chunks to get different biomes
    let mut biomes = std::collections::HashSet::new();
    for x in 0..5 {
        for y in 0..5 {
            let chunk = generator.generate_chunk(
                entropic_spatial_engine::ChunkCoord::new(x, y)
            ).unwrap();
            biomes.insert(chunk.biome);
        }
    }

    // Should have at least 2 different biomes
    assert!(biomes.len() >= 2);
}

#[test]
fn test_chunk_performance() {
    let generator = TerrainGenerator::new();
    let coord = entropic_spatial_engine::ChunkCoord::new(0, 0);

    let start = std::time::Instant::now();
    let _ = generator.generate_chunk(coord).unwrap();
    let duration = start.elapsed();

    // Should complete in under 50ms
    assert!(duration.as_millis() < 50, "Chunk generation should be < 50ms");
}

#[test]
fn test_multiple_chunks_generation() {
    let generator = TerrainGenerator::new();

    let coords = vec![
        entropic_spatial_engine::ChunkCoord::new(0, 0),
        entropic_spatial_engine::ChunkCoord::new(1, 0),
        entropic_spatial_engine::ChunkCoord::new(0, 1),
    ];

    for coord in &coords {
        let chunk = generator.generate_chunk(*coord).unwrap();
        assert_eq!(chunk.coord, *coord);
        assert!(chunk.loaded);
    }
}
