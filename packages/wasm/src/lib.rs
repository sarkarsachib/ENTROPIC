use wasm_bindgen::prelude::*;

use entropic_dna_core::{
    schema::GameDNA,
    serialization::{from_json_str, to_json_string},
    validation::{checksum, ValidationEngine},
};

/// Validate a GameDNA JSON string and produce a JavaScript-serializable validation result.
///
/// Parses the given JSON into a GameDNA configuration, runs the validation engine on it,
/// and converts the validation outcome into a `JsValue` suitable for consumption by JavaScript.
///
/// # Examples
///
/// ```
/// let json = r#"{
///   "name": "ExampleGame",
///   "entities": []
/// }"#;
/// let result = entropic_wasm::validate_game_dna(json);
/// assert!(result.is_ok());
/// ```
///
/// # Returns
///
/// - `Ok`: a `JsValue` containing the validation outcome (success or structured validation errors).
/// - `Err`: a `JsValue` holding an error message when parsing or conversion fails.
#[wasm_bindgen]
pub fn validate_game_dna(json: &str) -> Result<JsValue, JsValue> {
    let config: GameDNA = from_json_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let engine = ValidationEngine::new();
    let result = engine.validate(&config);

    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Serialize a GameDNA JSON string into a normalized JSON representation.
///
/// Parses the provided GameDNA JSON, then returns a standardized JSON string
/// representation of the parsed configuration.
///
/// On success, returns `Ok(String)` containing the serialized JSON; on failure,
/// returns `Err(JsValue)` containing an error message.
///
/// # Examples
///
/// ```
/// let json = r#"{
///   "name": "MyGame",
///   "version": 1
/// }"#;
/// let out = serialize_game_dna(json).unwrap();
/// assert!(out.contains("\"name\""));
/// ```
#[wasm_bindgen]
pub fn serialize_game_dna(json: &str) -> Result<String, JsValue> {
    let config: GameDNA = from_json_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    to_json_string(&config).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Computes a checksum for a GameDNA JSON string.
///
/// Parses the provided JSON into a `GameDNA` configuration and returns its checksum.
///
/// # Returns
///
/// The checksum string computed from the parsed `GameDNA` configuration.
///
/// # Errors
///
/// Returns a `JsValue` containing the error message if parsing the JSON or generating the checksum fails.
///
/// # Examples
///
/// ```
/// let json = r#"{"name":"Example","version":1}"#;
/// let checksum = calculate_checksum(json).unwrap();
/// assert!(!checksum.is_empty());
/// ```
#[wasm_bindgen]
pub fn calculate_checksum(json: &str) -> Result<String, JsValue> {
    let config: GameDNA = from_json_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(checksum::generate_checksum(&config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_config() {
        let json = r#"{
          \"id\": \"test\",
          \"name\": \"Test Game\",
          \"version\": {\"major\":0,\"minor\":1,\"patch\":0},
          \"genre\": \"FPS\",
          \"camera\": \"Perspective3D\",
          \"tone\": \"Arcade\",
          \"world_scale\": \"SmallLevel\",
          \"target_platforms\": [\"PC\"],
          \"physics_profile\": \"Arcade\",
          \"max_players\": 1,
          \"is_competitive\": false,
          \"supports_coop\": false,
          \"difficulty\": \"Easy\",
          \"monetization\": \"PremiumBuy\",
          \"target_audience\": \"Everyone\",
          \"esrb_rating\": null,
          \"target_fps\": 60,
          \"max_draw_distance\": 1000.0,
          \"max_entities\": 100,
          \"max_npc_count\": 10,
          \"time_scale\": 1.0,
          \"weather_enabled\": false,
          \"seasons_enabled\": false,
          \"day_night_cycle\": false,
          \"persistent_world\": false,
          \"npc_count\": 0,
          \"ai_enabled\": false,
          \"ai_difficulty_scaling\": false,
          \"has_campaign\": false,
          \"has_side_quests\": false,
          \"dynamic_quests\": false,
          \"tags\": [],
          \"custom_properties\": {}
        }"#;

        let result = validate_game_dna(json);
        assert!(result.is_ok());
    }
}