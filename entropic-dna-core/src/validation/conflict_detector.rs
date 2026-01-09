//! Conflict detection engine for Game DNA configurations
//!
//! This module provides multi-field dependency checking and circular dependency
//! detection to identify incompatible combinations in game configurations.

use crate::schema::GameDNA;
use crate::validation::{ValidationResult, ValidationError, ValidationWarning};
use std::collections::{HashMap, HashSet};

/// Conflict detector for identifying incompatible field combinations
#[derive(Debug)]
pub struct ConflictDetector;

impl ConflictDetector {
    /// Creates a new ConflictDetector.
    ///
    /// # Examples
    ///
    /// ```
    /// let _ = ConflictDetector::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Run the full set of conflict checks against a GameDNA and collect all findings.
    ///
    /// The returned ValidationResult contains accumulated validation errors and warnings
    /// produced by the detector's field, circular-dependency, and platform-specific checks.
    ///
    /// # Examples
    ///
    /// ```
    /// let detector = ConflictDetector::new();
    /// let dna = GameDNA::default();
    /// let result = detector.detect_conflicts(&dna);
    /// assert!(result.errors.len() + result.warnings.len() >= 0);
    /// ```
    pub fn detect_conflicts(&self, game_dna: &GameDNA) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        self.detect_field_conflicts(game_dna, &mut result);
        self.detect_circular_dependencies(game_dna, &mut result);
        self.detect_platform_specific_conflicts(game_dna, &mut result);
        
        result
    }

    /// Runs all field-level conflict detectors and accumulates any findings into `result`.
    ///
    /// This invokes genre-vs-camera, genre-vs-physics, scale-vs-platform, monetization-vs-gameplay,
    /// and performance conflict checks and appends discovered errors or warnings to the provided
    /// `ValidationResult`.
    ///
    /// # Examples
    ///
    /// ```
    /// let detector = ConflictDetector::new();
    /// let game_dna = GameDNA::default();
    /// let mut result = ValidationResult::new();
    /// detector.detect_field_conflicts(&game_dna, &mut result);
    /// // `result` now contains any field conflict errors or warnings detected.
    /// ```
    fn detect_field_conflicts(&self, game_dna: &GameDNA, result: &mut ValidationResult) {
        // Genre vs Camera conflicts
        self.detect_genre_camera_conflicts(game_dna, result);
        
        // Genre vs Physics conflicts
        self.detect_genre_physics_conflicts(game_dna, result);
        
        // Scale vs Platform conflicts
        self.detect_scale_platform_conflicts(game_dna, result);
        
        // Monetization vs Gameplay conflicts
        self.detect_monetization_gameplay_conflicts(game_dna, result);
        
        // Performance constraint conflicts
        self.detect_performance_conflicts(game_dna, result);
    }

    /// Checks for configuration patterns that may create circular or logical dependency loops and records a corresponding warning in `result` when found.
    ///
    /// Specifically, emits a `POTENTIAL_DIFFICULTY_LOOP` warning on the `ai_difficulty_scaling` field if AI difficulty scaling is enabled while the difficulty mode is not `Dynamic`.
    ///
    /// # Examples
    ///
    /// ```
    /// let detector = ConflictDetector::new();
    /// let game_dna = GameDNA { ai_difficulty_scaling: true, difficulty: crate::schema::DifficultyMode::Static, ..Default::default() };
    /// let mut result = ValidationResult::new();
    /// detector.detect_circular_dependencies(&game_dna, &mut result);
    /// assert!(result.warnings.iter().any(|w| w.code == "POTENTIAL_DIFFICULTY_LOOP"));
    /// ```
    fn detect_circular_dependencies(&self, game_dna: &GameDNA, result: &mut ValidationResult) {
        // This is more of a defensive check for complex configurations
        // In the current schema, circular dependencies shouldn't be possible
        // but we check for logical inconsistencies that could indicate them
        
        // Example: AI difficulty scaling depends on dynamic difficulty,
        // but if both are set in a way that creates a loop
        if game_dna.ai_difficulty_scaling && !matches!(game_dna.difficulty, crate::schema::DifficultyMode::Dynamic) {
            // This is already handled by other validation, but we'll add a specific conflict
            result.add_warning(ValidationWarning::new(
                "POTENTIAL_DIFFICULTY_LOOP".to_string(),
                "ai_difficulty_scaling".to_string(),
                "AI difficulty scaling without dynamic difficulty may cause issues".to_string(),
                "Use dynamic difficulty mode with AI difficulty scaling".to_string(),
            ));
        }
    }

    /// Checks the GameDNA for platform-specific incompatibilities and records any errors or warnings.
    ///
    /// This inspects target platforms (Mobile, XR) and validates fields such as `world_scale` and
    /// `target_fps`, adding corresponding `ValidationError` or `ValidationWarning` entries to `result`.
    ///
    /// # Parameters
    ///
    /// - `game_dna`: The configuration to validate.
    /// - `result`: Mutable accumulator for validation findings.
    ///
    /// # Examples
    ///
    /// ```
    /// let detector = ConflictDetector::new();
    /// let mut result = ValidationResult::new();
    /// let game_dna = GameDNA::default(); // construct a GameDNA suitable for your context
    /// detector.detect_platform_specific_conflicts(&game_dna, &mut result);
    /// ```
    fn detect_platform_specific_conflicts(&self, game_dna: &GameDNA, result: &mut ValidationResult) {
        use crate::schema::TargetPlatform;
        
        // Mobile platform conflicts
        if game_dna.target_platforms.contains(&TargetPlatform::Mobile) {
            // Galaxy/Planet scale on mobile
            if matches!(game_dna.world_scale, crate::schema::WorldScale::Galaxy | crate::schema::WorldScale::Planet) {
                result.add_error(ValidationError::new(
                    "PLATFORM_SCALE_CONFLICT".to_string(),
                    "world_scale".to_string(),
                    "Galaxy/Planet scale games cannot target Mobile platforms".to_string(),
                    "Remove Mobile from target platforms or reduce world scale".to_string(),
                ));
            }
            
            // Very high FPS on mobile
            if game_dna.target_fps > 120 {
                result.add_warning(ValidationWarning::new(
                    "PLATFORM_FPS_CONFLICT".to_string(),
                    "target_fps".to_string(),
                    "Very high FPS target may not be achievable on Mobile".to_string(),
                    "Consider reducing FPS target for mobile compatibility".to_string(),
                ));
            }
        }

        // XR platform conflicts
        if game_dna.target_platforms.contains(&TargetPlatform::XR) {
            // Open world or larger on XR
            match game_dna.world_scale {
                crate::schema::WorldScale::OpenWorld | 
                crate::schema::WorldScale::Planet | 
                crate::schema::WorldScale::Galaxy => {
                    result.add_error(ValidationError::new(
                        "XR_SCALE_CONFLICT".to_string(),
                        "world_scale".to_string(),
                        "XR platforms cannot support OpenWorld or larger scales".to_string(),
                        "Use LargeLevel or smaller for XR games".to_string(),
                    ));
                }
                _ => {}
            }
        }
    }

    /// Checks for mismatches between the game's genre and its configured camera mode and records any
    /// resulting errors or warnings in `result`.
    ///
    /// - Emits an error `GENRE_CAMERA_CONFLICT` on the `camera` field when FPS/TPS genres use a camera
    ///   mode other than `Perspective3D` or `VR`.
    /// - Emits a warning `PUZZLE_3D_CAMERA` on the `camera` field when a Puzzle game uses a full 3D
    ///   `Perspective3D` camera but the game's tags do not indicate 3D content.
    ///
    /// # Examples
    ///
    /// ```
    /// let detector = ConflictDetector::new();
    /// let game_dna = GameDNA {
    ///     genre: Genre::FPS,
    ///     camera: CameraMode::Orthographic,
    ///     ..Default::default()
    /// };
    /// let mut result = ValidationResult::new();
    /// detector.detect_genre_camera_conflicts(&game_dna, &mut result);
    /// // result will contain a GENRE_CAMERA_CONFLICT error for the camera field
    /// ```
    fn detect_genre_camera_conflicts(&self, game_dna: &GameDNA, result: &mut ValidationResult) {
        use crate::schema::{Genre, CameraMode};
        
        match game_dna.genre {
            Genre::FPS | Genre::TPS => {
                // FPS/TPS require 3D camera
                if !matches!(game_dna.camera, CameraMode::Perspective3D | CameraMode::VR) {
                    result.add_error(ValidationError::new(
                        "GENRE_CAMERA_CONFLICT".to_string(),
                        "camera".to_string(),
                        format!("{} games require 3D camera, not {:?}", 
                            if matches!(game_dna.genre, Genre::FPS) { "FPS" } else { "TPS" },
                            game_dna.camera),
                        "Use Perspective3D or VR camera for shooter games".to_string(),
                    ));
                }
            }
            Genre::Puzzle => {
                // 2D puzzles shouldn't use full 3D cameras
                if matches!(game_dna.camera, CameraMode::Perspective3D) && !game_dna.tags.iter().any(|t| t.contains("3d")) {
                    result.add_warning(ValidationWarning::new(
                        "PUZZLE_3D_CAMERA".to_string(),
                        "camera".to_string(),
                        "2D puzzle game using 3D camera is unusual".to_string(),
                        "Consider using Perspective2D or Perspective2_5D for 2D puzzles".to_string(),
                    ));
                }
            }
            _ => {}
        }
    }

    /// Checks the GameDNA for genre-to-physics mismatches and records warnings on the provided ValidationResult.
    ///
    /// This inspects the game's genre and physics profile and appends warnings to `result` when the combination
    /// is likely to produce a poor player experience (for example, arcade physics for genres that expect
    /// more realistic simulation).
    ///
    /// Parameters:
    /// - `game_dna`: the configuration to validate.
    /// - `result`: mutable accumulator where warnings will be added.
    ///
    /// # Examples
    ///
    /// ```
    /// let detector = ConflictDetector::new();
    /// let game_dna = GameDNA { genre: Genre::Racing, physics_profile: PhysicsProfile::Arcade, ..Default::default() };
    /// let mut result = ValidationResult::new();
    /// detector.detect_genre_physics_conflicts(&game_dna, &mut result);
    /// assert!(result.warnings.iter().any(|w| w.code == "RACING_ARCADE_PHYSICS"));
    /// ```
    fn detect_genre_physics_conflicts(&self, game_dna: &GameDNA, result: &mut ValidationResult) {
        use crate::schema::{Genre, PhysicsProfile};
        
        match game_dna.genre {
            Genre::Racing => {
                // Racing games with arcade physics is unusual but allowed
                if game_dna.physics_profile == PhysicsProfile::Arcade {
                    result.add_warning(ValidationWarning::new(
                        "RACING_ARCADE_PHYSICS".to_string(),
                        "physics_profile".to_string(),
                        "Racing game with Arcade physics may feel less realistic".to_string(),
                        "Consider using Realistic physics for racing simulations".to_string(),
                    ));
                }
            }
            Genre::RPG | Genre::Horror => {
                // RPGs and Horror games should use SemiRealistic or Realistic
                if game_dna.physics_profile == PhysicsProfile::Arcade {
                    result.add_warning(ValidationWarning::new(
                        "RPG_HORROR_ARCADE_PHYSICS".to_string(),
                        "physics_profile".to_string(),
                        "RPG/Horror games with Arcade physics may feel less immersive".to_string(),
                        "Consider using SemiRealistic or Realistic physics".to_string(),
                    ));
                }
            }
            _ => {}
        }
    }

    /// Validate that the configured world scale is compatible with the target platforms.
    ///
    /// Emits errors when very large world scales (Galaxy or Planet) are targeted at Mobile or XR,
    /// and emits a warning when an OpenWorld scale targets Mobile.
    ///
    /// # Arguments
    ///
    /// * `game_dna` - The GameDNA being validated; inspected for `world_scale` and `target_platforms`.
    /// * `result` - Accumulates any `ValidationError` or `ValidationWarning` produced by the check.
    ///
    /// # Examples
    ///
    /// ```
    /// let detector = ConflictDetector::new();
    /// let game_dna = /* build GameDNA with world_scale = WorldScale::OpenWorld and TargetPlatform::Mobile */ ;
    /// let mut result = ValidationResult::new();
    /// detector.detect_scale_platform_conflicts(&game_dna, &mut result);
    /// // result will contain a warning for OpenWorld on Mobile or errors for Galaxy/Planet on Mobile/XR
    /// ```
    fn detect_scale_platform_conflicts(&self, game_dna: &GameDNA, result: &mut ValidationResult) {
        use crate::schema::{WorldScale, TargetPlatform};
        
        match game_dna.world_scale {
            WorldScale::Galaxy | WorldScale::Planet => {
                // These scales are not supported on mobile or XR
                for platform in &game_dna.target_platforms {
                    match platform {
                        TargetPlatform::Mobile | TargetPlatform::XR => {
                            result.add_error(ValidationError::new(
                                "SCALE_PLATFORM_CONFLICT".to_string(),
                                "target_platforms".to_string(),
                                format!("{:?} scale is not supported on {:?}", game_dna.world_scale, platform),
                                "Remove unsupported platforms or reduce world scale".to_string(),
                            ));
                        }
                        _ => {}
                    }
                }
            }
            WorldScale::OpenWorld => {
                // Open world is challenging on mobile
                if game_dna.target_platforms.contains(&TargetPlatform::Mobile) {
                    result.add_warning(ValidationWarning::new(
                        "OPEN_WORLD_MOBILE".to_string(),
                        "world_scale".to_string(),
                        "OpenWorld games on Mobile require significant optimization".to_string(),
                        "Consider using LargeLevel or smaller for mobile games".to_string(),
                    ));
                }
            }
            _ => {}
        }
    }

    /// Checks monetization and gameplay settings and emits warnings for configurations that are likely
    /// incompatible with the chosen monetization model.
    ///
    /// Specifically:
    /// - For Free-to-Play, adds a `F2P_SINGLE_PLAYER` warning on `"monetization"` when the game is
    ///   single-player (max_players == 1), not a persistent world, and not competitive.
    /// - For Subscription, adds a `SUBSCRIPTION_NO_PERSISTENT` warning on `"monetization"` when the
    ///   game is not a persistent world and supports fewer than 10 players.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Illustrative example; types are from the crate and omitted for brevity.
    /// let detector = ConflictDetector::new();
    /// let game_dna = GameDNA {
    ///     monetization: MonetizationModel::FreeToPlay,
    ///     max_players: 1,
    ///     persistent_world: false,
    ///     is_competitive: false,
    ///     ..Default::default()
    /// };
    /// let mut result = ValidationResult::new();
    /// detector.detect_monetization_gameplay_conflicts(&game_dna, &mut result);
    /// // result.warnings will include a F2P_SINGLE_PLAYER entry for "monetization"
    /// ```
    fn detect_monetization_gameplay_conflicts(&self, game_dna: &GameDNA, result: &mut ValidationResult) {
        use crate::schema::MonetizationModel;
        
        match game_dna.monetization {
            MonetizationModel::FreeToPlay => {
                // Free-to-play should have some form of multiplayer or persistent content
                if game_dna.max_players == 1 && !game_dna.persistent_world && !game_dna.is_competitive {
                    result.add_warning(ValidationWarning::new(
                        "F2P_SINGLE_PLAYER".to_string(),
                        "monetization".to_string(),
                        "Free-to-play single player games are challenging to monetize".to_string(),
                        "Consider adding multiplayer, competitive, or persistent world elements".to_string(),
                    ));
                }
            }
            MonetizationModel::Subscription => {
                // Subscription should have persistent or multiplayer content
                if !game_dna.persistent_world && game_dna.max_players < 10 {
                    result.add_warning(ValidationWarning::new(
                        "SUBSCRIPTION_NO_PERSISTENT".to_string(),
                        "monetization".to_string(),
                        "Subscription model works best with persistent worlds or large multiplayer".to_string(),
                        "Consider adding persistent world or increasing multiplayer support".to_string(),
                    ));
                }
            }
            _ => {}
        }
    }

    /// Checks the GameDNA for performance-related configuration issues and appends corresponding warnings to the provided ValidationResult.
    ///
    /// This inspects entity counts, NPC counts in combination with physics profile, and world scale with draw distance to detect common performance risks and recommends mitigations.
    ///
    /// # Examples
    ///
    /// ```
    /// let detector = ConflictDetector::new();
    /// let game_dna = GameDNA {
    ///     max_entities: 20_000,
    ///     target_fps: 90,
    ///     max_npc_count: 1_500,
    ///     physics_profile: crate::schema::PhysicsProfile::Realistic,
    ///     world_scale: crate::schema::WorldScale::OpenWorld,
    ///     max_draw_distance: 2500.0,
    ///     ..Default::default()
    /// };
    /// let mut result = ValidationResult::new();
    /// detector.detect_performance_conflicts(&game_dna, &mut result);
    /// assert!(!result.warnings.is_empty());
    /// ```
    fn detect_performance_conflicts(&self, game_dna: &GameDNA, result: &mut ValidationResult) {
        // High entity count with high FPS target
        if game_dna.max_entities > 10000 && game_dna.target_fps > 60 {
            result.add_warning(ValidationWarning::new(
                "HIGH_ENTITIES_HIGH_FPS".to_string(),
                "performance".to_string(),
                "High entity count with high FPS target may cause performance issues".to_string(),
                "Consider reducing entity count or FPS target".to_string(),
            ));
        }

        // Very high NPC count with complex physics
        if game_dna.max_npc_count > 1000 && matches!(game_dna.physics_profile, crate::schema::PhysicsProfile::Realistic) {
            result.add_warning(ValidationWarning::new(
                "HIGH_NPC_REALISTIC_PHYSICS".to_string(),
                "performance".to_string(),
                "High NPC count with Realistic physics may cause performance issues".to_string(),
                "Consider reducing NPC count or using SemiRealistic physics".to_string(),
            ));
        }

        // Large world with high draw distance
        match game_dna.world_scale {
            crate::schema::WorldScale::LargeLevel | crate::schema::WorldScale::OpenWorld => {
                if game_dna.max_draw_distance > 2000.0 {
                    result.add_warning(ValidationWarning::new(
                        "LARGE_WORLD_HIGH_DRAW_DISTANCE".to_string(),
                        "performance".to_string(),
                        "Large world with high draw distance may cause performance issues".to_string(),
                        "Consider reducing draw distance for better performance".to_string(),
                    ));
                }
            }
            _ => {}
        }
    }

    /// Aggregate suggested fixes from a ValidationResult into a map keyed by field.
    ///
    /// Each map entry maps a field name to a list of suggested fixes: for each error the error's `details`
    /// string is added, and for each warning the warning's `suggestion` string is added.
    ///
    /// # Examples
    ///
    /// ```
    /// let detector = ConflictDetector::new();
    /// let result = ValidationResult::default(); // contains collected errors and warnings
    /// let fixes = detector.suggest_fixes(&result);
    /// // `fixes` is a HashMap<String, Vec<String>> mapping field names to suggested fixes
    /// ```
    pub fn suggest_fixes(&self, result: &ValidationResult) -> HashMap<String, Vec<String>> {
        let mut suggestions = HashMap::new();
        
        for error in &result.errors {
            let field_suggestions = suggestions.entry(error.field.clone()).or_insert_with(Vec::new);
            field_suggestions.push(error.details.clone());
        }
        
        for warning in &result.warnings {
            let field_suggestions = suggestions.entry(warning.field.clone()).or_insert_with(Vec::new);
            field_suggestions.push(warning.suggestion.clone());
        }
        
        suggestions
    }
}