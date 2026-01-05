# ENTROPIC Game DNA Core

The canonical Game DNA schema and data model for the ENTROPIC game engine. This library provides the foundational, type-safe representation of game configurations that serves as the single source of truth across the entire ENTROPIC system.

## 🎯 Overview

`entropic-dna-core` is a production-ready Rust library that defines the complete schema for representing game configurations in a deterministic, serializable format. Every game built with the ENTROPIC engine is defined by its Game DNA - a comprehensive blueprint that captures all aspects of the game's design, technical requirements, and behavioral characteristics.

## ✨ Features

- **Complete Game Schema**: 30+ configuration parameters covering gameplay, performance, AI, narrative, and business models
- **Type-Safe Design**: Leverages Rust's type system to prevent invalid configurations at compile time
- **Deterministic Serialization**: JSON serialization with sorted keys ensures consistent output across systems
- **Schema Evolution**: Built-in versioning and migration framework for future compatibility
- **Comprehensive Error Handling**: Detailed error messages with context and suggestions for resolution
- **Zero Unsafe Code**: 100% safe Rust implementation (MSRV 1.70+)
- **Workspace Ready**: Designed to work as a foundational library in larger Rust workspaces

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
entropic-dna-core = { path = "../entropic-dna-core" }
```

Or for published version:

```toml
[dependencies]
entropic-dna-core = "0.1.0"
```

## 🚀 Quick Start

### Creating a Simple Game DNA

```rust
use entropic_dna_core::{GameDNA, Genre, schema::TargetPlatform};

// Create a minimal valid GameDNA
let game = GameDNA::minimal(
    "My Awesome Game".to_string(),
    Genre::FPS,
    vec![TargetPlatform::PC]
);

println!("Created game: {}", game.name);
```

### Building a Comprehensive Configuration

```rust
use entropic_dna_core::{
    GameDNA,
    schema::{
        Genre, CameraMode, Tone, WorldScale, TargetPlatform,
        MonetizationModel, PhysicsProfile, DifficultyMode
    }
};

let game = GameDNA::builder()
    .name("Epic Space RPG".to_string())
    .genre(Genre::RPG)
    .camera(CameraMode::Perspective3D)
    .tone(Tone::Cinematic)
    .world_scale(WorldScale::OpenWorld)
    .target_platforms(vec![TargetPlatform::PC, TargetPlatform::Console])
    .physics_profile(PhysicsProfile::SemiRealistic)
    .max_players(4)
    .supports_coop(true)
    .difficulty(DifficultyMode::Dynamic)
    .monetization(MonetizationModel::PremiumBuy)
    .target_fps(60)
    .weather_enabled(true)
    .day_night_cycle(true)
    .has_campaign(true)
    .tag("multiplayer".to_string())
    .custom_property("engine", "entropic-v1")
    .build()?;
```

### Serialization and Deserialization

```rust
use entropic_dna_core::serialization::{to_json_string, from_json_str};

// Serialize to JSON
let json = to_json_string(&game)?;
println!("Game DNA JSON: {}", json);

// Deserialize back
let deserialized = from_json_str(&json)?;
assert_eq!(game.name, deserialized.name);
```

## 📋 Game DNA Schema

### Core Enums

**Genre**: FPS, RPG, TPS, Strategy, Casual, Horror, Racing, Simulation, Puzzle, Educational, CustomGenre(String)

**CameraMode**: Perspective2D, Perspective2_5D, Perspective3D, Isometric, VR, CustomCamera(String)

**Tone**: Realistic, Arcade, Cinematic, Stylized, Minimalist, CustomTone(String)

**WorldScale**: TinyLevel, SmallLevel, MediumLevel, LargeLevel, OpenWorld, Planet, Galaxy, CustomScale(String)

**TargetPlatform**: Mobile, PC, Console, XR, CloudStreamed, MultiPlatform

**MonetizationModel**: FreeToPlay, PremiumBuy, Subscription, OneTimePay, Hybrid, Custom(String)

**PhysicsProfile**: Arcade, SemiRealistic, Realistic, CustomPhysics(String)

**DifficultyMode**: Easy, Medium, Hard, Dynamic, CustomDifficulty(String)

### Core Struct

**GameDNA** contains all game configuration:

```rust
pub struct GameDNA {
    // Identity
    pub id: String,                    // UUID
    pub name: String,
    pub version: SemanticVersion,
    
    // Core Configuration
    pub genre: Genre,
    pub camera: CameraMode,
    pub tone: Tone,
    pub world_scale: WorldScale,
    pub target_platforms: Vec<TargetPlatform>,
    
    // Gameplay
    pub physics_profile: PhysicsProfile,
    pub max_players: u32,
    pub is_competitive: bool,
    pub supports_coop: bool,
    pub difficulty: DifficultyMode,
    
    // Business
    pub monetization: MonetizationModel,
    pub target_audience: String,
    pub esrb_rating: Option<String>,
    
    // Performance
    pub target_fps: u32,
    pub max_draw_distance: f32,
    pub max_entities: u32,
    pub max_npc_count: u32,
    
    // World Simulation
    pub time_scale: f32,
    pub weather_enabled: bool,
    pub seasons_enabled: bool,
    pub day_night_cycle: bool,
    pub persistent_world: bool,
    
    // AI & NPCs
    pub npc_count: u32,
    pub ai_enabled: bool,
    pub ai_difficulty_scaling: bool,
    
    // Narrative
    pub has_campaign: bool,
    pub has_side_quests: bool,
    pub dynamic_quests: bool,
    
    // Metadata
    pub tags: Vec<String>,
    pub custom_properties: HashMap<String, String>,
}
```

## 🔧 Modules

### `schema`
Core Game DNA types and enums. Defines the complete structure of game configurations.

### `serialization`
Deterministic JSON serialization with round-trip guarantees. Same input always produces same output.

### `version`
Schema versioning and migration framework. Handles evolution of the schema over time.

### `errors`
Comprehensive error types:
- `ParseError` - Invalid JSON/MessagePack data
- `SchemaError` - Configuration validation failures
- `SerializationError` - Encoding/decoding errors
- `VersionError` - Schema version mismatches

## 🎯 Examples

Run the examples with:

```bash
# Create a comprehensive 3D action RPG
cargo run --example create_simple_game

# Demonstrate JSON serialization/deserialization
cargo run --example serialize_game

# Show version handling and compatibility
cargo run --example version_handling

# Show complete validation workflow
cargo run --example validation_workflow
```

## 🧪 Testing

The library includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html

# Run benchmarks (when implemented)
cargo bench
```

## 📊 Code Quality

```bash
# Format code
cargo fmt

# Run clippy lints
cargo clippy -- -D warnings

# Run tests
cargo test

# Build documentation
cargo doc --open

# Check for security vulnerabilities
cargo audit
```

## 🔒 Safety

This library uses **no unsafe code**. All functionality is implemented in 100% safe Rust.

## 📈 Performance

- **Memory Efficient**: Minimal allocations, stack-optimized where possible
- **Deterministic**: Same input always produces same output
- **Fast Serialization**: Leverages serde_json's high-performance parser
- **Zero-Copy**: Where possible, borrows data instead of copying

## 🎮 Use Cases

### Game Engine Integration
```rust
// Load game configuration at engine startup
let game_dna = load_game_dna("game.dna.json")?;
engine.configure(&game_dna);
```

### Game Analytics
```rust
// Analyze game configurations across a portfolio
let games: Vec<GameDNA> = load_all_games()?;
let fps_games = games.iter().filter(|g| g.genre == Genre::FPS).count();
```

### Automated Testing
```rust
// Generate test configurations
let test_configs = generate_test_configurations();
for config in test_configs {
    assert!(config.validate().is_ok());
}
```

### Configuration Management
```rust
// Store and retrieve game configurations
let json = serialize_game_dna(&game)?;
save_to_database(game.id(), json)?;
```

## 🔄 Schema Evolution

The schema version is embedded in every GameDNA instance. Current version: **0.1.0**

When the schema evolves:
1. Version number increments according to semantic versioning
2. Migration framework handles converting old versions
3. Compatibility checks prevent data corruption
4. Clear error messages guide upgrade process

## 📝 License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/entropic-engine/entropic-dna-core
cd entropic-dna-core

# Install dependencies
cargo build

# Run tests
cargo test

# Run examples
cargo run --example create_simple_game
```

### Contribution Guidelines

1. All code must pass `cargo fmt` and `cargo clippy`
2. Add tests for new functionality
3. Update documentation for public APIs
4. Maintain 85%+ code coverage
5. No unsafe code without explicit justification

## 🔍 Comprehensive Validation Engine

The validation module provides a robust layer that ensures all game configurations are internally consistent, compatible, and deterministic before being locked and distributed downstream.

### Key Validation Features:

- **Genre ↔ Camera Compatibility**: Ensures FPS games use 3D cameras, strategy games use appropriate views, etc.
- **Genre ↔ Physics Profile**: Validates that arcade games use arcade physics, realistic games use realistic physics
- **Tone ↔ Gameplay Combinations**: Prevents contradictory combinations like cinematic tone with hyper-casual mechanics
- **Scale ↔ Platform Compatibility**: Ensures galaxy-scale games don't target mobile platforms
- **Monetization ↔ Gameplay**: Validates that free-to-play games have appropriate monetization strategies
- **Performance Constraints**: Checks that FPS targets are achievable on target platforms
- **World Simulation**: Validates weather, seasons, and time scale configurations
- **AI & NPC Constraints**: Ensures AI-enabled games have appropriate NPC counts
- **Campaign & Quest Logic**: Validates narrative consistency and quest system requirements

### Usage Example:

```rust
use entropic_dna_core::validation::{ValidationEngine, LockedGameDNABuilder};

// Create a validation engine
let engine = ValidationEngine::new();

// Validate a game configuration
let validation_result = engine.validate(&game);

if validation_result.is_valid {
    println!("✓ Configuration is valid!");
} else {
    println!("✗ Validation errors found:");
    for error in &validation_result.errors {
        println!("  - {}: {}", error.code, error.message);
        println!("    Fix: {}", error.details);
    }
}

// Publish as a locked, immutable configuration
let locked_builder = LockedGameDNABuilder::new(game);
let locked_game = locked_builder.publish()?;

println!("Checksum: {}", locked_game.checksum);
```

### Conflict Detection:

```rust
use entropic_dna_core::validation::ConflictDetector;

let detector = ConflictDetector::new();
let conflicts = detector.detect_conflicts(&game);

if !conflicts.errors.is_empty() {
    println!("Conflicts detected:");
    for error in &conflicts.errors {
        println!("  - {}: {}", error.code, error.message);
    }
}
```

### Deterministic Checksums:

```rust
use entropic_dna_core::validation::checksum;

// Generate a SHA-256 checksum for configuration integrity
let checksum = checksum::generate_checksum(&game);

// Verify integrity
let is_valid = checksum::verify_checksum(&game, &checksum);
```

## 📚 Architecture

```
entropic-dna-core/
├── Cargo.toml              # Dependencies and metadata
├── src/
│   ├── lib.rs             # Public API and integration tests
│   ├── schema/            # Core GameDNA types and enums
│   │   └── mod.rs
│   ├── serialization/     # JSON/MessagePack/Protobuf support
│   │   └── mod.rs
│   ├── version/           # Schema versioning and migrations
│   │   └── mod.rs
│   ├── validation/        # Comprehensive validation engine
│   │   ├── mod.rs         # Main validation API
│   │   ├── rules.rs       # Validation rule definitions
│   │   ├── constraints.rs # Constraint checking
│   │   ├── conflict_detector.rs # Conflict detection
│   │   └── checksum.rs    # Deterministic hashing
│   └── errors/            # Error types
│       └── mod.rs
├── examples/              # Example programs
│   ├── create_simple_game.rs
│   ├── serialize_game.rs
│   ├── version_handle.rs
│   └── validation_workflow.rs
└── tests/                 # Integration tests
```

## 🎓 Learning Resources

- [ENTROPIC Engine Documentation](https://docs.entropic.engine)
- [Game DNA Schema Specification](https://spec.entropic.engine)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## 🔗 Related Projects

- [entropic-engine-core](https://github.com/entropic-engine/core) - Main game engine
- [entropic-editor](https://github.com/entropic-engine/editor) - Visual editor
- [entropic-runtime](https://github.com/entropic-engine/runtime) - Runtime environment

## 📞 Support

- 📧 Email: support@entropic.engine
- 💬 Discord: [Entropic Engine Community](https://discord.gg/entropic)
- 🐛 Issues: [GitHub Issues](https://github.com/entropic-engine/entropic-dna-core/issues)
- 📖 Documentation: [docs.entropic.engine](https://docs.entropic.engine)

---

**Entropic Game DNA Core** - The Foundation of Next-Generation Game Development