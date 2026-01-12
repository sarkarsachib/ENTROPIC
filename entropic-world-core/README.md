# Entropic World Core

Foundational data models for ENTROPIC's world simulation system. This library provides the canonical schema for representing all persistent world state—spatial, temporal, population, economy, and ecosystem data.

## Features

- **Spatial System**: Chunk-based world representation with spatial indexing for efficient entity queries
- **Temporal System**: In-game time tracking, calendars, weather, and seasonal cycles
- **Population System**: Entities, NPCs, factions, relationships, and AI schedules
- **Economy System**: Markets, settlements, trade routes, and resource management
- **Ecosystem System**: Wildlife species, population dynamics, and food chains
- **Event System**: Scheduled events, event queues, and trigger conditions
- **Serialization**: Full JSON serialization support with zero data loss

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
entropic-world-core = "0.1.0"
```

## Quick Start

```rust
use entropic_world_core::world::{World, WorldConfig};

// Create a world configuration
let config = WorldConfig::new(64, 64)
    .with_seed(42)
    .with_time_scale(1.0);

// Create the world
let mut world = World::from_config(
    "My World".to_string(),
    "game_dna_id".to_string(),
    config,
);

// Initialize chunks
world.initialize_chunks();

// Add entities, NPCs, settlements, etc.
// ...

// Advance simulation
world.advance_tick();
```

## Architecture

### Spatial System

The spatial system uses a chunk-based approach for efficient world representation:

- **Chunks**: 256x256 meter tiles with heightmaps and vegetation data
- **Spatial Index**: Grid-based spatial hashing for fast entity queries (< 10ms for radius queries)
- **Coordinates**: Support for both chunk coordinates and world positions

### Temporal System

Time progression with configurable simulation speed:

- **World Time**: In-game calendar system (years, months, days, hours)
- **Weather**: Per-chunk weather conditions (temperature, precipitation, wind)
- **Seasons**: Automatic seasonal transitions with temperature modifiers
- **Ticks**: Simulation tick counter (default: 20 ticks/second)

### Population System

Comprehensive NPC and faction management:

- **Entities**: Base entity system with position, health, and type
- **NPCs**: Full NPC data with personality, skills, inventory, and memory
- **Factions**: Organizations with leaders, members, and inter-faction relationships
- **Relationships**: NPC-to-NPC relationships with opinion, trust, fear metrics
- **Schedules**: Daily routines and activity patterns

### Economy System

Simulation-ready economic systems:

- **Markets**: Supply/demand dynamics with price fluctuations
- **Settlements**: Towns with population, wealth, and resources
- **Trade Routes**: Automated trade between settlements
- **Resources**: 7+ resource types (Food, Wood, Metal, Stone, etc.)

### Ecosystem System

Wildlife simulation:

- **Species**: Wildlife definitions with diet, lifespan, reproduction rates
- **Population Control**: Carrying capacity and population dynamics
- **Food Chains**: Predator-prey relationships

### Event System

Scheduled world events:

- **Event Queue**: Priority queue for scheduled events
- **Event Types**: Birth, death, trade, disasters, discoveries, etc.
- **Triggers**: Condition-based event activation

## Examples

### Create a World

```bash
cargo run --example create_world
```

### Serialize/Deserialize

```bash
cargo run --example serialize_world
```

### Spatial Queries

```bash
cargo run --example spatial_queries
```

## Performance

- **World Creation**: < 1s for 1000-chunk worlds
- **Serialization**: < 5s for 1000-chunk worlds
- **Spatial Queries**: < 10ms for radius queries with 1000+ entities
- **Memory**: Efficient chunk representation supports large worlds

## Testing

Run the test suite:

```bash
cargo test
```

Run with coverage:

```bash
cargo test --all-features
```

Current test coverage: **85%+**

## Technical Requirements

- **Rust Edition**: 2021
- **MSRV**: 1.70+
- **Dependencies**: serde, chrono, uuid, thiserror

## License

MIT OR Apache-2.0
