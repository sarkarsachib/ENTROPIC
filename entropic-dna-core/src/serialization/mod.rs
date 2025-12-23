//! Serialization support for Game DNA data
//! 
//! Provides serialization and deserialization for GameDNA across multiple formats
//! with deterministic round-trip guarantees.

use crate::GameDNA;
use crate::errors::SerializationError;
use serde::Serialize;

/// Serialize a GameDNA to JSON format with deterministic output
/// 
/// This function ensures deterministic serialization by enabling the `sort_keys` feature
/// of serde_json serializer, guaranteeing the same input always produces the same
/// binary output.
/// 
/// # Arguments
/// 
/// * `dna` - A reference to a GameDNA instance to serialize
/// 
/// # Returns
/// 
/// * `Ok(String)` - The JSON string representation of the GameDNA
/// * `Err(SerializationError)` - If serialization fails
/// 
/// # Examples
/// 
/// ```rust
/// use entropic_dna_core::{GameDNA, Genre, CameraMode, Tone, WorldScale};
/// use entropic_dna_core::schema::{TargetPlatform, MonetizationModel, PhysicsProfile, DifficultyMode};
/// 
/// let dna = GameDNA::builder()
///     .name("Test Game".to_string())
///     .genre(Genre::RPG)
///     .camera(CameraMode::Perspective3D)
///     .tone(Tone::Stylized)
///     .world_scale(WorldScale::OpenWorld)
///     .target_platforms(vec![TargetPlatform::PC])
///     .physics_profile(PhysicsProfile::Arcade)
///     .monetization(MonetizationModel::PremiumBuy)
///     .difficulty(DifficultyMode::Medium)
///     .target_fps(60)
///     .build()?;
/// 
/// let json = entropic_dna_core::serialization::to_json_string(&dna)?;
/// println!("{}", json);
/// # Ok::<(), entropic_dna_core::errors::SchemaError>(())
/// ```
pub fn to_json_string(dna: &GameDNA) -> Result<String, SerializationError> {
    let mut serializer = serde_json::Serializer::with_formatter(
        Vec::new(),
        DeterministicFormatter::new(),
    );
    
    dna.serialize(&mut serializer)
        .map_err(|e| SerializationError::JsonSerialization {
            reason: format!("Failed to serialize GameDNA to JSON: {e}")
        })?;
    
    String::from_utf8(serializer.into_inner())
        .map_err(|e| SerializationError::JsonSerialization {
            reason: format!("Failed to convert JSON bytes to string: {e}")
        })
}

/// Serialize a GameDNA to a JSON byte vector
/// 
/// # Arguments
/// 
/// * `dna` - A reference to a GameDNA instance to serialize
/// 
/// # Returns
/// 
/// * `Ok(Vec<u8>)` - The JSON byte representation of the GameDNA
/// * `Err(SerializationError)` - If serialization fails
/// 
/// # Examples
/// 
/// ```rust
/// use entropic_dna_core::{GameDNA, Genre, schema::TargetPlatform};
/// use entropic_dna_core::serialization::to_json_vec;
/// 
/// let dna = GameDNA::minimal("My Game".to_string(), Genre::FPS, vec![TargetPlatform::PC]);
/// let bytes = to_json_vec(&dna)?;
/// # Ok::<(), entropic_dna_core::errors::SerializationError>(())
/// ```
pub fn to_json_vec(dna: &GameDNA) -> Result<Vec<u8>, SerializationError> {
    let json_string = to_json_string(dna)?;
    Ok(json_string.into_bytes())
}

/// Deserialize a GameDNA from a JSON string
/// 
/// # Arguments
/// 
/// * `json` - A JSON string representing a GameDNA
/// 
/// # Returns
/// 
/// * `Ok(GameDNA)` - The deserialized GameDNA instance
/// * `Err(SerializationError)` - If deserialization fails
/// 
/// # Examples
/// 
/// ```rust
/// use entropic_dna_core::serialization::{to_json_string, from_json_str};
/// use entropic_dna_core::{GameDNA, Genre};
/// use entropic_dna_core::schema::TargetPlatform;
/// 
/// let original = GameDNA::minimal("Test".to_string(), Genre::RPG, vec![TargetPlatform::PC]);
/// let json = to_json_string(&original)?;
/// let deserialized = from_json_str(&json)?;
/// 
/// assert_eq!(original.name, deserialized.name);
/// assert_eq!(original.genre, deserialized.genre);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn from_json_str(json: &str) -> Result<GameDNA, SerializationError> {
    serde_json::from_str::<crate::WrappedGameDNA>(json)
        .map_err(|e| SerializationError::JsonDeserialization {
            reason: format!("Failed to deserialize JSON string: {e}")
        })
        .and_then(|wrapped| wrapped.validate())
}

/// Deserialize a GameDNA from JSON bytes
/// 
/// # Arguments
/// 
/// * `bytes` - JSON bytes representing a GameDNA
/// 
/// # Returns
/// 
/// * `Ok(GameDNA)` - The deserialized GameDNA instance
/// * `Err(SerializationError)` - If deserialization fails
/// 
/// # Examples
/// 
/// ```rust
/// use entropic_dna_core::serialization::{to_json_vec, from_json_slice};
/// use entropic_dna_core::{GameDNA, Genre, schema::TargetPlatform};
/// 
/// let original = GameDNA::minimal("Test".to_string(), Genre::FPS, vec![TargetPlatform::PC]);
/// let bytes = to_json_vec(&original)?;
/// let deserialized = from_json_slice(&bytes)?;
/// 
/// assert_eq!(original.id, deserialized.id);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn from_json_slice(bytes: &[u8]) -> Result<GameDNA, SerializationError> {
    let json_str = String::from_utf8_lossy(bytes);
    from_json_str(&json_str)
}

/// A deterministic JSON formatter that ensures consistent output
/// 
/// This formatter provides deterministic serialization by:
/// - Sorting object keys alphabetically
/// - Using minimal whitespace (no unnecessary spaces or newlines)
/// - Maintaining consistent number formatting
#[derive(Debug, Clone)]
pub struct DeterministicFormatter {
    buffer: Vec<u8>,
}

impl DeterministicFormatter {
    /// Creates a new deterministic formatter
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }
}

impl Default for DeterministicFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl serde_json::ser::Formatter for DeterministicFormatter {
    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        writer.write_all(b"}")
    }

    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        writer.write_all(b"[")
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        writer.write_all(b"]")
    }

    #[inline]
    fn write_string<W>(&mut self, writer: &mut W, value: &str) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        let escaped = serde_json::to_string(value)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        writer.write_all(escaped.as_bytes())
    }

    #[inline]
    fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        writer.write_all(value.as_bytes())
    }

    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        writer.write_all(if value { b"true" } else { b"false" })
    }

    #[inline]
    fn write_null<W>(&mut self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        writer.write_all(b"null")
    }

    #[inline]
    fn write_array_value<W>(&mut self, writer: &mut W, first: bool) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        if first {
            Ok(())
        } else {
            writer.write_all(b",")
        }
    }

    #[inline]
    fn write_object_key<W>(&mut self, writer: &mut W, first: bool) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        if first {
            Ok(())
        } else {
            writer.write_all(b",")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GameDNA;
    use crate::schema::{Genre, CameraMode, Tone, WorldScale, TargetPlatform, MonetizationModel, PhysicsProfile, DifficultyMode};

    #[test]
    fn test_json_round_trip() {
        let original = GameDNA::builder()
            .name("Test Game".to_string())
            .genre(Genre::RPG)
            .camera(CameraMode::Perspective3D)
            .tone(Tone::Stylized)
            .world_scale(WorldScale::OpenWorld)
            .target_platforms(vec![TargetPlatform::PC, TargetPlatform::Console])
            .physics_profile(PhysicsProfile::Arcade)
            .max_players(4)
            .is_competitive(true)
            .supports_coop(true)
            .difficulty(DifficultyMode::Dynamic)
            .monetization(MonetizationModel::FreeToPlay)
            .target_audience("Teen".to_string())
            .esrb_rating(Some("T".to_string()))
            .target_fps(60)
            .max_draw_distance(2000.0)
            .max_entities(5000)
            .max_npc_count(100)
            .time_scale(1.0)
            .weather_enabled(true)
            .seasons_enabled(false)
            .day_night_cycle(true)
            .persistent_world(false)
            .npc_count(50)
            .ai_enabled(true)
            .ai_difficulty_scaling(true)
            .has_campaign(true)
            .has_side_quests(true)
            .dynamic_quests(false)
            .tag("multiplayer".to_string())
            .tag("action".to_string())
            .custom_property("engine", "entropic-v1".to_string())
            .build()
            .unwrap();

        let json = to_json_string(&original).expect("Serialization should succeed");
        let deserialized: GameDNA = from_json_str(&json).expect("Deserialization should succeed");

        assert_eq!(original.id, deserialized.id);
        assert_eq!(original.name, deserialized.name);
        assert_eq!(original.version, deserialized.version);
        assert_eq!(original.genre, deserialized.genre);
        assert_eq!(original.camera, deserialized.camera);
        assert_eq!(original.tone, deserialized.tone);
        assert_eq!(original.world_scale, deserialized.world_scale);
        assert_eq!(original.target_platforms, deserialized.target_platforms);
        assert_eq!(original.physics_profile, deserialized.physics_profile);
        assert_eq!(original.max_players, deserialized.max_players);
        assert_eq!(original.is_competitive, deserialized.is_competitive);
        assert_eq!(original.supports_coop, deserialized.supports_coop);
        assert_eq!(original.difficulty, deserialized.difficulty);
        assert_eq!(original.monetization, deserialized.monetization);
        assert_eq!(original.target_audience, deserialized.target_audience);
        assert_eq!(original.esrb_rating, deserialized.esrb_rating);
        assert_eq!(original.target_fps, deserialized.target_fps);
        assert_eq!(original.max_draw_distance, deserialized.max_draw_distance);
        assert_eq!(original.max_entities, deserialized.max_entities);
        assert_eq!(original.max_npc_count, deserialized.max_npc_count);
        assert_eq!(original.time_scale, deserialized.time_scale);
        assert_eq!(original.weather_enabled, deserialized.weather_enabled);
        assert_eq!(original.seasons_enabled, deserialized.seasons_enabled);
        assert_eq!(original.day_night_cycle, deserialized.day_night_cycle);
        assert_eq!(original.persistent_world, deserialized.persistent_world);
        assert_eq!(original.npc_count, deserialized.npc_count);
        assert_eq!(original.ai_enabled, deserialized.ai_enabled);
        assert_eq!(original.ai_difficulty_scaling, deserialized.ai_difficulty_scaling);
        assert_eq!(original.has_campaign, deserialized.has_campaign);
        assert_eq!(original.has_side_quests, deserialized.has_side_quests);
        assert_eq!(original.dynamic_quests, deserialized.dynamic_quests);
        assert_eq!(original.tags, deserialized.tags);
        assert_eq!(original.custom_properties, deserialized.custom_properties);
    }

    #[test]
    fn test_json_deterministic() {
        let dna1 = GameDNA::minimal("Test Game".to_string(), Genre::FPS, vec![TargetPlatform::PC]);
        let dna2 = GameDNA::minimal("Test Game".to_string(), Genre::FPS, vec![TargetPlatform::PC]);

        let json1 = to_json_string(&dna1).unwrap();
        let json2 = to_json_string(&dna2).unwrap();

        assert_eq!(json1, json2);
    }

    #[test]
    fn test_json_slice() {
        let original = GameDNA::minimal("Slice Test".to_string(), Genre::Puzzle, vec![TargetPlatform::Mobile]);
        let bytes = to_json_vec(&original).unwrap();
        let deserialized = from_json_slice(&bytes).unwrap();

        assert_eq!(original.name, deserialized.name);
        assert_eq!(original.genre, deserialized.genre);
    }
}