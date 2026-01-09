//! Deterministic checksum generation for Game DNA configurations
//!
//! This module provides SHA-256 hashing for GameDNA configurations to ensure
//! integrity and create immutable "locked" configurations.

use crate::schema::GameDNA;
use sha2::{Sha256, Digest};
use serde_json;
use std::fmt::Write;

/// Produces a deterministic SHA-256 checksum for a GameDNA configuration.
///
/// The checksum is computed from a deterministic JSON serialization of `game_dna` (object keys
/// sorted and pretty-printed) so identical logical configurations yield identical checksums.
///
/// # Examples
///
/// ```ignore
/// // Construct or obtain a `GameDNA` instance appropriate for your codebase.
/// let game_dna = GameDNA::default();
/// let checksum = generate_checksum(&game_dna);
/// // SHA-256 hex digests are 64 lowercase hex characters.
/// assert_eq!(checksum.len(), 64);
/// ```
///
/// # Returns
///
/// A lowercase hexadecimal string containing the SHA-256 digest of the deterministic JSON representation.
pub fn generate_checksum(game_dna: &GameDNA) -> String {
    // Create a deterministic JSON representation
    let json_string = create_deterministic_json(game_dna);
    
    // Generate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(json_string.as_bytes());
    let hash_result = hasher.finalize();
    
    // Convert to hex string
    format!("{:x}", hash_result)
}

/// Produce a deterministic, pretty-printed JSON string for a GameDNA suitable for checksum generation.

///

/// This function serializes `game_dna` to JSON, recursively sorts all object keys to ensure a stable key

/// ordering, and returns a consistently formatted (pretty-printed) JSON string. The resulting string is

/// stable across runs for the same logical `GameDNA` content and is intended for use as the canonical

/// input to checksum functions.

///

/// # Panics

///

/// Panics if serialization of `GameDNA` to `serde_json::Value` or the final serialization to a string fails.

///

/// # Examples

///

/// ```no_run

/// // Given a `GameDNA` value `dna`, produce its deterministic JSON representation:

/// // let dna = /* construct or obtain a GameDNA instance */;

/// // let json_str = create_deterministic_json(&dna);

/// // assert!(json_str.starts_with('{'));

/// ```
fn create_deterministic_json(game_dna: &GameDNA) -> String {
    // We need to create a JSON string with sorted keys for determinism
    // Serialize to a value first, then sort the keys
    let mut value = serde_json::to_value(game_dna).expect("Failed to serialize GameDNA");
    
    // Recursively sort all object keys
    sort_json_keys(&mut value);
    
    // Convert to string with consistent formatting
    serde_json::to_string_pretty(&value).expect("Failed to serialize sorted JSON")
}

/// Sorts JSON object keys recursively to produce a deterministic key order for serialization.
///
/// Modifies `value` in place: object keys are reordered lexicographically and nested objects and
/// array elements are processed recursively so that serializing the value (e.g., with
/// `serde_json::to_string_pretty`) yields a stable representation.
///
/// # Examples
///
/// ```
/// use serde_json::json;
/// let mut v = json!({"b":1,"a":{"d":4,"c":3}});
/// // After sorting, keys should be in lexicographic order
/// sort_json_keys(&mut v);
/// assert_eq!(v.to_string(), r#"{"a":{"c":3,"d":4},"b":1}"#);
/// ```
fn sort_json_keys(value: &mut serde_json::Value) {
    if let serde_json::Value::Object(map) = value {
        // Convert to a vector of tuples, sort by key, then rebuild
        let mut sorted_pairs: Vec<_> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        sorted_pairs.sort_by(|a, b| a.0.cmp(&b.0));
        
        // Rebuild the map with sorted keys
        let sorted_map = serde_json::Map::new();
        for (key, mut val) in sorted_pairs {
            // Recursively sort nested objects
            sort_json_keys(&mut val);
            map.insert(key, val);
        }
    } else if let serde_json::Value::Array(arr) = value {
        // Sort arrays of objects by their string representation for determinism
        // This is a simple approach - for more complex cases, you might need
        // a more sophisticated sorting strategy
        for val in arr {
            sort_json_keys(val);
        }
    }
}

/// Verifies that the SHA-256 checksum of `game_dna` matches `expected_checksum`.
///
/// # Returns
/// `true` if the computed checksum equals `expected_checksum`, `false` otherwise.
///
/// # Examples
///
/// ```
/// // let dna = GameDNA::from(...);
/// // let checksum = generate_checksum(&dna);
/// // assert!(verify_checksum(&dna, &checksum));
/// ```
pub fn verify_checksum(game_dna: &GameDNA, expected_checksum: &str) -> bool {
    let actual_checksum = generate_checksum(game_dna);
    actual_checksum == expected_checksum
}

/// Generates a SHA-256 checksum for the given GameDNA combined with version and timestamp metadata.
///
/// The checksum is computed over the string formed as `version:timestamp:json`, where `json` is a
/// deterministic JSON serialization of the provided GameDNA. The `version` and `timestamp` values
/// are included verbatim in the combined input.
///
/// # Parameters
///
/// - `version`: Version identifier to include in the checksum input (e.g., `"v1.0"`).
/// - `timestamp`: Timestamp string to include in the checksum input (e.g., an ISO 8601 string).
///
/// # Returns
///
/// A lowercase hexadecimal `String` containing the SHA-256 digest of the combined metadata and deterministic JSON.
///
/// # Examples
///
/// ```
/// let dna = GameDNA::default();
/// let checksum = generate_checksum_with_metadata(&dna, "v1.0", "2025-01-01T12:00:00Z");
/// assert_eq!(checksum.len(), 64);
/// ```
pub fn generate_checksum_with_metadata(game_dna: &GameDNA, version: &str, timestamp: &str) -> String {
    let json_string = create_deterministic_json(game_dna);
    
    // Create a combined string with metadata
    let mut combined = String::new();
    write!(combined, "{}:{}:{}", version, timestamp, json_string).expect("Failed to create combined string");
    
    // Generate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let hash_result = hasher.finalize();
    
    // Convert to hex string
    format!("{:x}", hash_result)
}

/// Produce a 16-character hexadecimal checksum prefix for a GameDNA.
///
/// The returned string is the first 16 characters of the full SHA-256 hex digest
/// computed deterministically from the provided `GameDNA`.
///
/// # Examples
///
/// ```no_run
/// let game_dna = /* construct or obtain a GameDNA instance */ ;
/// let short = generate_short_checksum(&game_dna);
/// assert_eq!(short.len(), 16);
/// ```
pub fn generate_short_checksum(game_dna: &GameDNA) -> String {
    let full_checksum = generate_checksum(game_dna);
    full_checksum.chars().take(16).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{GameDNA, Genre, TargetPlatform};
    
    #[test]
    fn test_checksum_deterministic() {
        // Create two identical GameDNA instances
        let game1 = GameDNA::minimal("Test Game".to_string(), Genre::FPS, vec![TargetPlatform::PC]);
        let game2 = GameDNA::minimal("Test Game".to_string(), Genre::FPS, vec![TargetPlatform::PC]);
        
        // They should have the same checksum
        let checksum1 = generate_checksum(&game1);
        let checksum2 = generate_checksum(&game2);
        
        assert_eq!(checksum1, checksum2);
    }
    
    #[test]
    fn test_checksum_different() {
        // Create two different GameDNA instances
        let game1 = GameDNA::minimal("Game 1".to_string(), Genre::FPS, vec![TargetPlatform::PC]);
        let game2 = GameDNA::minimal("Game 2".to_string(), Genre::RPG, vec![TargetPlatform::Console]);
        
        // They should have different checksums
        let checksum1 = generate_checksum(&game1);
        let checksum2 = generate_checksum(&game2);
        
        assert_ne!(checksum1, checksum2);
    }
    
    #[test]
    fn test_checksum_verification() {
        let game = GameDNA::minimal("Test Game".to_string(), Genre::FPS, vec![TargetPlatform::PC]);
        let checksum = generate_checksum(&game);
        
        // Verification should pass
        assert!(verify_checksum(&game, &checksum));
        
        // Verification with wrong checksum should fail
        assert!(!verify_checksum(&game, "wrong_checksum"));
    }
    
    #[test]
    fn test_short_checksum() {
        let game = GameDNA::minimal("Test Game".to_string(), Genre::FPS, vec![TargetPlatform::PC]);
        let short_checksum = generate_short_checksum(&game);
        
        // Short checksum should be 16 characters
        assert_eq!(short_checksum.len(), 16);
        
        // Should be hex characters only
        assert!(short_checksum.chars().all(|c| c.is_ascii_hexdigit()));
    }
}