use entropic_spatial_engine::{ChunkManager, StreamingEvent};
use entropic_world_core::World;
use std::sync::Arc;

#[tokio::test]
async fn test_chunk_streaming() {
    let world = Arc::new(World::new("Test".to_string(), "game1".to_string(), 10, 10));
    let manager = ChunkManager::new(world.clone(), 2);

    manager.update_visible_chunks(0.0, 0.0).await.unwrap();

    // Should queue chunks around (0, 0)
    assert!(manager.load_queue_size() > 0);

    // Process at least one chunk
    let loaded = manager.process_load_queue().await.unwrap();
    assert!(loaded.is_some());
    assert_eq!(manager.loaded_chunk_count(), 1);
}

#[tokio::test]
async fn test_streaming_events() {
    let world = Arc::new(World::new("Test".to_string(), "game1".to_string(), 5, 5));
    let manager = Arc::new(ChunkManager::new(world, 2));
    let (streamer, mut event_rx) = entropic_spatial_engine::ChunkStreamer::new(manager.clone());

    // Send update
    streamer.update_position(100.0, 100.0).unwrap();

    // Wait for events
    let mut event_count = 0;
    tokio::time::timeout(
        tokio::time::Duration::from_millis(200),
        async {
            while let Some(event) = event_rx.recv().await {
                match event {
                    StreamingEvent::ChunkLoaded { .. } => event_count += 1,
                    StreamingEvent::ChunkUnloaded { .. } => event_count += 1,
                    StreamingEvent::Error { .. } => panic!("Unexpected error"),
                    _ => {}
                }
                if event_count >= 3 {
                    break;
                }
            }
        }
    )
    .await
    .ok();

    assert!(event_count > 0);
}

#[tokio::test]
async fn test_streaming_pause_resume() {
    let world = Arc::new(World::new("Test".to_string(), "game1".to_string(), 5, 5));
    let manager = Arc::new(ChunkManager::new(world, 2));
    let (streamer, mut event_rx) = entropic_spatial_engine::ChunkStreamer::new(manager.clone());

    streamer.pause().unwrap();

    let paused = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        async {
            if let Some(entropic_spatial_engine::StreamingEvent::StreamPaused) = event_rx.recv().await {
                return true;
            }
            false
        }
    )
    .await
    .unwrap_or(false);

    assert!(paused, "Should receive pause event");

    streamer.resume().unwrap();

    let resumed = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        async {
            if let Some(entropic_spatial_engine::StreamingEvent::StreamResumed) = event_rx.recv().await {
                return true;
            }
            false
        }
    )
    .await
    .unwrap_or(false);

    assert!(resumed, "Should receive resume event");
}
