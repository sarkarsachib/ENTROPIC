//! Deterministic checksum generation for Game DNA configurations
//!
//! This module provides SHA-256 hashing for GameDNA configurations to ensure
//! integrity and create immutable "locked" configurations.

use crate::schema::GameDNA;
use sha2::{Sha256, Digest};
use serde_json;
use std::fmt::Write;

/// Generate a deterministic SHA-256 checksum for a GameDNA configuration
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

/// Create a deterministic JSON representation of GameDNA for checksum generation
fn create_deterministic_json(game_dna: &GameDNA) -> String {
    // We need to create a JSON string with sorted keys for determinism
    // Serialize to a value first, then sort the keys
    let mut value = serde_json::to_value(game_dna).expect("Failed to serialize GameDNA");
    
    // Recursively sort all object keys
    sort_json_keys(&mut value);
    
    // Convert to string with consistent formatting
    serde_json::to_string_pretty(&value).expect("Failed to serialize sorted JSON")
}

/// Recursively sort JSON object keys for deterministic serialization
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

/// Verify the integrity of a GameDNA configuration against a checksum
pub fn verify_checksum(game_dna: &GameDNA, expected_checksum: &str) -> bool {
    let actual_checksum = generate_checksum(game_dna);
    actual_checksum == expected_checksum
}

/// Generate a checksum with additional metadata (version, timestamp)
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

/// Generate a short checksum (first 16 characters of full checksum)
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