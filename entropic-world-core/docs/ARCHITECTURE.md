# Entropic World Core Architecture

## Overview

The Entropic World Core is the foundational data model library for the ENTROPIC game engine's world simulation system. It provides a complete, deterministic, and serializable representation of all world state.

## Design Principles

1. **Deterministic**: Same seed → same world always
2. **Serializable**: Full JSON serialization with zero data loss
3. **Performant**: Efficient spatial queries (< 10ms for 1000+ entities)
4. **Modular**: Clear separation of concerns across systems
5. **Type-Safe**: Leverages Rust's type system for correctness

## Core Systems

### 1. Spatial System (`src/spatial/`)

The spatial system manages the physical layout and positioning of the world.

#### Components

- **Chunk**: 256x256 meter tile with heightmap, vegetation, and entity tracking
- **ChunkCoord**: 2D coordinate system for chunks
- **WorldPosition**: 3D float coordinates for precise positioning
- **SpatialIndex**: Grid-based spatial hashing for O(1) entity lookups
- **Terrain**: Biome definitions and structure management
- **Region**: Grouping of chunks for larger-scale organization

#### Spatial Index Design

```rust
SpatialIndex {
    grid: HashMap<(i32, i32), Vec<EntityId>>,
    grid_size: 16.0 meters
}
```

The spatial index uses a grid-based approach where:
- World is divided into 16x16 meter cells
- Each cell stores entity IDs
- Radius queries check only nearby cells
- Achieves sub-millisecond query times

### 2. Temporal System (`src/temporal/`)

Manages time progression and environmental conditions.

#### Components

- **WorldTime**: In-game calendar (year, month, day, hour, minute, second, tick)
- **Calendar**: Customizable calendar system with month/day names
- **Weather**: Per-chunk weather conditions (temperature, humidity, wind, precipitation)
- **Seasons**: Automatic seasonal transitions with temperature modifiers

#### Time Progression

```
1 second (real) = 20 ticks
1 tick = 50ms
Configurable time_scale multiplier
```

### 3. Population System (`src/population/`)

Comprehensive NPC and faction management.

#### Components

- **Entity**: Base entity with position, health, type
- **NPC**: Full NPC data including:
  - Personality traits (6 dimensions)
  - Skills (proficiency-based)
  - Inventory
  - Daily schedules
  - Memory system
  - Relationships
- **Faction**: Organizations with:
  - Leadership hierarchy
  - Member tracking
  - Inter-faction relationships (allies/enemies)
  - Resources and power
- **Relationship**: NPC-to-NPC relationships with opinion, trust, fear, attraction

#### Memory System

NPCs maintain:
- Recent events (sorted by importance, capacity-limited)
- Long-term knowledge (key-value store)
- Automatic pruning of low-importance memories

### 4. Economy System (`src/economy/`)

Full economic simulation support.

#### Components

- **Market**: Supply/demand dynamics
  - Price fluctuations based on supply/demand ratio
  - Volatility modeling
  - Resource trading
- **Settlement**: Towns and outposts
  - Population tracking
  - Wealth management
  - Building ownership
  - Resource storage
- **TradeRoute**: Automated trade between settlements
  - Frequency-based execution
  - Caravan size
  - Activation/deactivation
- **Resource**: 7+ resource types (Food, Wood, Metal, Stone, Cloth, Herbs, Gold)

### 5. Ecosystem System (`src/ecosystem/`)

Wildlife and population dynamics.

#### Components

- **Species**: Wildlife definitions
  - Diet (herbivore, carnivore, omnivore)
  - Reproduction rate
  - Lifespan
  - Preferred biomes
  - Food chain relationships
- **PopulationControl**: Population dynamics
  - Birth/death rates
  - Carrying capacity
  - Automatic population balancing
- **FoodChain**: Predator-prey relationships

### 6. Event System (`src/events/`)

Scheduled world events.

#### Components

- **WorldEvent**: Event structure with:
  - Type (Birth, Death, Trade, Disaster, etc.)
  - Timestamp
  - Location
  - Involved entities
  - Description
- **EventQueue**: Priority queue for scheduled events
  - Tick-based scheduling
  - Automatic event firing
- **EventTrigger**: Condition-based event activation
  - Time-based triggers
  - Population thresholds
  - Resource depletion
  - Custom conditions

### 7. Serialization System (`src/serialization/`)

Full serialization support.

#### Features

- JSON serialization (human-readable)
- Binary serialization (compact)
- Custom serialization for non-string HashMap keys
- Chunk compression support
- Version migration framework

#### Serialization Design

```rust
// Custom serialization for HashMap<ChunkCoord, Chunk>
fn serialize_chunk_map() -> Vec<(ChunkCoord, Chunk)>
fn deserialize_chunk_map() -> HashMap<ChunkCoord, Chunk>
```

## Data Flow

```
World Creation
    ↓
Chunk Initialization
    ↓
Entity/NPC Addition
    ↓
Spatial Index Updates
    ↓
Simulation Loop
    ├─ Advance Tick
    ├─ Update Time
    ├─ Process Events
    ├─ Update Markets
    ├─ Update Populations
    └─ Save State (optional)
```

## Performance Characteristics

### Memory

- Chunk: ~260KB (256x256 heightmap + vegetation)
- Entity: ~200 bytes
- NPC: ~2KB (with full data)
- 1000-chunk world: ~260MB base + entities

### Speed

- World creation: < 1s for 1000 chunks
- Chunk initialization: ~1ms per chunk
- Spatial query (radius): < 10ms for 1000+ entities
- Serialization: < 5s for 1000-chunk world
- Entity addition: < 1ms

## Thread Safety

Current implementation is single-threaded. For multi-threaded access:
- Wrap World in `Arc<RwLock<World>>`
- Use message passing for entity updates
- Consider chunk-level locking for parallel chunk updates

## Extension Points

### Adding New Entity Types

```rust
pub enum EntityType {
    NPC,
    Animal,
    Structure,
    Item,
    Effect,
    // Add new types here
    Custom(u32),
}
```

### Adding New Resource Types

```rust
pub enum ResourceType {
    Food,
    Wood,
    // ... existing types
    Custom(u32), // Use for game-specific resources
}
```

### Custom Event Types

```rust
pub enum EventType {
    // ... existing types
    Custom(String), // Use for game-specific events
}
```

## Best Practices

### Chunk Management

- Initialize chunks on-demand for large worlds
- Unload distant chunks to save memory
- Use chunk regions for multi-chunk operations

### Spatial Queries

- Prefer radius queries over exhaustive entity iteration
- Use spatial index for collision detection
- Batch similar queries together

### Entity Updates

- Update spatial index when entities move
- Batch entity updates per tick
- Use entity IDs for references, not direct pointers

### Serialization

- Save world state at regular intervals
- Use delta encoding for incremental saves
- Compress heightmap data for large worlds

## Testing Strategy

- **Unit Tests**: Each module has comprehensive unit tests
- **Integration Tests**: Full world lifecycle tests
- **Performance Tests**: Spatial query benchmarks
- **Serialization Tests**: Round-trip verification
- **Coverage**: 85%+ test coverage

## Future Enhancements

### Planned Features

- [ ] Chunk streaming for infinite worlds
- [ ] Multi-threaded simulation
- [ ] Network serialization (binary protocol)
- [ ] Hot-reload support for world edits
- [ ] World diff/patch system
- [ ] Async event processing

### Optimization Opportunities

- [ ] SIMD for heightmap operations
- [ ] Chunk compression (LZ4/Zstd)
- [ ] Entity pooling
- [ ] Lazy deserialization
- [ ] Memory-mapped file storage
