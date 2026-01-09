//! Comprehensive validation engine for Game DNA configurations
//!
//! This module provides a robust validation layer that ensures all game configurations
//! are internally consistent, compatible, and deterministic before being locked and
//! distributed downstream.

pub mod rules;
pub mod constraints;
pub mod conflict_detector;
pub mod checksum;

use crate::schema::GameDNA;
use chrono::DateTime;
use chrono::Utc;
use std::collections::HashMap;

/// Validation result containing errors, warnings, and suggestions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    /// Whether the configuration is valid
    pub is_valid: bool,
    /// List of validation errors
    pub errors: Vec<ValidationError>,
    /// List of validation warnings
    pub warnings: Vec<ValidationWarning>,
    /// List of suggestions for improvement
    pub suggestions: Vec<String>,
}

impl ValidationResult {
    /// Create a new validation result
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Add an error to the validation result
    pub fn add_error(&mut self, error: ValidationError) {
        self.is_valid = false;
        self.errors.push(error);
    }

    /// Add a warning to the validation result
    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }

    /// Add a suggestion to the validation result
    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    /// Merge another validation result into this one
    pub fn merge(&mut self, other: ValidationResult) {
        if !other.is_valid {
            self.is_valid = false;
        }
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self.suggestions.extend(other.suggestions);
    }
}

/// Validation error with detailed information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Error code (e.g., "INCOMPATIBLE_CAMERA_FOR_GENRE")
    pub code: String,
    /// Field that caused the error
    pub field: String,
    /// Error message
    pub message: String,
    /// Detailed explanation
    pub details: String,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(code: String, field: String, message: String, details: String) -> Self {
        Self {
            code,
            field,
            message,
            details,
        }
    }
}

/// Validation warning with suggested fixes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationWarning {
    /// Warning code
    pub code: String,
    /// Field that triggered the warning
    pub field: String,
    /// Warning message
    pub message: String,
    /// Suggestion for fixing the issue
    pub suggestion: String,
}

impl ValidationWarning {
    /// Create a new validation warning
    pub fn new(code: String, field: String, message: String, suggestion: String) -> Self {
        Self {
            code,
            field,
            message,
            suggestion,
        }
    }
}

/// Main validation engine
#[derive(Debug)]
pub struct ValidationEngine;

impl ValidationEngine {
    /// Create a new validation engine
    pub fn new() -> Self {
        Self
    }

    /// Validate a GameDNA configuration
    pub fn validate(&self, game_dna: &GameDNA) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Apply all validation rules
        rules::validate_basic_fields(game_dna, &mut result);
        rules::validate_genre_camera_compatibility(game_dna, &mut result);
        rules::validate_genre_physics_compatibility(game_dna, &mut result);
        rules::validate_tone_gameplay_combinations(game_dna, &mut result);
        rules::validate_scale_platform_compatibility(game_dna, &mut result);
        rules::validate_monetization_gameplay(game_dna, &mut result);
        rules::validate_performance_constraints(game_dna, &mut result);
        rules::validate_world_simulation(game_dna, &mut result);
        rules::validate_ai_npc_constraints(game_dna, &mut result);
        rules::validate_campaign_quest_logic(game_dna, &mut result);

        // Check constraints
        constraints::validate_all_constraints(game_dna, &mut result);

        result
    }

    /// Validate a specific field
    pub fn validate_field(&self, game_dna: &GameDNA, field: &str) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        match field {
            "name" => rules::validate_name(game_dna, &mut result),
            "genre" => rules::validate_genre(game_dna, &mut result),
            "camera" => rules::validate_camera(game_dna, &mut result),
            "tone" => rules::validate_tone(game_dna, &mut result),
            "world_scale" => rules::validate_world_scale(game_dna, &mut result),
            "target_platforms" => rules::validate_target_platforms(game_dna, &mut result),
            "physics_profile" => rules::validate_physics_profile(game_dna, &mut result),
            "max_players" => rules::validate_max_players(game_dna, &mut result),
            "target_fps" => rules::validate_target_fps(game_dna, &mut result),
            "time_scale" => rules::validate_time_scale(game_dna, &mut result),
            "npc_count" => rules::validate_npc_count(game_dna, &mut result),
            _ => result.add_warning(ValidationWarning::new(
                "UNKNOWN_FIELD".to_string(),
                field.to_string(),
                format!("Unknown field: {}", field),
                "Check the field name and try again.".to_string(),
            )),
        }

        result
    }
}

/// Builder pattern for GameDNA with validation support
#[derive(Debug)]
pub struct ValidatedGameDNABuilder {
    game_dna: GameDNA,
    validation_result: ValidationResult,
}

impl ValidatedGameDNABuilder {
    /// Create a new validated builder from a GameDNA
    pub fn new(game_dna: GameDNA) -> Self {
        let validation_result = ValidationEngine::new().validate(&game_dna);
        Self {
            game_dna,
            validation_result,
        }
    }

    /// Get the current validation result
    pub fn validation_result(&self) -> &ValidationResult {
        &self.validation_result
    }

    /// Validate all fields and return the result
    pub fn validate_all(&mut self) -> &ValidationResult {
        self.validation_result = ValidationEngine::new().validate(&self.game_dna);
        &self.validation_result
    }

    /// Validate a specific field
    pub fn validate_field(&mut self, field: &str) -> &ValidationResult {
        let field_result = ValidationEngine::new().validate_field(&self.game_dna, field);
        self.validation_result.merge(field_result);
        &self.validation_result
    }

    /// Check if the configuration is valid
    pub fn is_valid(&self) -> bool {
        self.validation_result.is_valid
    }

    /// Get the current GameDNA
    pub fn game_dna(&self) -> &GameDNA {
        &self.game_dna
    }

    /// Consume the builder and return the GameDNA (if valid)
    pub fn build(self) -> Result<GameDNA, ValidationResult> {
        if self.validation_result.is_valid {
            Ok(self.game_dna)
        } else {
            Err(self.validation_result)
        }
    }
}

/// Locked GameDNA configuration with checksum and immutability
#[derive(Debug, Clone)]
pub struct LockedGameDNA {
    /// The locked GameDNA configuration
    pub config: GameDNA,
    /// SHA-256 checksum of the configuration
    pub checksum: String,
    /// Timestamp when the configuration was locked
    pub lock_timestamp: DateTime<Utc>,
    /// Whether the configuration is locked (immutable)
    pub is_locked: bool,
}

impl LockedGameDNA {
    /// Create a new locked GameDNA from a validated configuration
    pub fn new(config: GameDNA) -> Self {
        let checksum = checksum::generate_checksum(&config);
        let lock_timestamp = Utc::now();
        
        Self {
            config,
            checksum,
            lock_timestamp,
            is_locked: true,
        }
    }

    /// Verify the integrity of the locked configuration
    pub fn verify_integrity(&self) -> bool {
        if !self.is_locked {
            return false;
        }
        
        let current_checksum = checksum::generate_checksum(&self.config);
        current_checksum == self.checksum
    }

    /// Unlock the configuration (requires explicit action)
    pub fn unlock(&mut self) {
        self.is_locked = false;
    }

    /// Get the configuration (only if unlocked)
    pub fn get_config(&self) -> Option<&GameDNA> {
        if self.is_locked {
            None
        } else {
            Some(&self.config)
        }
    }

    /// Get the configuration as a mutable reference (only if unlocked)
    pub fn get_config_mut(&mut self) -> Option<&mut GameDNA> {
        if self.is_locked {
            None
        } else {
            Some(&mut self.config)
        }
    }
}

/// Builder pattern for creating locked GameDNA configurations
#[derive(Debug)]
pub struct LockedGameDNABuilder {
    game_dna: GameDNA,
    validation_engine: ValidationEngine,
}

impl LockedGameDNABuilder {
    /// Create a new locked builder from a GameDNA
    pub fn new(game_dna: GameDNA) -> Self {
        Self {
            game_dna,
            validation_engine: ValidationEngine::new(),
        }
    }

    /// Validate the current configuration
    pub fn validate(&self) -> ValidationResult {
        self.validation_engine.validate(&self.game_dna)
    }

    /// Validate a specific field
    pub fn validate_field(&self, field: &str) -> ValidationResult {
        self.validation_engine.validate_field(&self.game_dna, field)
    }

    /// Publish the configuration as a locked GameDNA
    /// Returns Ok if validation passes, Err with validation result if it fails
    pub fn publish(self) -> Result<LockedGameDNA, ValidationResult> {
        let validation_result = self.validation_engine.validate(&self.game_dna);
        
        if validation_result.is_valid {
            Ok(LockedGameDNA::new(self.game_dna))
        } else {
            Err(validation_result)
        }
    }
}