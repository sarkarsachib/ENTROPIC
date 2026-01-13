use entropic_spatial_engine::{ChunkManager, TerrainGenerator};
use entropic_world_core::World;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    println!("=== World Generation Demo ===\n");

    // Create a new world
    let world = Arc::new(World::new(
        "Generated World".to_string(),
        "game_dna_1".to_string(),
        5,
        5,
    ));
    world.initialize_chunks();

    println!("Created world with {}x{} chunks", world.width_chunks, world.height_chunks);

    // Create chunk manager with view distance of 3 chunks
    let manager = ChunkManager::new(world.clone(), 3);

    // Generate chunks around origin
    println!("\nGenerating chunks around (100.0, 100.0)...");
    manager.update_visible_chunks(100.0, 100.0).await.unwrap();

    println!("Load queue size: {}", manager.load_queue_size());

    // Process load queue
    let mut loaded_count = 0;
    while let Some(coord) = manager.process_load_queue().await.unwrap() {
        loaded_count += 1;
        println!("Loaded chunk: ({}, {})", coord.x, coord.y);

        // Get chunk details
        if let Some(chunk) = manager.get_chunk(coord) {
            println!("  - Biome: {:?}", chunk.biome);
            println!("  - Loaded: {}", chunk.loaded);
            println!("  - Water level: {}", chunk.water_level);

            // Sample some height values
            let height_samples: Vec<_> = chunk.elevation.iter().take(5).collect();
            println!("  - Sample heights: {:?}", height_samples);
        }

        if loaded_count >= 5 {
            println!("Stopping after loading 5 chunks (demo)...");
            break;
        }
    }

    println!("\nTotal chunks loaded: {}", manager.loaded_chunk_count());

    // Demonstrate terrain generator
    println!("\n=== Terrain Generator Demo ===");
    let generator = TerrainGenerator::with_seed(42);
    let coord = entropic_spatial_engine::ChunkCoord::new(10, 10);
    let chunk = generator.generate_chunk(coord).unwrap();

    println!("Generated chunk at ({}, {})", coord.x, coord.y);
    println!("Biome: {:?}", chunk.biome);
    println!("Elevation range: {:.2} - {:.2}",
        chunk.elevation.iter().cloned().fold(f32::INFINITY, f32::min),
        chunk.elevation.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
    );
    println!("Vegetation range: {} - {}",
        chunk.vegetation.iter().cloned().min().unwrap_or(0),
        chunk.vegetation.iter().cloned().max().unwrap_or(255)
    );

    // Test determinism
    println!("\n=== Determinism Test ===");
    let chunk2 = generator.generate_chunk(coord).unwrap();
    assert_eq!(chunk.biome, chunk2.biome, "Biomes should match");
    assert_eq!(chunk.elevation, chunk2.elevation, "Elevation should match");
    println!("✓ Deterministic generation confirmed (same seed = same terrain)");
}
