//! Example: Creating a simple 3D action RPG
//!
//! This example demonstrates how to create a comprehensive GameDNA configuration
//! for a 3D action RPG game using the builder pattern.

use entropic_dna_core::{
    GameDNA,
    schema::{
        Genre, CameraMode, Tone, WorldScale, TargetPlatform,
        MonetizationModel, PhysicsProfile, DifficultyMode,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Creating a 3D Action RPG Game DNA...\n");

    // Create a comprehensive 3D action RPG configuration
    let game = GameDNA::builder()
        .name("Chronicles of the Lost Realm".to_string())
        .genre(Genre::RPG)
        .camera(CameraMode::Perspective3D)
        .tone(Tone::Cinematic)
        .world_scale(WorldScale::OpenWorld)
        .target_platforms(vec![TargetPlatform::PC, TargetPlatform::Console])
        .physics_profile(PhysicsProfile::SemiRealistic)
        .max_players(1)
        .is_competitive(false)
        .supports_coop(false)
        .difficulty(DifficultyMode::Dynamic)
        .monetization(MonetizationModel::PremiumBuy)
        .target_audience("Teen to Adult".to_string())
        .esrb_rating(Some("T".to_string()))
        .target_fps(60)
        .max_draw_distance(3000.0)
        .max_entities(5000)
        .max_npc_count(200)
        .time_scale(1.0)
        .weather_enabled(true)
        .seasons_enabled(true)
        .day_night_cycle(true)
        .persistent_world(true)
        .npc_count(150)
        .ai_enabled(true)
        .ai_difficulty_scaling(true)
        .has_campaign(true)
        .has_side_quests(true)
        .dynamic_quests(true)
        .tag("fantasy".to_string())
        .tag("open-world".to_string())
        .tag("story-driven".to_string())
        .custom_property("engine", "entropic-v1")
        .custom_property("has_magic_system", "true")
        .custom_property("mount_count", "12")
        .build()?;

    // Display game information
    println!("✅ Game DNA Created Successfully!");
    println!("═══════════════════════════════════\n");
    
    println!("🎮 Game Information:");
    println!("   Name: {}", game.name);
    println!("   ID: {}", game.id);
    println!("   Version: {}", game.version);
    println!("   Genre: {:?}", game.genre);
    println!("   Camera: {:?}", game.camera);
    println!("   Tone: {:?}", game.tone);
    println!("   World Scale: {:?}", game.world_scale);
    
    println!("\n🎯 Target Configuration:");
    println!("   Platforms: {:?}", game.target_platforms);
    println!("   FPS Target: {}", game.target_fps);
    println!("   Max Entities: {}", game.max_entities);
    println!("   Max NPCs: {}", game.max_npc_count);
    println!("   Draw Distance: {:.0}m", game.max_draw_distance);
    
    println!("\n🕹️  Gameplay Features:");
    println!("   Physics: {:?}", game.physics_profile);
    println!("   Difficulty: {:?}", game.difficulty);
    println!("   Monetization: {:?}", game.monetization);
    println!("   Co-op Support: {}", game.supports_coop);
    println!("   Max Players: {}", game.max_players);
    
    println!("\n🌎 World Simulation:");
    println!("   Weather: {}", game.weather_enabled);
    println!("   Seasons: {}", game.seasons_enabled);
    println!("   Day/Night Cycle: {}", game.day_night_cycle);
    println!("   Persistent World: {}", game.persistent_world);
    println!("   Time Scale: {}", game.time_scale);
    
    println!("\n🤖 AI & NPCs:");
    println!("   AI Enabled: {}", game.ai_enabled);
    println!("   AI Difficulty Scaling: {}", game.ai_difficulty_scaling);
    println!("   NPC Count: {}", game.npc_count);
    
    println!("\n📖 Narrative:");
    println!("   Campaign: {}", game.has_campaign);
    println!("   Side Quests: {}", game.has_side_quests);
    println!("   Dynamic Quests: {}", game.dynamic_quests);
    
    println!("\n🏷️  Tags: {}", game.tags.join(", "));
    
    println!("\n🔧 Custom Properties:");
    for (key, value) in &game.custom_properties {
        println!("   {}: {}", key, value);
    }
    
    println!("\n✨ Game DNA Created Successfully!");
    
    Ok(())
}