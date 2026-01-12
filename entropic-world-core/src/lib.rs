//! # Entropic World Core
//!
//! Foundational data models for ENTROPIC's world simulation system.
//! This library provides the canonical schema for representing all persistent
//! world state—spatial, temporal, population, economy, and ecosystem data.
//!
//! ## Features
//!
//! - **Spatial System**: Chunks, coordinates, terrain, and spatial indexing
//! - **Temporal System**: World time, calendars, weather, and seasons
//! - **Population System**: Entities, NPCs, factions, and relationships
//! - **Economy System**: Markets, settlements, trade routes, and resources
//! - **Ecosystem System**: Species, population control, and food chains
//! - **Event System**: World events, event queues, and triggers
//! - **Serialization**: JSON and binary serialization support
//!
//! ## Example
//!
//! ```rust
//! use entropic_world_core::world::{World, WorldConfig};
//!
//! // Create a new world
//! let config = WorldConfig::new(64, 64)
//!     .with_seed(42)
//!     .with_time_scale(1.0);
//!
//! let mut world = World::from_config(
//!     "My World".to_string(),
//!     "game_dna_id".to_string(),
//!     config,
//! );
//!
//! // Initialize chunks
//! world.initialize_chunks();
//!
//! // Advance simulation
//! world.advance_tick();
//! ```

pub mod constants;
pub mod economy;
pub mod ecosystem;
pub mod errors;
pub mod events;
pub mod population;
pub mod serialization;
pub mod spatial;
pub mod temporal;
pub mod world;

pub use world::World;
pub use errors::{WorldError, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_world_creation() {
        let world = World::new(
            "Test World".to_string(),
            "game_dna_1".to_string(),
            10,
            10,
        );
        assert_eq!(world.name, "Test World");
    }
}
