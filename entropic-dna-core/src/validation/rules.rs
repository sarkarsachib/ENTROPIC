//! Validation rules for Game DNA configurations
//!
//! This module contains all the specific validation rules for different aspects
//! of game configurations, including genre compatibility, performance constraints,
//! and logical consistency checks.

use crate::schema::{GameDNA, Genre, CameraMode, Tone, WorldScale, TargetPlatform, MonetizationModel, PhysicsProfile};
use crate::validation::{ValidationResult, ValidationError, ValidationWarning};

/// Validate basic required fields
pub fn validate_basic_fields(game_dna: &GameDNA, result: &mut ValidationResult) {
    if game_dna.name.is_empty() {
        result.add_error(ValidationError::new(
            "EMPTY_NAME".to_string(),
            "name".to_string(),
            "Game name cannot be empty".to_string(),
            "Provide a meaningful name for the game".to_string(),
        ));
    }

    if game_dna.id.is_empty() {
        result.add_error(ValidationError::new(
            "EMPTY_ID".to_string(),
            "id".to_string(),
            "Game ID cannot be empty".to_string(),
            "Generate a UUID for the game ID".to_string(),
        ));
    }

    if game_dna.target_platforms.is_empty() {
        result.add_error(ValidationError::new(
            "NO_TARGET_PLATFORMS".to_string(),
            "target_platforms".to_string(),
            "At least one target platform must be specified".to_string(),
            "Add at least one platform (PC, Mobile, Console, etc.)".to_string(),
        ));
    }
}

/// Validate name field
pub fn validate_name(game_dna: &GameDNA, result: &mut ValidationResult) {
    if game_dna.name.is_empty() {
        result.add_error(ValidationError::new(
            "EMPTY_NAME".to_string(),
            "name".to_string(),
            "Game name cannot be empty".to_string(),
            "Provide a meaningful name for the game".to_string(),
        ));
    } else if game_dna.name.len() > 100 {
        result.add_warning(ValidationWarning::new(
            "LONG_NAME".to_string(),
            "name".to_string(),
            "Game name is very long".to_string(),
            "Consider using a shorter name (under 100 characters)".to_string(),
        ));
    }
}

/// Validate genre field
pub fn validate_genre(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Genre is an enum, so it's always valid at the type level
    // We could add specific genre validation logic here if needed
}

/// Validate camera field
pub fn validate_camera(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Camera is an enum, so it's always valid at the type level
}

/// Validate tone field
pub fn validate_tone(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Tone is an enum, so it's always valid at the type level
}

/// Validate world scale field
pub fn validate_world_scale(game_dna: &GameDNA, result: &mut ValidationResult) {
    // World scale is an enum, so it's always valid at the type level
}

/// Validate target platforms field
pub fn validate_target_platforms(game_dna: &GameDNA, result: &mut ValidationResult) {
    if game_dna.target_platforms.is_empty() {
        result.add_error(ValidationError::new(
            "NO_TARGET_PLATFORMS".to_string(),
            "target_platforms".to_string(),
            "At least one target platform must be specified".to_string(),
            "Add at least one platform (PC, Mobile, Console, etc.)".to_string(),
        ));
    }
}

/// Validate physics profile field
pub fn validate_physics_profile(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Physics profile is an enum, so it's always valid at the type level
}

/// Validate max players field
pub fn validate_max_players(game_dna: &GameDNA, result: &mut ValidationResult) {
    if game_dna.max_players == 0 {
        result.add_warning(ValidationWarning::new(
            "ZERO_PLAYERS".to_string(),
            "max_players".to_string(),
            "Max players is set to 0".to_string(),
            "Set max_players to at least 1 for a playable game".to_string(),
        ));
    } else if game_dna.max_players > 1000 {
        result.add_warning(ValidationWarning::new(
            "HIGH_PLAYER_COUNT".to_string(),
            "max_players".to_string(),
            "Very high player count".to_string(),
            "Consider whether this player count is realistic for your game type".to_string(),
        ));
    }
}

/// Validate target FPS field
pub fn validate_target_fps(game_dna: &GameDNA, result: &mut ValidationResult) {
    if game_dna.target_fps == 0 {
        result.add_error(ValidationError::new(
            "ZERO_FPS".to_string(),
            "target_fps".to_string(),
            "Target FPS cannot be 0".to_string(),
            "Set target_fps to a reasonable value (30-120 for most games)".to_string(),
        ));
    } else if game_dna.target_fps > 240 {
        result.add_warning(ValidationWarning::new(
            "HIGH_FPS_TARGET".to_string(),
            "target_fps".to_string(),
            "Very high FPS target".to_string(),
            "Consider whether this FPS target is achievable on your target platforms".to_string(),
        ));
    }

    // Platform-specific FPS validation
    for platform in &game_dna.target_platforms {
        match platform {
            TargetPlatform::Mobile => {
                if game_dna.target_fps > 120 {
                    result.add_warning(ValidationWarning::new(
                        "MOBILE_HIGH_FPS".to_string(),
                        "target_fps".to_string(),
                        "High FPS target for mobile platform".to_string(),
                        "Mobile devices typically target 30-60 FPS, with 120 being the upper limit for high-end devices".to_string(),
                    ));
                }
            }
            TargetPlatform::Console => {
                if game_dna.target_fps > 120 {
                    result.add_warning(ValidationWarning::new(
                        "CONSOLE_HIGH_FPS".to_string(),
                        "target_fps".to_string(),
                        "High FPS target for console platform".to_string(),
                        "Consoles typically target 30-60 FPS".to_string(),
                    ));
                }
            }
            _ => {} // PC and other platforms can handle higher FPS
        }
    }
}

/// Validate time scale field
pub fn validate_time_scale(game_dna: &GameDNA, result: &mut ValidationResult) {
    if game_dna.time_scale <= 0.0 {
        result.add_error(ValidationError::new(
            "INVALID_TIME_SCALE".to_string(),
            "time_scale".to_string(),
            "Time scale must be positive".to_string(),
            "Set time_scale to a positive value (1.0 = real-time)".to_string(),
        ));
    } else if game_dna.time_scale > 100.0 {
        result.add_warning(ValidationWarning::new(
            "HIGH_TIME_SCALE".to_string(),
            "time_scale".to_string(),
            "Very high time scale".to_string(),
            "High time scale values can cause simulation instability".to_string(),
        ));
    }

    // Auto-fix: if day_night_cycle is enabled but time_scale is 0, suggest setting to 1.0
    if game_dna.day_night_cycle && game_dna.time_scale == 0.0 {
        result.add_warning(ValidationWarning::new(
            "DAY_NIGHT_WITHOUT_TIME_SCALE".to_string(),
            "time_scale".to_string(),
            "Day/night cycle enabled but time_scale is 0".to_string(),
            "Set time_scale to 1.0 for real-time day/night cycle".to_string(),
        ));
    }
}

/// Validate NPC count field
pub fn validate_npc_count(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Check if AI is enabled but NPC count is 0
    if game_dna.ai_enabled && game_dna.npc_count == 0 {
        result.add_warning(ValidationWarning::new(
            "AI_WITHOUT_NPC".to_string(),
            "npc_count".to_string(),
            "AI is enabled but NPC count is 0".to_string(),
            "Set npc_count to a positive value when AI is enabled".to_string(),
        ));
    }

    // Validate NPC count against world scale
    match game_dna.world_scale {
        WorldScale::TinyLevel => {
            if game_dna.npc_count > 50 {
                result.add_warning(ValidationWarning::new(
                    "NPC_COUNT_TOO_HIGH_FOR_SCALE".to_string(),
                    "npc_count".to_string(),
                    "NPC count too high for TinyLevel world scale".to_string(),
                    "TinyLevel worlds should have max 50 NPCs".to_string(),
                ));
            }
        }
        WorldScale::SmallLevel => {
            if game_dna.npc_count > 200 {
                result.add_warning(ValidationWarning::new(
                    "NPC_COUNT_TOO_HIGH_FOR_SCALE".to_string(),
                    "npc_count".to_string(),
                    "NPC count too high for SmallLevel world scale".to_string(),
                    "SmallLevel worlds should have max 200 NPCs".to_string(),
                ));
            }
        }
        WorldScale::LargeLevel => {
            if game_dna.npc_count > 1000 {
                result.add_warning(ValidationWarning::new(
                    "NPC_COUNT_TOO_HIGH_FOR_SCALE".to_string(),
                    "npc_count".to_string(),
                    "NPC count too high for LargeLevel world scale".to_string(),
                    "LargeLevel worlds should have max 1000 NPCs".to_string(),
                ));
            }
        }
        WorldScale::OpenWorld => {
            if game_dna.npc_count > 5000 {
                result.add_warning(ValidationWarning::new(
                    "NPC_COUNT_TOO_HIGH_FOR_SCALE".to_string(),
                    "npc_count".to_string(),
                    "NPC count very high for OpenWorld".to_string(),
                    "OpenWorld games typically have max 5000 NPCs".to_string(),
                ));
            }
        }
        _ => {} // Other scales don't have specific NPC limits
    }
}

/// Validate genre and camera compatibility
pub fn validate_genre_camera_compatibility(game_dna: &GameDNA, result: &mut ValidationResult) {
    match game_dna.genre {
        Genre::FPS | Genre::TPS => {
            // FPS/TPS games require 3D camera
            if !matches!(game_dna.camera, CameraMode::Perspective3D | CameraMode::VR) {
                result.add_error(ValidationError::new(
                    "INCOMPATIBLE_CAMERA_FOR_GENRE".to_string(),
                    "camera".to_string(),
                    format!("{} games require first-person or third-person 3D camera, not {:?}", 
                        match game_dna.genre {
                            Genre::FPS => "FPS",
                            Genre::TPS => "TPS",
                            _ => "Shooter",
                        },
                        game_dna.camera),
                    "Change camera to Perspective3D for immersive shooter experience".to_string(),
                ));
            }
        }
        Genre::Strategy => {
            // Strategy games work with Isometric or 3D, rarely 1st-person
            if matches!(game_dna.camera, CameraMode::Perspective2D) {
                result.add_warning(ValidationWarning::new(
                    "UNCOMMON_CAMERA_FOR_STRATEGY".to_string(),
                    "camera".to_string(),
                    "Strategy games rarely use 2D perspective".to_string(),
                    "Consider using Isometric or Perspective3D for strategy games".to_string(),
                ));
            }
        }
        Genre::Racing => {
            // Racing games typically use 3rd-person or custom camera
            if matches!(game_dna.camera, CameraMode::Perspective2D | CameraMode::Isometric) {
                result.add_warning(ValidationWarning::new(
                    "UNCOMMON_CAMERA_FOR_RACING".to_string(),
                    "camera".to_string(),
                    "Racing games typically don't use 2D or Isometric cameras".to_string(),
                    "Consider using Perspective3D or a custom racing camera".to_string(),
                ));
            }
        }
        Genre::Horror => {
            // Horror can be 1st-person 3D or 3rd-person
            if matches!(game_dna.camera, CameraMode::Perspective2D | CameraMode::Isometric) {
                result.add_warning(ValidationWarning::new(
                    "UNCOMMON_CAMERA_FOR_HORROR".to_string(),
                    "camera".to_string(),
                    "Horror games typically use 1st-person or 3rd-person 3D cameras".to_string(),
                    "Consider using Perspective3D for immersive horror experience".to_string(),
                ));
            }
        }
        Genre::Puzzle => {
            // 2D Puzzle games can use 2D or 2.5D camera only
            if matches!(game_dna.camera, CameraMode::Perspective3D | CameraMode::VR) {
                result.add_warning(ValidationWarning::new(
                    "UNCOMMON_CAMERA_FOR_PUZZLE".to_string(),
                    "camera".to_string(),
                    "2D puzzle games typically don't use full 3D cameras".to_string(),
                    "Consider using Perspective2D or Perspective2_5D for puzzle games".to_string(),
                ));
            }
        }
        Genre::Casual => {
            // Casual games are flexible with camera modes
            // No specific validation needed
        }
        _ => {} // Other genres don't have specific camera requirements
    }
}

/// Validate genre and physics profile compatibility
pub fn validate_genre_physics_compatibility(game_dna: &GameDNA, result: &mut ValidationResult) {
    match game_dna.genre {
        Genre::FPS | Genre::TPS | Genre::Racing | Genre::Simulation => {
            // Arcade games require Arcade physics
            if matches!(game_dna.genre, Genre::FPS | Genre::TPS) && game_dna.physics_profile == PhysicsProfile::Arcade {
                result.add_warning(ValidationWarning::new(
                    "ARCADE_PHYSICS_FOR_SHOOTER".to_string(),
                    "physics_profile".to_string(),
                    "Shooter games with Arcade physics may feel less realistic".to_string(),
                    "Consider using SemiRealistic or Realistic physics for shooters".to_string(),
                ));
            }
        }
        Genre::Racing => {
            if game_dna.physics_profile == PhysicsProfile::Realistic {
                // Realistic physics is good for racing
            } else if game_dna.physics_profile == PhysicsProfile::Arcade {
                result.add_warning(ValidationWarning::new(
                    "ARCADE_PHYSICS_FOR_RACING".to_string(),
                    "physics_profile".to_string(),
                    "Racing games with Arcade physics may feel less realistic".to_string(),
                    "Consider using Realistic physics for realistic racing simulation".to_string(),
                ));
            }
        }
        Genre::RPG => {
            if !matches!(game_dna.physics_profile, PhysicsProfile::SemiRealistic | PhysicsProfile::Realistic) {
                result.add_warning(ValidationWarning::new(
                    "UNCOMMON_PHYSICS_FOR_RPG".to_string(),
                    "physics_profile".to_string(),
                    "RPG games typically use SemiRealistic or Realistic physics".to_string(),
                    "Consider using SemiRealistic physics for a good balance".to_string(),
                ));
            }
        }
        Genre::Horror => {
            if !matches!(game_dna.physics_profile, PhysicsProfile::SemiRealistic | PhysicsProfile::Realistic) {
                result.add_warning(ValidationWarning::new(
                    "UNCOMMON_PHYSICS_FOR_HORROR".to_string(),
                    "physics_profile".to_string(),
                    "Horror games typically use SemiRealistic or Realistic physics".to_string(),
                    "Consider using SemiRealistic physics for survival horror games".to_string(),
                ));
            }
        }
        Genre::Casual | Genre::Puzzle | Genre::Educational => {
            if game_dna.physics_profile == PhysicsProfile::Realistic {
                result.add_warning(ValidationWarning::new(
                    "REALISTIC_PHYSICS_FOR_CASUAL".to_string(),
                    "physics_profile".to_string(),
                    "Casual games with Realistic physics may be too complex".to_string(),
                    "Consider using Arcade physics for casual games".to_string(),
                ));
            }
        }
        _ => {} // Other genres don't have specific physics requirements
    }
}

/// Validate tone and gameplay combinations
pub fn validate_tone_gameplay_combinations(game_dna: &GameDNA, result: &mut ValidationResult) {
    match game_dna.tone {
        Tone::Cinematic => {
            // Cinematic tone should focus on campaign, not competitive play
            if game_dna.is_competitive {
                result.add_warning(ValidationWarning::new(
                    "CINEMATIC_WITH_COMPETITIVE".to_string(),
                    "tone".to_string(),
                    "Cinematic tone with competitive gameplay is unusual".to_string(),
                    "Cinematic games typically focus on story-driven campaign experiences".to_string(),
                ));
            }
        }
        Tone::Arcade => {
            // Arcade tone supports fast-paced competitive play
            // This is a good combination, no warnings needed
        }
        Tone::Realistic => {
            // Realistic tone conflicts with hyper-casual mechanics
            if matches!(game_dna.genre, Genre::Casual) && game_dna.max_players > 4 {
                result.add_warning(ValidationWarning::new(
                    "REALISTIC_WITH_CASUAL".to_string(),
                    "tone".to_string(),
                    "Realistic tone with casual game mechanics is unusual".to_string(),
                    "Consider using Arcade or Stylized tone for casual games".to_string(),
                ));
            }
        }
        Tone::Stylized => {
            // Stylized tone works with most genres
            // No specific validation needed
        }
        _ => {} // Other tones don't have specific gameplay requirements
    }
}

/// Validate scale and platform compatibility
pub fn validate_scale_platform_compatibility(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Galaxy/Planet scale is PC/Console only, not Mobile
    match game_dna.world_scale {
        WorldScale::Galaxy | WorldScale::Planet => {
            if game_dna.target_platforms.contains(&TargetPlatform::Mobile) {
                result.add_error(ValidationError::new(
                    "SCALE_NOT_SUPPORTED_ON_MOBILE".to_string(),
                    "world_scale".to_string(),
                    format!("{:?} scale is not supported on Mobile platforms", game_dna.world_scale),
                    "Galaxy and Planet scale games require PC or Console platforms".to_string(),
                ));
            }
        }
        WorldScale::OpenWorld => {
            if game_dna.target_platforms.contains(&TargetPlatform::Mobile) {
                result.add_warning(ValidationWarning::new(
                    "OPEN_WORLD_ON_MOBILE".to_string(),
                    "world_scale".to_string(),
                    "OpenWorld scale on Mobile is challenging".to_string(),
                    "OpenWorld games on mobile require careful optimization and design".to_string(),
                ));
            }
        }
        _ => {} // Other scales are fine on all platforms
    }

    // XR platform has max world scale of LargeLevel
    if game_dna.target_platforms.contains(&TargetPlatform::XR) {
        match game_dna.world_scale {
            WorldScale::OpenWorld | WorldScale::Planet | WorldScale::Galaxy => {
                result.add_error(ValidationError::new(
                    "SCALE_TOO_LARGE_FOR_XR".to_string(),
                    "world_scale".to_string(),
                    format!("{:?} scale is too large for XR platforms", game_dna.world_scale),
                    "XR platforms support maximum LargeLevel scale".to_string(),
                ));
            }
            _ => {} // Other scales are fine for XR
        }
    }
}

/// Validate monetization and gameplay combinations
pub fn validate_monetization_gameplay(game_dna: &GameDNA, result: &mut ValidationResult) {
    match game_dna.monetization {
        MonetizationModel::OneTimePay => {
            // One-time pay should focus on single-player or co-op
            if game_dna.is_competitive && !game_dna.supports_coop {
                result.add_warning(ValidationWarning::new(
                    "ONE_TIME_PAY_WITH_COMPETITIVE".to_string(),
                    "monetization".to_string(),
                    "One-time pay with competitive multiplayer is unusual".to_string(),
                    "One-time pay games typically focus on single-player or co-op experiences".to_string(),
                ));
            }
        }
        MonetizationModel::FreeToPlay => {
            // Free-to-play games should be competitive or service-based
            if !game_dna.is_competitive && !game_dna.persistent_world && game_dna.max_players == 1 {
                result.add_warning(ValidationWarning::new(
                    "FREE_TO_PLAY_WITHOUT_COMPETITIVE".to_string(),
                    "monetization".to_string(),
                    "Free-to-play games typically need competitive or persistent elements".to_string(),
                    "Consider adding competitive multiplayer or persistent world features".to_string(),
                ));
            }
        }
        MonetizationModel::Subscription => {
            // Subscription model requires persistent worlds or MMO-scale
            if !game_dna.persistent_world && game_dna.max_players < 100 {
                result.add_warning(ValidationWarning::new(
                    "SUBSCRIPTION_WITHOUT_PERSISTENT".to_string(),
                    "monetization".to_string(),
                    "Subscription model typically requires persistent worlds or large multiplayer".to_string(),
                    "Subscription games usually have persistent worlds or MMO-scale multiplayer".to_string(),
                ));
            }
        }
        MonetizationModel::PremiumBuy => {
            // Premium buy with multiplayer should have anti-cheat
            if game_dna.max_players > 1 {
                // Check if there are any anti-cheat related tags
                if !game_dna.tags.iter().any(|tag| tag.contains("anti-cheat")) {
                    result.add_warning(ValidationWarning::new(
                        "PREMIUM_MULTIPLAYER_WITHOUT_ANTI_CHEAT".to_string(),
                        "monetization".to_string(),
                        "Premium multiplayer games should consider anti-cheat measures".to_string(),
                        "Add anti-cheat tag if your game has anti-cheat protection".to_string(),
                    ));
                }
            }
        }
        _ => {} // Other monetization models don't have specific requirements
    }
}

/// Validate performance constraints
pub fn validate_performance_constraints(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Validate target FPS is achievable on target platforms
    validate_target_fps(game_dna, result);

    // Validate max entities scales with max players
    if game_dna.max_players > 1 {
        if game_dna.supports_coop {
            // Co-op: entities <= 10x players
            if game_dna.max_entities > game_dna.max_players * 10 {
                result.add_warning(ValidationWarning::new(
                    "ENTITY_COUNT_TOO_HIGH_FOR_COOP".to_string(),
                    "max_entities".to_string(),
                    "Entity count may be too high for co-op gameplay".to_string(),
                    "Co-op games typically have entities <= 10x players".to_string(),
                ));
            }
        } else if game_dna.is_competitive {
            // Competitive: entities <= 5x players
            if game_dna.max_entities > game_dna.max_players * 5 {
                result.add_warning(ValidationWarning::new(
                    "ENTITY_COUNT_TOO_HIGH_FOR_COMPETITIVE".to_string(),
                    "max_entities".to_string(),
                    "Entity count may be too high for competitive gameplay".to_string(),
                    "Competitive games typically have entities <= 5x players".to_string(),
                ));
            }
        }
    }

    // Validate NPC count vs world scale (already handled in validate_npc_count)
}

/// Validate world simulation settings
pub fn validate_world_simulation(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Persistent world requires backend infrastructure
    if game_dna.persistent_world {
        // Check for backend infrastructure tag
        if !game_dna.tags.iter().any(|tag| tag.contains("backend") || tag.contains("server")) {
            result.add_warning(ValidationWarning::new(
                "PERSISTENT_WORLD_WITHOUT_BACKEND".to_string(),
                "persistent_world".to_string(),
                "Persistent world requires backend infrastructure".to_string(),
                "Add backend/server tags if you have the infrastructure".to_string(),
            ));
        }
    }

    // Weather and seasons are compatible
    if game_dna.weather_enabled && game_dna.seasons_enabled {
        // This is a good combination, no warnings needed
    }

    // Time scale must be > 0.0 (already handled in validate_time_scale)

    // Day/night cycle without time scale warning (already handled in validate_time_scale)
}

/// Validate AI and NPC constraints
pub fn validate_ai_npc_constraints(game_dna: &GameDNA, result: &mut ValidationResult) {
    // NPC count must be > 0 if ai_enabled = true (already handled in validate_npc_count)

    // AI difficulty scaling with static difficulty warning
    if game_dna.ai_difficulty_scaling && !matches!(game_dna.difficulty, DifficultyMode::Dynamic) {
        result.add_warning(ValidationWarning::new(
            "AI_SCALING_WITH_STATIC_DIFFICULTY".to_string(),
            "ai_difficulty_scaling".to_string(),
            "AI difficulty scaling with static difficulty is unusual".to_string(),
            "Consider using Dynamic difficulty mode with AI difficulty scaling".to_string(),
        ));
    }

    // NPC count must respect platform limits
    validate_npc_count(game_dna, result);
}

/// Validate campaign and quest logic
pub fn validate_campaign_quest_logic(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Has campaign requires narrative capabilities
    if game_dna.has_campaign {
        // Check for narrative-related tags
        if !game_dna.tags.iter().any(|tag| tag.contains("narrative") || tag.contains("story")) {
            result.add_warning(ValidationWarning::new(
                "CAMPAIGN_WITHOUT_NARRATIVE_TAG".to_string(),
                "has_campaign".to_string(),
                "Campaign mode should have narrative capabilities".to_string(),
                "Add narrative/story tags if your game has story elements".to_string(),
            ));
        }
    }

    // Dynamic quests require AI enabled
    if game_dna.dynamic_quests && !game_dna.ai_enabled {
        result.add_error(ValidationError::new(
            "DYNAMIC_QUESTS_WITHOUT_AI".to_string(),
            "dynamic_quests".to_string(),
            "Dynamic quests require AI to be enabled".to_string(),
            "Enable AI (ai_enabled: true) for dynamic quests".to_string(),
        ));
    }

    // Has side quests without has campaign warning
    if game_dna.has_side_quests && !game_dna.has_campaign {
        result.add_warning(ValidationWarning::new(
            "SIDE_QUESTS_WITHOUT_CAMPAIGN".to_string(),
            "has_side_quests".to_string(),
            "Side quests without a main campaign is unusual".to_string(),
            "Consider adding a main campaign or renaming side quests to main quests".to_string(),
        ));
    }
}