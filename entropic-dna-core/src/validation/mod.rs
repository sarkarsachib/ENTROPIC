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
    /// Creates a default, valid ValidationResult with no errors, warnings, or suggestions.
    ///
    /// # Examples
    ///
    /// ```
    /// let res = ValidationResult::new();
    /// assert!(res.is_valid);
    /// assert!(res.errors.is_empty());
    /// assert!(res.warnings.is_empty());
    /// assert!(res.suggestions.is_empty());
    /// ```
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

    /// Appends a validation warning to the result without changing overall validity.
    ///
    /// This pushes the provided `ValidationWarning` onto the internal warnings list and does not modify `is_valid`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut res = ValidationResult::new();
    /// res.add_warning(ValidationWarning::new(
    ///     "W001".into(),
    ///     "name".into(),
    ///     "Deprecated field".into(),
    ///     "Consider removing this field".into(),
    /// ));
    /// assert_eq!(res.warnings.len(), 1);
    /// assert!(res.is_valid);
    /// ```
    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }

    /// Appends a human-readable suggestion to the validation result's suggestions list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut res = ValidationResult::new();
    /// res.add_suggestion("Consider reducing NPC count to improve performance.".into());
    /// assert_eq!(res.suggestions.len(), 1);
    /// assert_eq!(res.suggestions[0], "Consider reducing NPC count to improve performance.");
    /// ```
    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }

    /// Merge another ValidationResult into this one, combining errors, warnings, and suggestions.
    ///
    /// If the other result is invalid, this result becomes invalid. All errors, warnings,
    /// and suggestions from `other` are appended to this result.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut a = ValidationResult::new();
    /// let mut b = ValidationResult::new();
    /// b.add_error(ValidationError::new("E1".into(), "field".into(), "msg".into(), "".into()));
    /// a.merge(b);
    /// assert!(!a.is_valid);
    /// assert_eq!(a.errors.len(), 1);
    /// ```
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
    /// Creates a ValidationError with the given components.
    ///
    /// # Parameters
    /// - `code`: A short, machine-readable error code identifying the error type.
    /// - `field`: The name of the field associated with the error.
    /// - `message`: A human-readable error message suitable for display or logs.
    /// - `details`: Additional diagnostic details useful for debugging.
    ///
    /// # Examples
    ///
    /// ```
    /// let err = ValidationError::new(
    ///     "MISSING_FIELD".to_string(),
    ///     "name".to_string(),
    ///     "Name is required".to_string(),
    ///     "field `name` was empty in payload".to_string(),
    /// );
    /// assert_eq!(err.code, "MISSING_FIELD");
    /// assert_eq!(err.field, "name");
    /// ```
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
    /// Constructs a ValidationWarning with the provided code, field, message, and suggestion.
    ///
    /// # Examples
    ///
    /// ```
    /// let w = ValidationWarning::new(
    ///     "MISSING_ICON".to_string(),
    ///     "ui.icon".to_string(),
    ///     "Icon is missing".to_string(),
    ///     "Add a 64x64 PNG icon".to_string(),
    /// );
    /// assert_eq!(w.code, "MISSING_ICON");
    /// assert_eq!(w.field, "ui.icon");
    /// assert_eq!(w.message, "Icon is missing");
    /// assert_eq!(w.suggestion, "Add a 64x64 PNG icon");
    /// ```
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
    /// Creates a ValidationEngine instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let engine = ValidationEngine::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Validates an entire GameDNA configuration and returns an aggregated ValidationResult.
    ///
    /// The returned ValidationResult contains accumulated errors, warnings, and suggestions
    /// produced by applying the full set of validation rules and constraints.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::validation::ValidationEngine;
    /// use crate::schema::GameDNA;
    ///
    /// let engine = ValidationEngine::new();
    /// // construct or load a GameDNA instance named `game_dna` here
    /// let game_dna: GameDNA = /* ... */ unimplemented!();
    /// let result = engine.validate(&game_dna);
    /// // inspect result.is_valid, result.errors, result.warnings, result.suggestions
    /// ```
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

    /// Validate a single GameDNA field and produce a field-scoped ValidationResult.
    ///
    /// Performs validation only for the named field and returns a ValidationResult containing
    /// any errors, warnings, and suggestions that apply to that field. If the provided field
    /// name is not recognized, the result will include a warning with code `"UNKNOWN_FIELD"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::validation::ValidationEngine;
    /// use crate::schema::GameDNA;
    ///
    /// let game_dna = GameDNA::default();
    /// let engine = ValidationEngine::new();
    /// let result = engine.validate_field(&game_dna, "name");
    /// // `result` now contains errors/warnings relevant to the `name` field (or an UNKNOWN_FIELD warning).
    /// ```
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
    /// Initializes a ValidatedGameDNABuilder and immediately runs validation on the provided GameDNA.
    ///
    /// The returned builder contains the original GameDNA and its initial ValidationResult produced by the ValidationEngine.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::schema::GameDNA;
    /// # use crate::validation::ValidatedGameDNABuilder;
    /// let game = GameDNA::default();
    /// let builder = ValidatedGameDNABuilder::new(game);
    /// let _result = builder.validation_result();
    /// ```
    pub fn new(game_dna: GameDNA) -> Self {
        let validation_result = ValidationEngine::new().validate(&game_dna);
        Self {
            game_dna,
            validation_result,
        }
    }

    /// Access the builder's current validation result.
    ///
    /// # Returns
    ///
    /// A reference to the `ValidationResult` that holds the builder's current validity state, errors, warnings, and suggestions.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = ValidatedGameDNABuilder::new(GameDNA::default());
    /// let result = builder.validation_result();
    /// // inspect validity or diagnostics
    /// let _ = result.is_valid;
    /// ```
    pub fn validation_result(&self) -> &ValidationResult {
        &self.validation_result
    }

    /// Revalidates the builder's GameDNA and updates its stored ValidationResult.
    ///
    /// Replaces the builder's internal ValidationResult with the result of a full validation
    /// run over the current `GameDNA`, then returns a reference to the updated result.
    ///
    /// # Examples
    ///
    /// ```
    /// let game = GameDNA::default();
    /// let mut builder = ValidatedGameDNABuilder::new(game);
    /// let result = builder.validate_all();
    /// // returned reference points to the builder's stored validation result
    /// assert!(std::ptr::eq(result, builder.validation_result()));
    /// ```
    pub fn validate_all(&mut self) -> &ValidationResult {
        self.validation_result = ValidationEngine::new().validate(&self.game_dna);
        &self.validation_result
    }

    /// Validate a single GameDNA field and merge the field-specific findings into the builder's validation result.
    ///
    /// The `field` argument is the name of the GameDNA field to validate (for example: "genre", "camera",
    /// "target_platforms"). Unknown field names produce a warning with code `"UNKNOWN_FIELD"`.
    ///
    /// # Examples
    ///
    /// ```
    /// let game = GameDNA::default();
    /// let mut builder = ValidatedGameDNABuilder::new(game);
    /// // Validate the "genre" field and merge results into the builder's stored ValidationResult.
    /// let result = builder.validate_field("genre");
    /// // The returned reference points to the builder's current ValidationResult.
    /// assert_eq!(result as *const _, builder.validation_result() as *const _);
    /// ```
    pub fn validate_field(&mut self, field: &str) -> &ValidationResult {
        let field_result = ValidationEngine::new().validate_field(&self.game_dna, field);
        self.validation_result.merge(field_result);
        &self.validation_result
    }

    /// Indicates whether the builder's current validation result represents a valid configuration.
    ///
    /// # Returns
    ///
    /// `true` if the current `ValidationResult` indicates the configuration is valid, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let game = crate::schema::GameDNA::default();
    /// let builder = ValidatedGameDNABuilder::new(game);
    /// assert!(builder.is_valid() == builder.validation_result().is_valid);
    /// ```
    pub fn is_valid(&self) -> bool {
        self.validation_result.is_valid
    }

    /// Accesses the wrapped GameDNA.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // `builder` is a `ValidatedGameDNABuilder` initialized elsewhere
    /// let game = builder.game_dna();
    /// // `game` is a `&GameDNA` reference
    /// ```
    pub fn game_dna(&self) -> &GameDNA {
        &self.game_dna
    }

    /// Finalizes the builder by consuming it and returning the contained `GameDNA` if validation succeeded.
    ///
    /// # Returns
    ///
    /// `Ok(GameDNA)` when the builder's validation_result indicates validity, `Err(ValidationResult)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let game = GameDNA::default();
    /// let builder = ValidatedGameDNABuilder::new(game);
    /// // Either unwrap the valid GameDNA or handle the validation result
    /// let locked = builder.build().unwrap();
    /// ```
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
    /// Creates a new LockedGameDNA from a validated GameDNA.
    ///
    /// The returned instance stores a checksum of the provided configuration, records the current UTC lock timestamp,
    /// and is marked as locked.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::schema::GameDNA;
    /// use crate::validation::LockedGameDNA;
    ///
    /// let validated_config = /* a validated GameDNA */ ;
    /// let locked = LockedGameDNA::new(validated_config);
    /// assert!(locked.is_locked);
    /// ```
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

    /// Checks whether the locked GameDNA's stored checksum still matches the current checksum of its config.
    ///
    /// Returns `true` if the instance is locked and the stored checksum equals the checksum computed from the current config, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::schema::GameDNA;
    /// // Construct and lock a validated GameDNA (details omitted)
    /// let game = GameDNA::default();
    /// let locked = LockedGameDNA::new(game);
    /// assert!(locked.verify_integrity());
    /// ```
    pub fn verify_integrity(&self) -> bool {
        if !self.is_locked {
            return false;
        }
        
        let current_checksum = checksum::generate_checksum(&self.config);
        current_checksum == self.checksum
    }

    /// Unlocks the locked configuration, allowing callers to access the inner `GameDNA`.
    ///
    /// After calling this method, `get_config` and `get_config_mut` will return `Some(&GameDNA)` and
    /// `Some(&mut GameDNA)` respectively until `is_locked` is set true again.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = GameDNA::default();
    /// let mut locked = LockedGameDNA::new(config);
    /// locked.unlock();
    /// assert!(locked.get_config().is_some());
    /// ```
    pub fn unlock(&mut self) {
        self.is_locked = false;
    }

    /// Get a reference to the inner GameDNA when unlocked.
    ///
    /// Returns `Some(&GameDNA)` if the LockedGameDNA is unlocked, or `None` if it is locked.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // assume `locked` is a LockedGameDNA instance
    /// if let Some(cfg) = locked.get_config() {
    ///     // read from cfg
    /// }
    /// ```
    pub fn get_config(&self) -> Option<&GameDNA> {
        if self.is_locked {
            None
        } else {
            Some(&self.config)
        }
    }

    /// Obtain a mutable reference to the inner GameDNA when unlocked.
    ///
    /// If the LockedGameDNA is locked, no mutable access is provided.
    ///
    /// # Returns
    ///
    /// `Some(&mut GameDNA)` if unlocked, `None` if locked.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Assume `valid_game_dna` is a previously validated `GameDNA` value.
    /// let mut locked = LockedGameDNA::new(valid_game_dna);
    /// // While locked, mutable access is not available.
    /// assert!(locked.get_config_mut().is_none());
    ///
    /// // After unlocking, mutable access is permitted.
    /// locked.unlock();
    /// let cfg = locked.get_config_mut();
    /// assert!(cfg.is_some());
    /// ```
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
    /// Creates a new LockedGameDNABuilder for the provided GameDNA.
    ///
    /// The builder is initialized with the given `game_dna` and a fresh validation engine.
    ///
    /// # Arguments
    ///
    /// * `game_dna` - The GameDNA to be validated and eventually published.
    ///
    /// # Examples
    ///
    /// ```
    /// // Construct a GameDNA (details omitted)
    /// let game = GameDNA::default();
    /// let builder = LockedGameDNABuilder::new(game);
    /// // You can now run validation or publish via the builder:
    /// let result = builder.validate();
    /// ```
    pub fn new(game_dna: GameDNA) -> Self {
        Self {
            game_dna,
            validation_engine: ValidationEngine::new(),
        }
    }

    /// Runs validation over the builder's GameDNA and returns the aggregated validation result.
    ///
    /// The returned `ValidationResult` contains any errors, warnings, and suggestions produced by applying
    /// the engine's full set of validation rules and constraints to the builder's configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let game = GameDNA::default();
    /// let builder = LockedGameDNABuilder::new(game);
    /// let result = builder.validate();
    /// // Inspect `result` for errors, warnings, or suggestions.
    /// assert!(result.is_valid || !result.is_valid);
    /// ```
    pub fn validate(&self) -> ValidationResult {
        self.validation_engine.validate(&self.game_dna)
    }

    /// Validate a single named field of the wrapped GameDNA and return the field-specific validation outcome.
    ///
    /// The provided field name is validated against known GameDNA fields (for example: "name", "genre",
    /// "camera", "tone", "world_scale", "target_platforms", "physics_profile", "max_players",
    /// "target_fps", "time_scale", "npc_count"). Unknown field names produce a warning entry in the result.
    ///
    /// # Parameters
    ///
    /// - `field`: Name of the GameDNA field to validate (for example `"genre"` or `"camera"`).
    ///
    /// # Returns
    ///
    /// A `ValidationResult` containing any errors, warnings, and suggestions produced while validating the specified field.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::schema::GameDNA;
    /// use crate::validation::ValidatedGameDNABuilder;
    ///
    /// let game = GameDNA::default();
    /// let builder = ValidatedGameDNABuilder::new(game);
    /// let field_result = builder.validate_field("genre");
    /// assert!(field_result.errors.len() >= 0);
    /// ```
    pub fn validate_field(&self, field: &str) -> ValidationResult {
        self.validation_engine.validate_field(&self.game_dna, field)
    }

    /// Publishes the GameDNA by validating it and, if valid, locking and returning a LockedGameDNA.
    ///
    /// If validation fails, the original validation result containing errors and warnings is returned.
    ///
    /// # Returns
    ///
    /// `Ok(LockedGameDNA)` when the GameDNA passes validation, `Err(ValidationResult)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// // assumes `game_dna` is a valid GameDNA value and `LockedGameDNABuilder` is in scope
    /// let builder = LockedGameDNABuilder::new(game_dna);
    /// match builder.publish() {
    ///     Ok(locked) => {
    ///         assert!(locked.is_locked);
    ///     }
    ///     Err(result) => {
    ///         panic!("validation failed: {:?}", result.errors);
    ///     }
    /// }
    /// ```
    pub fn publish(self) -> Result<LockedGameDNA, ValidationResult> {
        let validation_result = self.validation_engine.validate(&self.game_dna);
        
        if validation_result.is_valid {
            Ok(LockedGameDNA::new(self.game_dna))
        } else {
            Err(validation_result)
        }
    }
}