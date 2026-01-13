use entropic_spatial_engine::{ChunkManager, ChunkStreamer, StreamingEvent};
use entropic_world_core::World;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    println!("=== Chunk Streaming Demo ===\n");

    // Create world and chunk manager
    let world = Arc::new(World::new(
        "Streaming World".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
    ));
    world.initialize_chunks();

    let manager = Arc::new(ChunkManager::new(world, 4));

    // Create streamer
    let (streamer, mut event_rx) = ChunkStreamer::new(manager.clone());

    println!("Streaming chunks for player moving through world...");

    // Simulate player movement
    let positions = vec![
        (100.0, 100.0),
        (300.0, 100.0),
        (300.0, 300.0),
        (100.0, 300.0),
        (100.0, 100.0),
    ];

    for (i, (x, y)) in positions.iter().enumerate() {
        println!("\nStep {}: Player at ({}, {})", i + 1, x, y);

        // Update player position
        streamer.update_position(*x, *y).unwrap();

        // Process events
        let mut step_events = 0;
        tokio::time::timeout(
            tokio::time::Duration::from_millis(100),
            async {
                while let Some(event) = event_rx.recv().await {
                    step_events += 1;
                    match event {
                        StreamingEvent::ChunkLoaded { coord } => {
                            println!("  ✓ Chunk loaded: ({}, {})", coord.x, coord.y);
                        }
                        StreamingEvent::ChunkUnloaded { coord } => {
                            println!("  ✗ Chunk unloaded: ({}, {})", coord.x, coord.y);
                        }
                        StreamingEvent::Error { message } => {
                            println!("  ✗ Error: {}", message);
                        }
                        _ => {}
                    }

                    if step_events >= 5 {
                        break;
                    }
                }
            }
        )
        .await
        .ok();

        // Wait a bit to simulate game loop
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    println!("\n=== Final State ===");
    println!("Loaded chunks: {}", manager.loaded_chunk_count());
    println!("Remaining in queue: {}", manager.load_queue_size());

    // Unload all
    streamer.pause().unwrap();
    manager.unload_all();
    println!("\nUnloaded all chunks");
}
