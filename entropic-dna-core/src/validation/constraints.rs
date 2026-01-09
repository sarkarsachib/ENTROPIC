//! Constraint checking for Game DNA configurations
//!
//! This module provides validation for genre ↔ camera ↔ physics compatibility
//! and other multi-field constraints that require cross-field validation.

use crate::schema::{GameDNA, Genre, CameraMode, PhysicsProfile, TargetPlatform, WorldScale, MonetizationModel};
use crate::validation::{ValidationResult, ValidationError, ValidationWarning};

/// Validate all constraints for a GameDNA configuration
pub fn validate_all_constraints(game_dna: &GameDNA, result: &mut ValidationResult) {
    validate_genre_camera_physics_constraints(game_dna, result);
    validate_platform_specific_constraints(game_dna, result);
    validate_performance_budget_constraints(game_dna, result);
    validate_logical_consistency_constraints(game_dna, result);
}

/// Validate genre, camera, and physics constraints together
pub fn validate_genre_camera_physics_constraints(game_dna: &GameDNA, result: &mut ValidationResult) {
    // FPS/TPS with 3D camera and realistic physics is a good combination
    if matches!(game_dna.genre, Genre::FPS | Genre::TPS) {
        if matches!(game_dna.camera, CameraMode::Perspective3D | CameraMode::VR) {
            if matches!(game_dna.physics_profile, PhysicsProfile::SemiRealistic | PhysicsProfile::Realistic) {
                // This is a good combination, no warnings
            } else {
                result.add_warning(ValidationWarning::new(
                    "SHOOTER_WITH_ARCADE_PHYSICS".to_string(),
                    "physics_profile".to_string(),
                    "Shooter game with Arcade physics may feel less immersive".to_string(),
                    "Consider using SemiRealistic or Realistic physics for better immersion".to_string(),
                ));
            }
        }
    }

    // Strategy games with isometric camera and arcade physics
    if matches!(game_dna.genre, Genre::Strategy) {
        if matches!(game_dna.camera, CameraMode::Isometric) {
            if game_dna.physics_profile == PhysicsProfile::Arcade {
                // This is a good combination for RTS games
            }
        }
    }

    // Racing games with 3D camera and realistic physics
    if matches!(game_dna.genre, Genre::Racing) {
        if matches!(game_dna.camera, CameraMode::Perspective3D) {
            if game_dna.physics_profile == PhysicsProfile::Realistic {
                // This is a good combination for realistic racing
            }
        }
    }
}

/// Validate platform-specific constraints
pub fn validate_platform_specific_constraints(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Mobile platform constraints
    if game_dna.target_platforms.contains(&TargetPlatform::Mobile) {
        // Mobile games should have reasonable performance targets
        if game_dna.target_fps > 120 {
            result.add_warning(ValidationWarning::new(
                "MOBILE_HIGH_FPS".to_string(),
                "target_fps".to_string(),
                "High FPS target for mobile may not be achievable on all devices".to_string(),
                "Consider targeting 60 FPS for broad mobile compatibility".to_string(),
            ));
        }

        // Mobile games should have reasonable entity counts
        if game_dna.max_entities > 5000 {
            result.add_warning(ValidationWarning::new(
                "MOBILE_HIGH_ENTITY_COUNT".to_string(),
                "max_entities".to_string(),
                "High entity count may cause performance issues on mobile".to_string(),
                "Consider reducing entity count for better mobile performance".to_string(),
            ));
        }

        // Mobile games should have reasonable NPC counts
        if game_dna.max_npc_count > 500 {
            result.add_warning(ValidationWarning::new(
                "MOBILE_HIGH_NPC_COUNT".to_string(),
                "max_npc_count".to_string(),
                "High NPC count may cause performance issues on mobile".to_string(),
                "Consider reducing NPC count for better mobile performance".to_string(),
            ));
        }
    }

    // Console platform constraints
    if game_dna.target_platforms.contains(&TargetPlatform::Console) {
        // Console games should target reasonable FPS
        if game_dna.target_fps > 120 {
            result.add_warning(ValidationWarning::new(
                "CONSOLE_HIGH_FPS".to_string(),
                "target_fps".to_string(),
                "High FPS target may not be achievable on all console hardware".to_string(),
                "Consider targeting 60 FPS for broad console compatibility".to_string(),
            ));
        }
    }

    // XR platform constraints
    if game_dna.target_platforms.contains(&TargetPlatform::XR) {
        // XR games should have reasonable performance targets
        if game_dna.target_fps > 90 {
            result.add_warning(ValidationWarning::new(
                "XR_HIGH_FPS".to_string(),
                "target_fps".to_string(),
                "Very high FPS target for XR may cause performance issues".to_string(),
                "Consider targeting 72-90 FPS for XR compatibility".to_string(),
            ));
        }

        // XR games should have reasonable entity counts
        if game_dna.max_entities > 10000 {
            result.add_warning(ValidationWarning::new(
                "XR_HIGH_ENTITY_COUNT".to_string(),
                "max_entities".to_string(),
                "High entity count may cause performance issues in XR".to_string(),
                "Consider reducing entity count for better XR performance".to_string(),
            ));
        }
    }
}

/// Validate performance budget constraints
pub fn validate_performance_budget_constraints(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Calculate rough performance budget based on platform and scale
    let mut performance_budget = 0;
    
    for platform in &game_dna.target_platforms {
        match platform {
            TargetPlatform::Mobile => performance_budget += 1000, // Low budget
            TargetPlatform::Console => performance_budget += 5000, // Medium budget
            TargetPlatform::PC => performance_budget += 10000, // High budget
            TargetPlatform::XR => performance_budget += 3000, // Medium-low budget
            _ => performance_budget += 5000, // Default medium budget
        }
    }

    // Adjust budget based on world scale
    match game_dna.world_scale {
        WorldScale::TinyLevel => performance_budget *= 2, // Small worlds are easier
        WorldScale::SmallLevel => performance_budget *= 1, // No change
        WorldScale::MediumLevel => performance_budget = (performance_budget as f32 * 0.8) as u32, // 20% reduction
        WorldScale::LargeLevel => performance_budget = (performance_budget as f32 * 0.6) as u32, // 40% reduction
        WorldScale::OpenWorld => performance_budget = (performance_budget as f32 * 0.4) as u32, // 60% reduction
        WorldScale::Planet => performance_budget = (performance_budget as f32 * 0.2) as u32, // 80% reduction
        WorldScale::Galaxy => performance_budget = (performance_budget as f32 * 0.1) as u32, // 90% reduction
        _ => {} // No change for custom scales
    }

    // Calculate total performance cost
    let entity_cost = game_dna.max_entities / 100; // Each 100 entities = 1 unit
    let npc_cost = game_dna.max_npc_count / 50; // Each 50 NPCs = 1 unit
    let player_cost = game_dna.max_players * 10; // Each player = 10 units
    let fps_cost = game_dna.target_fps / 10; // Each 10 FPS = 1 unit
    let physics_cost = match game_dna.physics_profile {
        PhysicsProfile::Arcade => 10,
        PhysicsProfile::SemiRealistic => 30,
        PhysicsProfile::Realistic => 50,
        _ => 20,
    };

    let total_cost = entity_cost + npc_cost + player_cost + fps_cost + physics_cost;

    // Check if we're within budget
    if total_cost > performance_budget {
        let over_budget_pct = ((total_cost - performance_budget) as f32 / performance_budget as f32) * 100.0;
        result.add_warning(ValidationWarning::new(
            "PERFORMANCE_BUDGET_EXCEEDED".to_string(),
            "performance".to_string(),
            format!("Performance budget exceeded by {:.1}%", over_budget_pct),
            "Consider reducing entity/NPC counts, lowering FPS target, or simplifying physics".to_string(),
        ));
    }
}

/// Validate logical consistency constraints
pub fn validate_logical_consistency_constraints(game_dna: &GameDNA, result: &mut ValidationResult) {
    // Check for contradictory settings
    
    // Competitive game with single player only
    if game_dna.is_competitive && game_dna.max_players == 1 {
        result.add_warning(ValidationWarning::new(
            "COMPETITIVE_SINGLE_PLAYER".to_string(),
            "is_competitive".to_string(),
            "Game is marked as competitive but only supports single player".to_string(),
            "Set max_players > 1 for competitive multiplayer or set is_competitive to false".to_string(),
        ));
    }

    // Co-op game with single player only
    if game_dna.supports_coop && game_dna.max_players == 1 {
        result.add_warning(ValidationWarning::new(
            "COOP_SINGLE_PLAYER".to_string(),
            "supports_coop".to_string(),
            "Game supports co-op but only allows single player".to_string(),
            "Set max_players > 1 for co-op gameplay".to_string(),
        ));
    }

    // High player count but no competitive or co-op flags
    if game_dna.max_players > 4 && !game_dna.is_competitive && !game_dna.supports_coop {
        result.add_warning(ValidationWarning::new(
            "HIGH_PLAYER_COUNT_NO_MULTIPLAYER".to_string(),
            "max_players".to_string(),
            "High player count but no competitive or co-op flags set".to_string(),
            "Set is_competitive or supports_coop to true for multiplayer games".to_string(),
        ));
    }

    // Persistent world but no multiplayer
    if game_dna.persistent_world && game_dna.max_players == 1 {
        result.add_warning(ValidationWarning::new(
            "PERSISTENT_WORLD_SINGLE_PLAYER".to_string(),
            "persistent_world".to_string(),
            "Persistent world with single player only is unusual".to_string(),
            "Persistent worlds are typically used for multiplayer or online games".to_string(),
        ));
    }

    // Free-to-play but no monetization strategy indicated
    if matches!(game_dna.monetization, MonetizationModel::FreeToPlay) {
        if !game_dna.tags.iter().any(|tag| 
            tag.contains("ads") || 
            tag.contains("iap") || 
            tag.contains("in-app") || 
            tag.contains("microtransaction") ||
            tag.contains("battle-pass")
        ) {
            result.add_warning(ValidationWarning::new(
                "FREE_TO_PLAY_NO_MONETIZATION_STRATEGY".to_string(),
                "monetization".to_string(),
                "Free-to-play game has no obvious monetization strategy".to_string(),
                "Add tags indicating your monetization strategy (ads, IAP, battle-pass, etc.)".to_string(),
            ));
        }
    }

    // Premium game with high player count but no anti-cheat
    if matches!(game_dna.monetization, MonetizationModel::PremiumBuy | MonetizationModel::OneTimePay) {
        if game_dna.max_players > 4 && !game_dna.tags.iter().any(|tag| tag.contains("anti-cheat")) {
            result.add_warning(ValidationWarning::new(
                "PREMIUM_MULTIPLAYER_NO_ANTI_CHEAT".to_string(),
                "monetization".to_string(),
                "Premium multiplayer game should consider anti-cheat measures".to_string(),
                "Add anti-cheat tag if your game has anti-cheat protection".to_string(),
            ));
        }
    }
}