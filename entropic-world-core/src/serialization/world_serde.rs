use crate::world::World;
use crate::errors::{Result, WorldError};
use serde_json;

/// Serialize a World into a human-readable (pretty-printed) JSON string.
///
/// # Returns
///
/// `Ok` with the pretty-printed JSON representation of the `World`, or `Err` with
/// `WorldError::SerializationError` containing the serde error message if
/// serialization fails.
///
/// # Examples
///
/// ```
/// let world = World::new("earth".into(), 1, 1);
/// let json = serialize_to_json(&world).unwrap();
/// assert!(json.contains("earth"));
/// ```
pub fn serialize_to_json(world: &World) -> Result<String> {
    serde_json::to_string_pretty(world)
        .map_err(|e| WorldError::SerializationError(e.to_string()))
}

/// Serialize a `World` into a compact JSON string.
///
/// Produces a minified JSON representation of `world`. On failure returns
/// `WorldError::SerializationError` containing the serde error message.
///
/// # Examples
///
/// ```
/// let w = World::new("test", 1, 16, 16);
/// let json = entropic_world_core::serialization::world_serde::serialize_to_json_compact(&w).unwrap();
/// assert!(json.contains("\"name\":\"test\""));
/// ```
pub fn serialize_to_json_compact(world: &World) -> Result<String> {
    serde_json::to_string(world)
        .map_err(|e| WorldError::SerializationError(e.to_string()))
}

/// Deserialize a `World` instance from its JSON representation.
///
/// # Examples
///
/// ```no_run
/// let json = r#"{"name":"example","id":"uuid","width_chunks":16}"#;
/// let world = deserialize_from_json(json).unwrap();
/// ```
pub fn deserialize_from_json(json: &str) -> Result<World> {
    serde_json::from_str(json)
        .map_err(|e| WorldError::SerializationError(e.to_string()))
}

/// Serializes a `World` into compact JSON bytes.
///
/// On success returns a `Vec<u8>` containing the compact JSON encoding of the world.
/// On failure returns `WorldError::SerializationError` with a description of the serialization error.
///
/// # Examples
///
/// ```
/// let w = World::new("test", 1, 16, 16);
/// let bytes = serialize_to_bytes(&w).unwrap();
/// let s = std::str::from_utf8(&bytes).unwrap();
/// assert!(s.contains("\"name\":\"test\""));
/// ```
pub fn serialize_to_bytes(world: &World) -> Result<Vec<u8>> {
    serde_json::to_vec(world)
        .map_err(|e| WorldError::SerializationError(e.to_string()))
}

/// Deserializes a `World` from a JSON byte slice.
///
/// # Examples
///
/// ```
/// let original = World::new("test", 1, 16, 16);
/// let bytes = serde_json::to_vec(&original).unwrap();
/// let deserialized = deserialize_from_bytes(&bytes).unwrap();
/// assert_eq!(deserialized.id, original.id);
/// ```
pub fn deserialize_from_bytes(bytes: &[u8]) -> Result<World> {
    serde_json::from_slice(bytes)
        .map_err(|e| WorldError::SerializationError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_round_trip() {
        let world = World::new(
            "Test World".to_string(),
            "game_dna_1".to_string(),
            10,
            10,
        );

        let json = serialize_to_json(&world).unwrap();
        let deserialized = deserialize_from_json(&json).unwrap();

        assert_eq!(world.name, deserialized.name);
        assert_eq!(world.id, deserialized.id);
    }

    #[test]
    fn test_bytes_round_trip() {
        let world = World::new(
            "Test World".to_string(),
            "game_dna_1".to_string(),
            5,
            5,
        );

        let bytes = serialize_to_bytes(&world).unwrap();
        let deserialized = deserialize_from_bytes(&bytes).unwrap();

        assert_eq!(world.name, deserialized.name);
        assert_eq!(world.width_chunks, deserialized.width_chunks);
    }
}