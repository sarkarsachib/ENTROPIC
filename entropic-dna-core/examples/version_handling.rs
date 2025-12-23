//! Example: Schema Version Handling
//!
//! This example demonstrates how to work with GameDNA schema versions,
//! check compatibility, validate version formats, and understand the
//! versioning system.

use entropic_dna_core::{
    GameDNA,
    schema::{Genre, TargetPlatform},
    version::{VersionManager, CURRENT_VERSION, validate_version_format},
};
use std::cmp::Ordering;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔖 Game DNA Schema Version Handling\n");

    // Create a version manager
    let version_manager = VersionManager::new();
    
    println!("📊 Current Library Information:");
    println!("═════════════════════════════════\n");
    println!("   Current Schema Version: {}", CURRENT_VERSION);
    println!("   Minimum Compatible Version: {}", entropic_dna_core::MINIMUM_COMPATIBLE_VERSION);
    println!("   Latest Compatible Version: {}", version_manager.latest_compatible_version());
    println!();

    // Test version compatibility
    println!("🔍 Version Compatibility Tests:");
    println!("═════════════════════════════════\n");
    
    let test_versions = vec![
        CURRENT_VERSION,
        "0.1.0",
        "0.2.0",
        "1.0.0",
        "1.2.3",
        "2.0.0",
    ];
    
    for version in test_versions {
        let compatible = version_manager.is_compatible(version);
        let status = if compatible { "✅" } else { "❌" };
        println!("   {} Version {}: {}", status, version, if compatible { "COMPATIBLE" } else { "NOT COMPATIBLE" });
    }
    println!();

    // Demonstrate version format validation
    println!("✅ Version Format Validation:");
    println!("═══════════════════════════════\n");
    
    let valid_versions = vec![
        "0.1.0",
        "1.2.3",
        "10.20.30",
        "0.0.1",
    ];
    
    let invalid_versions = vec![
        ("1.2", "Missing patch version"),
        ("1.2.3.4", "Too many parts"),
        ("a.b.c", "Non-numeric parts"),
        ("", "Empty string"),
        ("1.2.x", "Invalid patch version"),
        ("v1.2.3", "Contains 'v' prefix"),
    ];
    
    println!("   Valid Versions:");
    for version in valid_versions {
        match validate_version_format(version) {
            Ok(_) => println!("      ✅ {}: Valid", version),
            Err(e) => println!("      ❌ {}: Error - {}", version, e),
        }
    }
    
    println!("\n   Invalid Versions:");
    for (version, reason) in invalid_versions {
        match validate_version_format(version) {
            Ok(_) => println!("      ⚠️  {}: Unexpectedly valid!", version),
            Err(_) => println!("      ✅ {}: Correctly rejected ({})", version, reason),
        }
    }
    println!();

    // Compare versions
    println!("⚖️  Version Comparison:");
    println!("════════════════════════\n");
    
    let comparisons = vec![
        ("1.0.0", "1.0.0", "Equal"),
        ("1.1.0", "1.0.0", "Greater (minor)"),
        ("2.0.0", "1.9.9", "Greater (major)"),
        ("1.0.0", "1.1.0", "Less (minor)"),
        ("0.9.0", "1.0.0", "Less (major)"),
        ("1.0.1", "1.0.0", "Greater (patch)"),
    ];
    
    for (v1, v2, expected) in comparisons {
        let result = version_manager.compare_versions(v1, v2)?;
        let symbol = match result {
            Ordering::Less => "<",
            Ordering::Equal => "=",
            Ordering::Greater => ">",
        };
        println!("   {} {} {} ({})", v1, symbol, v2, expected);
    }
    println!();

    // Check for breaking changes
    println!("💥 Breaking Change Detection:");
    println!("═══════════════════════════════\n");
    
    let breaking_tests = vec![
        ("1.0.0", "2.0.0", true, "Major version upgrade"),
        ("1.0.0", "1.1.0", false, "Minor version upgrade"),
        ("1.0.0", "1.0.1", false, "Patch version upgrade"),
        ("0.1.0", "0.2.0", false, "Minor upgrade (0.x.x)"),
        ("2.1.0", "2.1.5", false, "Patch upgrade"),
    ];
    
    for (from, to, expected_breaking, description) in breaking_tests {
        match version_manager.is_breaking_change(from, to) {
            Ok(is_breaking) => {
                let status = if is_breaking == expected_breaking { "✅" } else { "❌" };
                let breaking_status = if is_breaking { "BREAKING" } else { "NON-BREAKING" };
                println!("   {} {} → {}: {} ({})", status, from, to, breaking_status, description);
            },
            Err(e) => {
                println!("   ❌ {} → {}: Error - {}", from, to, e);
            }
        }
    }
    println!();

    // Create a GameDNA and check its version
    println!("🎮 GameDNA Version Information:");
    println!("═════════════════════════════════\n");
    
    let game = GameDNA::builder()
        .name("Version Test Game".to_string())
        .genre(Genre::Puzzle)
        .target_platforms(vec![TargetPlatform::Mobile, TargetPlatform::PC])
        .build()?;
    
    println!("   Game Name: {}", game.name);
    println!("   Game Version: {}", game.version);
    println!("   Schema Version: {}", CURRENT_VERSION);
    println!();
    
    // Check if the GameDNA is compatible with current schema
    match version_manager.check_schema_version(&game) {
        Ok(_) => println!("   ✅ GameDNA schema version is compatible"),
        Err(e) => println!("   ❌ Schema version mismatch: {}", e),
    }
    println!();

    // Demonstrate version downgrade prevention
    println!("🛡️  Downgrade Prevention:");
    println!("══════════════════════════\n");
    
    let downgrade_tests = vec![
        ("1.0.0", "1.0.0", "Same version - allowed"),
        ("2.0.0", "1.0.0", "Downgrade - blocked"),
        ("1.5.0", "1.4.0", "Minor downgrade - blocked"),
        ("1.0.0", "2.0.0", "Upgrade - allowed"),
    ];
    
    for (from, to, description) in downgrade_tests {
        match version_manager.is_breaking_change(from, to) {
            Ok(_) => println!("   ✅ {} → {}: {}", from, to, description),
            Err(e) => println!("   🛡️  {} → {}: Blocked - {}", from, to, description),
        }
    }
    println!();

    // Version best practices
    println!("📋 Version Management Best Practices:");
    println!("═══════════════════════════════════════\n");
    println!("   1. ✅ Always use semantic versioning (MAJOR.MINOR.PATCH)");
    println!("   2. ✅ Increment MAJOR for breaking changes");
    println!("   3. ✅ Increment MINOR for new features (backward compatible)");
    println!("   4. ✅ Increment PATCH for bug fixes (backward compatible)");
    println!("   5. ✅ Validate versions before processing GameDNA");
    println!("   6. ✅ Check compatibility before attempting to load GameDNA");
    println!("   7. ✅ Store schema version with GameDNA for future compatibility");
    println!("   8. ✅ Test migrations thoroughly when upgrading versions");
    println!();
    
    println!("   Current Schema Version: {}", CURRENT_VERSION);
    println!("   This represents the foundation of the ENTROPIC Game DNA system.");
    println!("   Future versions will provide migration paths for smooth upgrades.");
    
    println!();
    println!("🎯 Version Handling Example Complete!");
    
    Ok(())
}