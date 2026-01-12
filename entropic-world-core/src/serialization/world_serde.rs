use crate::world::World;
use crate::errors::{Result, WorldError};
use serde_json;

pub fn serialize_to_json(world: &World) -> Result<String> {
    serde_json::to_string_pretty(world)
        .map_err(|e| WorldError::SerializationError(e.to_string()))
}

pub fn serialize_to_json_compact(world: &World) -> Result<String> {
    serde_json::to_string(world)
        .map_err(|e| WorldError::SerializationError(e.to_string()))
}

pub fn deserialize_from_json(json: &str) -> Result<World> {
    serde_json::from_str(json)
        .map_err(|e| WorldError::SerializationError(e.to_string()))
}

pub fn serialize_to_bytes(world: &World) -> Result<Vec<u8>> {
    serde_json::to_vec(world)
        .map_err(|e| WorldError::SerializationError(e.to_string()))
}

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
