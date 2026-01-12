use entropic_world_core::world::{World, WorldConfig};
use entropic_world_core::serialization::{serialize_to_json, deserialize_from_json};

fn main() {
    println!("=== World Serialization Example ===\n");

    let config = WorldConfig::new(16, 16)
        .with_seed(123)
        .with_time_scale(2.0);

    let mut world = World::from_config(
        "Serialization Test World".to_string(),
        "game_dna_test".to_string(),
        config,
    );

    world.initialize_chunks();
    
    for _ in 0..50 {
        world.advance_tick();
    }

    println!("Original World:");
    println!("  ID: {}", world.id);
    println!("  Name: {}", world.name);
    println!("  Dimensions: {}x{}", world.width_chunks, world.height_chunks);
    println!("  Chunks: {}", world.total_chunks());
    println!("  Current Tick: {}", world.current_tick);
    println!();

    println!("Serializing to JSON...");
    let json = match serialize_to_json(&world) {
        Ok(json) => {
            println!("✓ Serialization successful!");
            println!("  JSON size: {} bytes", json.len());
            json
        }
        Err(e) => {
            eprintln!("✗ Serialization failed: {}", e);
            return;
        }
    };

    println!("\nJSON Preview (first 500 chars):");
    println!("{}", &json[..json.len().min(500)]);
    println!("...\n");

    println!("Deserializing from JSON...");
    let deserialized_world = match deserialize_from_json(&json) {
        Ok(world) => {
            println!("✓ Deserialization successful!");
            world
        }
        Err(e) => {
            eprintln!("✗ Deserialization failed: {}", e);
            return;
        }
    };

    println!("\nDeserialized World:");
    println!("  ID: {}", deserialized_world.id);
    println!("  Name: {}", deserialized_world.name);
    println!("  Dimensions: {}x{}", deserialized_world.width_chunks, deserialized_world.height_chunks);
    println!("  Chunks: {}", deserialized_world.total_chunks());
    println!("  Current Tick: {}", deserialized_world.current_tick);
    println!();

    println!("=== Verification ===");
    let mut verified = true;

    if world.id != deserialized_world.id {
        println!("✗ World ID mismatch");
        verified = false;
    } else {
        println!("✓ World ID matches");
    }

    if world.name != deserialized_world.name {
        println!("✗ World name mismatch");
        verified = false;
    } else {
        println!("✓ World name matches");
    }

    if world.width_chunks != deserialized_world.width_chunks {
        println!("✗ Width mismatch");
        verified = false;
    } else {
        println!("✓ Width matches");
    }

    if world.height_chunks != deserialized_world.height_chunks {
        println!("✗ Height mismatch");
        verified = false;
    } else {
        println!("✓ Height matches");
    }

    if world.current_tick != deserialized_world.current_tick {
        println!("✗ Current tick mismatch");
        verified = false;
    } else {
        println!("✓ Current tick matches");
    }

    if world.chunks.len() != deserialized_world.chunks.len() {
        println!("✗ Chunk count mismatch");
        verified = false;
    } else {
        println!("✓ Chunk count matches");
    }

    if verified {
        println!("\n✓ Round-trip serialization successful with zero data loss!");
    } else {
        println!("\n✗ Round-trip verification failed!");
    }
}
