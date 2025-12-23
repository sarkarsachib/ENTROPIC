//! Example: Serializing and Deserializing Game DNA
//!
//! This example demonstrates JSON serialization and deserialization of GameDNA,
//! including round-trip consistency verification.

use entropic_dna_core::{
    GameDNA,
    schema::{Genre, CameraMode, Tone, WorldScale, TargetPlatform},
    serialization::{self, to_json_string, from_json_str},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Game DNA Serialization Example\n");

    // Create a game configuration
    let original_game = GameDNA::builder()
        .name("Shadow Protocol".to_string())
        .genre(Genre::FPS)
        .camera(CameraMode::Perspective3D)
        .tone(Tone::Realistic)
        .world_scale(WorldScale::MediumLevel)
        .target_platforms(vec![TargetPlatform::PC, TargetPlatform::Console])
        .target_fps(120)
        .max_players(8)
        .is_competitive(true)
        .supports_coop(true)
        .tag("tactical".to_string())
        .tag("multiplayer".to_string())
        .custom_property("engine", "entropic-v1")
        .custom_property("has_ranked_mode", "true")
        .build()?;

    println!("🎮 Original Game DNA:");
    println!("   Name: {}", original_game.name);
    println!("   Genre: {:?}", original_game.genre);
    println!("   Platforms: {:?}", original_game.target_platforms);
    println!("   Max Players: {}", original_game.max_players);
    println!();

    // Serialize to JSON
    println!("📤 Serializing to JSON...");
    let json_string = to_json_string(&original_game)?;
    println!("✅ Serialization successful!\n");
    
    println!("📝 JSON Output:");
    println!("{}", json_string);
    println!();

    // Pretty print the JSON for better readability
    let pretty_json = serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&json_string)?)?;
    println!("📋 JSON Output (pretty-printed):");
    println!("{}", pretty_json);
    println!();

    // Deserialize back from JSON
    println!("📥 Deserializing from JSON...");
    let deserialized_game = from_json_str(&json_string)?;
    println!("✅ Deserialization successful!\n");

    // Verify round-trip consistency
    println!("🔍 Verifying Round-Trip Consistency:");
    println!("═══════════════════════════════════\n");
    
    let fields_to_check = vec![
        ("ID", original_game.id == deserialized_game.id),
        ("Name", original_game.name == deserialized_game.name),
        ("Version", original_game.version == deserialized_game.version),
        ("Genre", original_game.genre == deserialized_game.genre),
        ("Camera Mode", original_game.camera == deserialized_game.camera),
        ("Tone", original_game.tone == deserialized_game.tone),
        ("World Scale", original_game.world_scale == deserialized_game.world_scale),
        ("Target Platforms", original_game.target_platforms == deserialized_game.target_platforms),
        ("Max Players", original_game.max_players == deserialized_game.max_players),
        ("Competitive", original_game.is_competitive == deserialized_game.is_competitive),
        ("Co-op Support", original_game.supports_coop == deserialized_game.supports_coop),
        ("Target FPS", original_game.target_fps == deserialized_game.target_fps),
        ("Tags", original_game.tags == deserialized_game.tags),
        ("Custom Properties", original_game.custom_properties == deserialized_game.custom_properties),
    ];

    let mut all_match = true;
    for (field_name, matches) in fields_to_check {
        let status = if matches { "✅" } else { "❌" };
        println!("   {} {}: {}", status, field_name, if matches { "MATCH" } else { "MISMATCH" });
        if !matches {
            all_match = false;
        }
    }
    
    println!();
    if all_match {
        println!("🎉 Perfect round-trip! All fields match exactly.");
    } else {
        println!("⚠️  Some fields don't match. This indicates a potential serialization issue.");
    }
    
    println!();

    // Demonstrate serialization to bytes and back
    println!("💾 Byte Serialization Demo:");
    println!("═══════════════════════════\n");
    
    let bytes = serialization::to_json_vec(&original_game)?;
    println!("📊 Byte representation: {} bytes", bytes.len());
    println!("🎯 First 50 bytes: {:?}", &bytes[..50.min(bytes.len())]);
    println!();
    
    let from_bytes = serialization::from_json_slice(&bytes)?;
    println!("✅ Successfully deserialized from bytes!");
    println!("   Game Name: {}", from_bytes.name);
    println!("   Genre: {:?}", from_bytes.genre);
    
    println!();
    println!("🎯 Serialization Example Complete!");
    println!("📈 Key Takeaways:");
    println!("   • GameDNA serializes deterministically (same input → same output)");
    println!("   • Round-trips preserve all field values exactly");
    println!("   • JSON format is human-readable and machine-parseable");
    println!("   • Custom properties and tags are preserved");
    println!("   • Version information is maintained");
    
    Ok(())
}