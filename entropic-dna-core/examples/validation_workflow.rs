//! Example validation workflow for Game DNA configurations
//!
//! This example demonstrates the complete validation workflow:
//! 1. Build an incomplete config
//! 2. Detect validation errors
//! 3. Apply suggestions
//! 4. Publish locked config
//! 5. Verify integrity

use entropic_dna_core::{
    schema::{GameDNA, Genre, CameraMode, Tone, WorldScale, TargetPlatform, MonetizationModel, PhysicsProfile, DifficultyMode},
    validation::{ValidationEngine, ValidatedGameDNABuilder, LockedGameDNABuilder},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Game DNA Validation Workflow Example ===\n");

    // Step 1: Build an incomplete/invalid configuration
    println!("Step 1: Building initial configuration...");
    let mut builder = GameDNA::builder()
        .name("My Awesome Game".to_string())
        .genre(Genre::FPS)
        .camera(CameraMode::Isometric)  // This will cause an error for FPS
        .tone(Tone::Cinematic)
        .world_scale(WorldScale::OpenWorld)
        .target_platforms(vec![TargetPlatform::PC, TargetPlatform::Mobile])  // OpenWorld on Mobile is challenging
        .physics_profile(PhysicsProfile::Arcade)
        .max_players(1)
        .is_competitive(true)  // Competitive but single player
        .supports_coop(false)
        .difficulty(DifficultyMode::Medium)
        .monetization(MonetizationModel::FreeToPlay)
        .target_fps(144)
        .max_draw_distance(3000.0)
        .max_entities(5000)
        .max_npc_count(100)
        .time_scale(1.0)
        .weather_enabled(true)
        .seasons_enabled(true)
        .day_night_cycle(true)
        .persistent_world(false)
        .npc_count(50)
        .ai_enabled(true)
        .ai_difficulty_scaling(false)
        .has_campaign(true)
        .has_side_quests(false)
        .dynamic_quests(false);

    let initial_game = builder.build()?;
    println!("✓ Initial configuration built\n");

    // Step 2: Validate the configuration
    println!("Step 2: Validating configuration...");
    let validation_engine = ValidationEngine::new();
    let validation_result = validation_engine.validate(&initial_game);
    
    println!("Validation result: {} valid", if validation_result.is_valid { "✓" } else { "✗" });
    
    if !validation_result.errors.is_empty() {
        println!("\nErrors found ({}):", validation_result.errors.len());
        for error in &validation_result.errors {
            println!("  - {}: {}", error.code, error.message);
            println!("    Field: {}", error.field);
            println!("    Details: {}", error.details);
        }
    }
    
    if !validation_result.warnings.is_empty() {
        println!("\nWarnings found ({}):", validation_result.warnings.len());
        for warning in &validation_result.warnings {
            println!("  - {}: {}", warning.code, warning.message);
            println!("    Field: {}", warning.field);
            println!("    Suggestion: {}", warning.suggestion);
        }
    }
    
    if !validation_result.suggestions.is_empty() {
        println!("\nSuggestions ({}):", validation_result.suggestions.len());
        for suggestion in &validation_result.suggestions {
            println!("  - {}", suggestion);
        }
    }
    println!();

    // Step 3: Apply suggestions and fix issues
    println!("Step 3: Applying suggestions and fixing issues...");
    
    // Fix the FPS camera issue
    let mut fixed_builder = GameDNA::builder()
        .name("My Awesome Game".to_string())
        .genre(Genre::FPS)
        .camera(CameraMode::Perspective3D)  // Fixed: Use 3D camera for FPS
        .tone(Tone::Cinematic)
        .world_scale(WorldScale::LargeLevel)  // Fixed: OpenWorld on Mobile is too challenging
        .target_platforms(vec![TargetPlatform::PC])
        .physics_profile(PhysicsProfile::SemiRealistic)  // Fixed: Arcade physics for FPS is unusual
        .max_players(12)  // Fixed: Competitive game needs more players
        .is_competitive(true)
        .supports_coop(false)
        .difficulty(DifficultyMode::Dynamic)  // Better for competitive games
        .monetization(MonetizationModel::FreeToPlay)
        .target_fps(144)
        .max_draw_distance(3000.0)
        .max_entities(5000)
        .max_npc_count(100)
        .time_scale(1.0)
        .weather_enabled(true)
        .seasons_enabled(true)
        .day_night_cycle(true)
        .persistent_world(false)
        .npc_count(50)
        .ai_enabled(true)
        .ai_difficulty_scaling(false)
        .has_campaign(true)
        .has_side_quests(false)
        .dynamic_quests(false)
        .tag("multiplayer".to_string())
        .tag("competitive".to_string())
        .tag("fps".to_string());

    let fixed_game = fixed_builder.build()?;
    println!("✓ Configuration fixes applied\n");

    // Step 4: Validate the fixed configuration
    println!("Step 4: Validating fixed configuration...");
    let fixed_validation_result = validation_engine.validate(&fixed_game);
    
    println!("Fixed validation result: {} valid", if fixed_validation_result.is_valid { "✓" } else { "✗" });
    
    if !fixed_validation_result.errors.is_empty() {
        println!("\nErrors found ({}):", fixed_validation_result.errors.len());
        for error in &fixed_validation_result.errors {
            println!("  - {}: {}", error.code, error.message);
        }
    } else {
        println!("✓ No errors found!");
    }
    
    if !fixed_validation_result.warnings.is_empty() {
        println!("\nWarnings found ({}):", fixed_validation_result.warnings.len());
        for warning in &fixed_validation_result.warnings {
            println!("  - {}: {}", warning.code, warning.message);
        }
    } else {
        println!("✓ No warnings found!");
    }
    println!();

    // Step 5: Use the validated builder pattern
    println!("Step 5: Using validated builder pattern...");
    let mut validated_builder = ValidatedGameDNABuilder::new(fixed_game);
    
    println!("Initial validation: {} valid", if validated_builder.is_valid() { "✓" } else { "✗" });
    
    // Validate specific fields
    let field_result = validation_engine.validate_field(validated_builder.game_dna(), "target_fps");
    println!("FPS field validation: {} valid", if field_result.is_valid { "✓" } else { "✗" });
    
    println!();

    // Step 6: Publish as locked configuration
    println!("Step 6: Publishing locked configuration...");
    let locked_builder = LockedGameDNABuilder::new(validated_builder.game_dna().clone());
    let publish_result = locked_builder.publish();
    
    match publish_result {
        Ok(locked_game) => {
            println!("✓ Configuration successfully published!");
            println!("Checksum: {}", locked_game.checksum);
            println!("Lock timestamp: {}", locked_game.lock_timestamp);
            println!("Is locked: {}", locked_game.is_locked);
            
            // Verify integrity
            let integrity_ok = locked_game.verify_integrity();
            println!("Integrity verification: {}", if integrity_ok { "✓ PASS" } else { "✗ FAIL" });
            
            // Try to access config (should be None since it's locked)
            let config_access = locked_game.get_config();
            println!("Config access while locked: {}", if config_access.is_some() { "ALLOWED" } else { "BLOCKED" });
            
            // Unlock and try again
            let mut unlocked_game = locked_game;
            unlocked_game.unlock();
            let config_access_after_unlock = unlocked_game.get_config();
            println!("Config access after unlock: {}", if config_access_after_unlock.is_some() { "ALLOWED" } else { "BLOCKED" });
        }
        Err(validation_result) => {
            println!("✗ Publishing failed due to validation errors:");
            for error in &validation_result.errors {
                println!("  - {}: {}", error.code, error.message);
            }
        }
    }
    
    println!("\n=== Validation Workflow Complete ===");
    
    Ok(())
}