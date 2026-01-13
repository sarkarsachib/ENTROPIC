# Entropic Spatial Engine

High-performance spatial engine that manages world chunks, handles streaming/loading/unloading, provides efficient spatial queries, and supports deterministic terrain generation.

## Features

- **Chunk Manager**: Asynchronous chunk loading/unloading with priority queues
- **Terrain Generator**: Deterministic procedural generation using Perlin noise
- **Spatial Queries**: Fast radius, nearest entity, and raycast queries
- **Pathfinding**: A* algorithm with terrain awareness
- **Collision Detection**: Circle and terrain-based collision
- **LOD System**: Progressive detail levels for distant terrain
- **Serialization**: Binary and JSON chunk serialization
- **Streaming**: Async chunk streaming protocol

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
entropic-spatial-engine = { path = "../entropic-spatial-engine" }
```

## Quick Start

```rust,no_run
use entropic_spatial_engine::{ChunkManager, TerrainGenerator};
use entropic_world_core::World;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    let world = Arc::new(World::new(
        "My World".to_string(),
        "game_dna_id".to_string(),
        100,
        100,
    ));
    world.initialize_chunks();

    let manager = ChunkManager::new(world, 5);

    // Update visible chunks based on player position
    manager.update_visible_chunks(1000.0, 1000.0).await.unwrap();

    // Process load queue
    while let Some(coord) = manager.process_load_queue().await.unwrap() {
        println!("Loaded chunk: {:?}", coord);
    }
}
```

## Examples

- **generate_world**: Demonstrates terrain generation
- **stream_chunks**: Shows chunk streaming with player movement
- **spatial_queries**: Demonstrates spatial query operations
- **pathfinding_demo**: Shows A* pathfinding

Run examples with:
```bash
cargo run --example generate_world
cargo run --example stream_chunks
cargo run --example spatial_queries
cargo run --example pathfinding_demo
```

## Benchmarks

Run performance benchmarks:
```bash
cargo bench
```

Performance targets:
- Chunk generation: < 50ms
- Spatial queries: < 10ms (100m radius, 1000 entities)
- Pathfinding: < 5ms (A* with 1000 iterations)

## Testing

Run tests:
```bash
cargo test
```

## Documentation

Full API documentation is available at:
```bash
cargo doc --open
```

## License

MIT OR Apache-2.0
