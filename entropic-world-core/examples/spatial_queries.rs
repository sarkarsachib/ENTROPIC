use entropic_world_core::world::{World, WorldConfig};
use entropic_world_core::population::{Entity, EntityType};
use entropic_world_core::spatial::coordinates::ChunkCoord;
use std::time::Instant;

/// Runs a spatial-query performance benchmark.
///
/// Creates a World configured with 64×64 chunks, initializes chunks, populates 1000 NPC entities distributed across the world, and performs several spatial queries (radius, rectangle, and repeated radius queries) while printing timing and result counts. Also demonstrates entity lookup and neighbor discovery and prints a summary of entities and chunks.
///
/// # Examples
///
/// ```no_run
/// // Execute the example benchmark (prints timing and counts to stdout)
/// main();
/// ```
fn main() {
    println!("=== Spatial Query Performance Test ===\n");

    let config = WorldConfig::new(64, 64);
    let mut world = World::from_config(
        "Spatial Test World".to_string(),
        "game_dna_spatial".to_string(),
        config,
    );

    world.initialize_chunks();
    println!("✓ Initialized {} chunks\n", world.total_chunks());

    println!("Populating world with entities...");
    let start = Instant::now();
    
    for i in 0..1000 {
        let x = (i % 64) as f32 * 100.0;
        let y = (i / 64) as f32 * 100.0;
        let chunk_x = (x / 256.0) as u32 % world.width_chunks;
        let chunk_y = (y / 256.0) as u32 % world.height_chunks;
        
        let entity = Entity::new(
            format!("entity_{}", i),
            EntityType::NPC,
            x,
            y,
            0.0,
            ChunkCoord::new(chunk_x, chunk_y),
        );
        
        world.add_entity(entity);
    }
    
    let duration = start.elapsed();
    println!("✓ Added {} entities in {:?}\n", world.total_entities(), duration);

    println!("=== Spatial Query Tests ===\n");

    let query_center = (3200.0, 3200.0);
    let query_radius = 500.0;

    println!("Query Parameters:");
    println!("  Center: ({}, {})", query_center.0, query_center.1);
    println!("  Radius: {} meters\n", query_radius);

    println!("Executing radius query...");
    let start = Instant::now();
    let results = world.spatial_index.query_radius(
        query_center.0,
        query_center.1,
        query_radius,
    );
    let duration = start.elapsed();

    println!("✓ Query completed in {:?}", duration);
    println!("  Found {} entities within radius\n", results.len());

    if duration.as_millis() < 10 {
        println!("✓ Performance target met: < 10ms");
    } else {
        println!("✗ Performance target not met: {:?} >= 10ms", duration);
    }

    println!("\n=== Rectangle Query Test ===\n");

    let min_x = 2000.0;
    let min_y = 2000.0;
    let max_x = 4000.0;
    let max_y = 4000.0;

    println!("Query Parameters:");
    println!("  Rectangle: ({}, {}) to ({}, {})\n", min_x, min_y, max_x, max_y);

    println!("Executing rectangle query...");
    let start = Instant::now();
    let results = world.spatial_index.query_rect(min_x, min_y, max_x, max_y);
    let duration = start.elapsed();

    println!("✓ Query completed in {:?}", duration);
    println!("  Found {} entities in rectangle\n", results.len());

    println!("=== Multiple Query Performance ===\n");

    println!("Executing 100 radius queries...");
    let start = Instant::now();
    
    for i in 0..100 {
        let x = 1000.0 + (i as f32 * 50.0);
        let y = 1000.0 + (i as f32 * 50.0);
        let _ = world.spatial_index.query_radius(x, y, 300.0);
    }
    
    let duration = start.elapsed();
    println!("✓ Completed 100 queries in {:?}", duration);
    println!("  Average per query: {:?}\n", duration / 100);

    println!("=== Entity Lookup Test ===\n");

    if let Some(entity) = world.entities.get("entity_500") {
        println!("Entity 'entity_500':");
        println!("  Position: ({}, {})", entity.x, entity.y);
        println!("  Chunk: ({}, {})", entity.chunk.x, entity.chunk.y);
        println!("  Health: {}", entity.health);
        println!("  Alive: {}", entity.is_alive);
        
        println!("\n  Finding neighbors within 200m...");
        let start = Instant::now();
        let neighbors = world.spatial_index.query_radius(entity.x, entity.y, 200.0);
        let duration = start.elapsed();
        println!("  ✓ Found {} neighbors in {:?}", neighbors.len(), duration);
    }

    println!("\n=== Summary ===");
    println!("Total Entities: {}", world.total_entities());
    println!("Total Chunks: {}", world.total_chunks());
    println!("Spatial Index Grid Size: {} meters", 16.0);
    println!("\n✓ Spatial query system operational!");
}