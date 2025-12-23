//! Error types for Game DNA operations
//!
//! Provides structured error handling for parsing, schema validation,
//! serialization, and version management errors.

use thiserror::Error;

/// Error type for parsing GameDNA from serialization formats
#[derive(Error, Debug)]
pub enum ParseError {
    /// Invalid JSON syntax
    #[error("Invalid JSON syntax: {reason}\nJSON: {json_snippet}")]
    InvalidJson {
        /// Reason for JSON parsing failure
        reason: String,
        /// Snippet of problematic JSON
        json_snippet: String,
    },
    
    /// Invalid MessagePack data
    #[error("Invalid MessagePack data: {reason}")]
    InvalidMessagePack {
        /// Reason for MessagePack parsing failure
        reason: String,
    },
    
    /// Missing required field in parsed data
    #[error("Missing required field: {field_name}\nContext: {context}")]
    MissingField {
        /// Name of the missing field
        field_name: String,
        /// Context information about where the field was expected
        context: String,
    },
    
    /// Invalid field value
    #[error("Invalid value for field '{field_name}': {value}\nReason: {reason}")]
    InvalidFieldValue {
        /// Name of the field with invalid value
        field_name: String,
        /// The invalid value
        value: String,
        /// Explanation of why the value is invalid
        reason: String,
    },
    
    /// Invalid UUID format
    #[error("Invalid UUID format: {uuid}\n{help}")]
    InvalidUuid {
        /// The invalid UUID string
        uuid: String,
        /// Helpful suggestion for fixing the issue
        help: String,
    },
}

impl From<serde_json::Error> for ParseError {
    fn from(err: serde_json::Error) -> Self {
        ParseError::InvalidJson {
            reason: err.to_string(),
            json_snippet: String::new(),
        }
    }
}

/// Error type for schema validation and configuration errors
#[derive(Error, Debug)]
pub enum SchemaError {
    /// Invalid field value or configuration
    #[error("Invalid field '{field_name}': {description}\nSuggestion: {suggestion}")]
    InvalidField {
        /// Name of the invalid field
        field_name: String,
        /// Description of what's wrong
        description: String,
        /// Suggestion for how to fix it
        suggestion: String,
    },
    
    /// Incompatible configuration
    #[error("Incompatible configuration: {description}\nConflicting fields: {conflicting_fields}\nSuggestion: {suggestion}")]
    IncompatibleConfiguration {
        /// Description of the incompatibility
        description: String,
        /// Fields that are in conflict
        conflicting_fields: Vec<String>,
        /// Suggestion for resolving the conflict
        suggestion: String,
    },
    
    /// Missing required fields
    #[error("Missing required fields: {fields}\nPlease provide these fields to create a valid GameDNA.")]
    MissingRequiredFields {
        /// List of missing required field names
        fields: Vec<String>,
    },
    
    /// Invalid enum value or state
    #[error("Invalid enum configuration: {description}\nValid options: {valid_options}\nSuggestion: {suggestion}")]
    InvalidEnum {
        /// Description of the invalid enum usage
        description: String,
        /// Valid enum options
        valid_options: Vec<String>,
        /// Suggestion for valid usage
        suggestion: String,
    },
}

impl SchemaError {
    /// Creates a simpler InvalidField error with default suggestion
    pub fn invalid_field(field_name: String, description: String) -> Self {
        SchemaError::InvalidField {
            field_name,
            description,
            suggestion: "Check the field requirements and provide a valid value.".to_string(),
        }
    }
}

/// Error type for serialization and deserialization failures
#[derive(Error, Debug)]
pub enum SerializationError {
    /// JSON serialization error
    #[error("JSON serialization error: {reason}")]
    JsonSerialization {
        /// Reason for serialization failure
        reason: String,
    },
    
    /// JSON deserialization error
    #[error("JSON deserialization error: {reason}")]
    JsonDeserialization {
        /// Reason for deserialization failure
        reason: String,
    },
    
    /// MessagePack serialization error
    #[error("MessagePack serialization error: {reason}")]
    MessagePackSerialization {
        /// Reason for MessagePack serialization failure
        reason: String,
    },
    
    /// MessagePack deserialization error
    #[error("MessagePack deserialization error: {reason}")]
    MessagePackDeserialization {
        /// Reason for MessagePack deserialization failure
        reason: String,
    },
    
    /// Protobuf encoding error
    #[error("Protobuf encoding error: {reason}")]
    ProtobufEncoding {
        /// Reason for Protobuf encoding failure
        reason: String,
    },
    
    /// Protobuf decoding error
    #[error("Protobuf decoding error: {reason}")]
    ProtobufDecoding {
        /// Reason for Protobuf decoding failure
        reason: String,
    },
    
    /// Encoding error with specific type information
    #[error("Encoding error for type '{type_name}': {details}")]
    EncodingError {
        /// Name of the type that failed to encode
        type_name: String,
        /// Detailed error information
        details: String,
    },
    
    /// Decoding error with specific type information
    #[error("Decoding error for type '{type_name}': {details}")]
    DecodingError {
        /// Name of the type that failed to decode
        type_name: String,
        /// Detailed error information
        details: String,
    },
}

impl From<serde_json::Error> for SerializationError {
    fn from(err: serde_json::Error) -> Self {
        if err.is_data() {
            SerializationError::JsonDeserialization {
                reason: err.to_string(),
            }
        } else {
            SerializationError::JsonSerialization {
                reason: err.to_string(),
            }
        }
    }
}

/// Error type for version and compatibility management
#[derive(Error, Debug)]
pub enum VersionError {
    /// Schema version mismatch
    #[error("Schema version mismatch: {current_version} (current) vs {target_version} (target)\n{help}")]
    VersionMismatch {
        /// Current schema version
        current_version: String,
        /// Target schema version
        target_version: String,
        /// Helpful message for resolving the mismatch
        help: String,
    },
    
    /// Incompatible schema version (breaking changes)
    #[error("Incompatible schema version: {reason}\nCurrent version: {current_version}\nRequested version: {requested_version}\nMigration needed: {migration_available}")]
    IncompatibleVersion {
        /// Reason for incompatibility
        reason: String,
        /// Current schema version
        current_version: String,
        /// Requested schema version
        requested_version: String,
        /// Whether a migration is available
        migration_available: bool,
    },
    
    /// Migration not available for version upgrade
    #[error("No migration available from version {from_version} to {to_version}\n{help}")]
    MigrationNotAvailable {
        /// Source version
        from_version: String,
        /// Target version
        to_version: String,
        /// Helpful suggestion
        help: String,
    },
    
    /// Invalid version format
    #[error("Invalid version format: {version}\nReason: {reason}\nExpected format: {expected_format}")]
    InvalidVersionFormat {
        /// The invalid version string
        version: String,
        /// Explanation of what's wrong
        reason: String,
        /// Expected version format
        expected_format: String,
    },
    
    /// Version downgrade not supported
    #[error("Version downgrade detected: {from} to {to}\nDowngrades are not supported as they may cause data loss.")]
    VersionDowngrade {
        /// Current (higher) version
        from: String,
        /// Target (lower) version
        to: String,
    },
}