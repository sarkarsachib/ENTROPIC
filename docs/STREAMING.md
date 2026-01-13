# STREAMING.md

## Chunk Streaming System

The spatial engine provides an async chunk streaming system for efficient world loading.

### Architecture

```
ChunkStreamer (async task)
    ↓ commands
ChunkManager (state)
    ↓ generates
TerrainGenerator (deterministic)
    ↓ loads
Loaded Chunks (cache)
    ↓ unloads
Unloaded Chunks
```

### Priority System

Chunks are loaded based on priority:
- **Critical** (3): Player is in/near chunk
- **High** (2): Visible from player
- **Normal** (1): Adjacent to visible
- **Low** (0): Distant

### Usage

```rust
use entropic_spatial_engine::{ChunkManager, ChunkStreamer, StreamingCommand};

// Create chunk manager
let world = Arc::new(World::new(...));
let manager = Arc::new(ChunkManager::new(world, 5));

// Create streamer
let (streamer, mut event_rx) = ChunkStreamer::new(manager.clone());

// Update player position (triggers chunk loading)
streamer.update_position(player_x, player_y)?;

// Listen for events
while let Some(event) = event_rx.recv().await {
    match event {
        StreamingEvent::ChunkLoaded { coord } => {
            println!("Chunk loaded: {:?}", coord);
        }
        StreamingEvent::ChunkUnloaded { coord } => {
            println!("Chunk unloaded: {:?}", coord);
        }
        _ => {}
    }
}
```

### Performance Considerations

1. **Chunk Size**: Default 256x256 meters per chunk
2. **View Distance**: Adjust based on performance needs (default: 5 chunks)
3. **Max Loaded Chunks**: Configurable limit (default: 10,000 chunks)
4. **Load Interval**: Process queue every 50ms

### Best Practices

- Preload critical areas using `preload_chunks()`
- Adjust view distance based on player movement speed
- Pause streaming during loading screens
- Use `unload_all()` when changing scenes

### Events

**ChunkLoaded**: A chunk was successfully loaded
**ChunkUnloaded**: A chunk was unloaded from memory
**StreamPaused**: Streaming is paused
**StreamResumed**: Streaming resumed
**Error**: An error occurred during streaming
