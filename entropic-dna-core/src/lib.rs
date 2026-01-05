//! ENTROPIC Game DNA Core Library
//! 
//! The canonical Game DNA schema and data model for the ENTROPIC game engine.
//! This library provides type-safe, deterministic representation of game configurations
//! with comprehensive serialization and version management.
//! 
//! # Core Components
//! 
//! - **`schema`** - Core Game DNA types and enums defining the complete game configuration structure
//! - **`serialization`** - Deterministic JSON serialization with round-trip guarantees
//! - **`version`** - Schema versioning and migration framework
//! - **`errors`** - Comprehensive error types for parsing, validation, and serialization
//! 
//! # Features
//! 
//! - Type-safe Game DNA schema with 30+ configuration parameters
//! - Deterministic JSON serialization (sorted keys, consistent formatting)
//! - Semantic versioning and schema evolution support
//! - Custom error types with detailed diagnostic information
//! - Extensible design with custom properties and tags
//! - Builder pattern for ergonomic GameDNA construction
//! - No unsafe code - written in 100% safe Rust
//! 
//! # Example
//! 
//! ```rust
//! use entropic_dna_core::{
//!     GameDNA,
//!     schema::{Genre, CameraMode, Tone, WorldScale, TargetPlatform, MonetizationModel, PhysicsProfile, DifficultyMode}
//! };
//! 
//! // Create a simple game configuration
//! let game = GameDNA::minimal("My FPS Game".to_string(), Genre::FPS, vec![TargetPlatform::PC]);
//! 
//! // Or use the builder for comprehensive configuration
//! let complex_game = GameDNA::builder()
//!     .name("Epic RPG Adventure".to_string())
//!     .genre(Genre::RPG)
//!     .camera(CameraMode::Perspective3D)
//!     .tone(Tone::Cinematic)
//!     .world_scale(WorldScale::OpenWorld)
//!     .target_platforms(vec![TargetPlatform::PC, TargetPlatform::Console])
//!     .physics_profile(PhysicsProfile::SemiRealistic)
//!     .max_players(4)
//!     .supports_coop(true)
//!     .difficulty(DifficultyMode::Dynamic)
//!     .monetization(MonetizationModel::PremiumBuy)
//!     .target_fps(60)
//!     .weather_enabled(true)
//!     .day_night_cycle(true)
//!     .has_campaign(true)
//!     .has_side_quests(true)
//!     .tag("multiplayer".to_string())
//!     .tag("open-world".to_string())
//!     .custom_property("engine_version", "1.0".to_string())
//!     .build()?;
//! 
//! // Serialize to JSON
//! let json = entropic_dna_core::serialization::to_json_string(&complex_game)?;
//! println!("Game DNA: {}", json);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![deny(unsafe_code)]

// Re-export commonly used types at the crate root
pub use crate::schema::{GameDNA, GameDNABuilder, SemanticVersion};

pub mod errors;
pub mod schema;
pub mod serialization;
pub mod validation;
pub mod version;

#[cfg(test)]
mod tests;

/// Type alias for Results using entropic-dna-core's error types
pub type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

/// Re-export of version constants for convenience
pub use version::{CURRENT_VERSION, MINIMUM_COMPATIBLE_VERSION};

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::schema::*;
    
    #[test]
    fn test_complete_game_dna_lifecycle() {
        // Create a comprehensive GameDNA
        let original = GameDNA::builder()
            .name("Epic Space Adventure".to_string())
            .genre(Genre::RPG)
            .camera(CameraMode::Perspective3D)
            .tone(Tone::Cinematic)
            .world_scale(WorldScale::Galaxy)
            .target_platforms(vec![TargetPlatform::PC, TargetPlatform::Console])
            .physics_profile(PhysicsProfile::SemiRealistic)
            .max_players(1)
            .is_competitive(false)
            .supports_coop(false)
            .difficulty(DifficultyMode::Dynamic)
            .monetization(MonetizationModel::PremiumBuy)
            .target_audience("Teen to Adult".to_string())
            .esrb_rating(Some("T".to_string()))
            .target_fps(60)
            .max_draw_distance(5000.0)
            .max_entities(10000)
            .max_npc_count(500)
            .time_scale(1.0)
            .weather_enabled(true)
            .seasons_enabled(true)
            .day_night_cycle(true)
            .persistent_world(false)
            .npc_count(200)
            .ai_enabled(true)
            .ai_difficulty_scaling(true)
            .has_campaign(true)
            .has_side_quests(true)
            .dynamic_quests(true)
            .tag("sci-fi".to_string())
            .tag("space".to_string())
            .tag("exploration".to_string())
            .custom_property("galaxy_count", "5".to_string())
            .custom_property("has_alien_races", "true".to_string())
            .build()
            .expect("GameDNA should be valid");
        
        // Validate
        original.validate().expect("GameDNA should validate");
        
        // Serialize to JSON
        let json = serialization::to_json_string(&original)
            .expect("Serialization should succeed");
        
        // Deserialize back
        let deserialized = serialization::from_json_str(&json)
            .expect("Deserialization should succeed");
        
        // Verify all fields match
        assert_eq!(original.id, deserialized.id);
        assert_eq!(original.name, deserialized.name);
        assert_eq!(original.version, deserialized.version);
        assert_eq!(original.genre, deserialized.genre);
        assert_eq!(original.camera, deserialized.camera);
        assert_eq!(original.tone, deserialized.tone);
        assert_eq!(original.world_scale, deserialized.world_scale);
        assert_eq!(original.target_platforms, deserialized.target_platforms);
        assert_eq!(original.physics_profile, deserialized.physics_profile);
        assert_eq!(original.max_players, deserialized.max_players);
        assert_eq!(original.is_competitive, deserialized.is_competitive);
        assert_eq!(original.supports_coop, deserialized.supports_coop);
        assert_eq!(original.difficulty, deserialized.difficulty);
        assert_eq!(original.monetization, deserialized.monetization);
        assert_eq!(original.target_audience, deserialized.target_audience);
        assert_eq!(original.esrb_rating, deserialized.esrb_rating);
        assert_eq!(original.target_fps, deserialized.target_fps);
        assert_eq!(original.max_draw_distance, deserialized.max_draw_distance);
        assert_eq!(original.max_entities, deserialized.max_entities);
        assert_eq!(original.max_npc_count, deserialized.max_npc_count);
        assert_eq!(original.time_scale, deserialized.time_scale);
        assert_eq!(original.weather_enabled, deserialized.weather_enabled);
        assert_eq!(original.seasons_enabled, deserialized.seasons_enabled);
        assert_eq!(original.day_night_cycle, deserialized.day_night_cycle);
        assert_eq!(original.persistent_world, deserialized.persistent_world);
        assert_eq!(original.npc_count, deserialized.npc_count);
        assert_eq!(original.ai_enabled, deserialized.ai_enabled);
        assert_eq!(original.ai_difficulty_scaling, deserialized.ai_difficulty_scaling);
        assert_eq!(original.has_campaign, deserialized.has_campaign);
        assert_eq!(original.has_side_quests, deserialized.has_side_quests);
        assert_eq!(original.dynamic_quests, deserialized.dynamic_quests);
        assert_eq!(original.tags, deserialized.tags);
        assert_eq!(original.custom_properties, deserialized.custom_properties);
    }
    
    #[test]
    fn test_all_genres() {
        let platforms = vec![TargetPlatform::PC];
        
        let genres = vec![
            Genre::FPS,
            Genre::RPG,
            Genre::TPS,
            Genre::Strategy,
            Genre::Casual,
            Genre::Horror,
            Genre::Racing,
            Genre::Simulation,
            Genre::Puzzle,
            Genre::Educational,
            Genre::CustomGenre("Battle Royale".to_string()),
        ];
        
        for genre in genres {
            let game = GameDNA::minimal("Test Game".to_string(), genre.clone(), platforms.clone());
            assert_eq!(game.genre, genre);
            
            let json = serialization::to_json_string(&game)
                .expect(&format!("Genre {:?} should serialize", genre));
            let deserialized = serialization::from_json_str(&json)
                .expect(&format!("Genre {:?} should deserialize", genre));
            
            assert_eq!(deserialized.genre, genre);
        }
    }
    
    #[test]
    fn test_all_camera_modes() {
        let mut game = GameDNA::minimal("Test".to_string(), Genre::FPS, vec![TargetPlatform::PC]);
        
        let cameras = vec![
            CameraMode::Perspective2D,
            CameraMode::Perspective2_5D,
            CameraMode::Perspective3D,
            CameraMode::Isometric,
            CameraMode::VR,
            CameraMode::CustomCamera("Cinematic".to_string()),
        ];
        
        for camera in cameras {
            game.camera = camera.clone();
            let json = serialization::to_json_string(&game)
                .expect(&format!("Camera {:?} should serialize", camera));
            let deserialized = serialization::from_json_str(&json)
                .expect(&format!("Camera {:?} should deserialize", camera));
            
            assert_eq!(deserialized.camera, camera);
        }
    }
    
    #[test]
    fn test_invalid_game_dna_validation() {
        // Empty name should fail validation
        let mut builder = GameDNA::builder();
        builder.name = String::new();
        let result = builder.build();
        assert!(result.is_err());
        
        // Zero FPS should fail validation
        let result = GameDNA::builder()
            .name("Test".to_string())
            .genre(Genre::FPS)
            .target_platforms(vec![TargetPlatform::PC])
            .target_fps(0)
            .build();
        assert!(result.is_err());
        
        // No platforms should fail validation
        let result = GameDNA::builder()
            .name("Test".to_string())
            .genre(Genre::FPS)
            .target_platforms(vec![])
            .build();
        assert!(result.is_err());
        
        // Invalid time scale should fail validation
        let result = GameDNA::builder()
            .name("Test".to_string())
            .genre(Genre::FPS)
            .target_platforms(vec![TargetPlatform::PC])
            .time_scale(0.0)
            .build();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_version_compatibility() {
        let manager = version::VersionManager::new();
        
        // Current version should be compatible
        assert!(manager.is_compatible(version::CURRENT_VERSION));
        
        // Old version should not be compatible
        assert!(!manager.is_compatible("0.0.1"));
        
        // Future version should not be compatible with current implementation
        assert!(!manager.is_compatible("1.0.0"));
    }
    
    #[test]
    fn test_semantic_version() {
        let v1 = SemanticVersion::new(1, 2, 3);
        assert_eq!(v1.major, 1);
        assert_eq!(v1.minor, 2);
        assert_eq!(v1.patch, 3);
        assert_eq!(v1.to_string(), "1.2.3");
        
        let v2 = SemanticVersion::default_version();
        assert_eq!(v2.major, 0);
        assert_eq!(v2.minor, 1);
        assert_eq!(v2.patch, 0);
        assert_eq!(v2.to_string(), "0.1.0");
    }
}