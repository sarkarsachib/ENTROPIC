//! Version management for Game DNA schema
//! 
//! Provides schema versioning, compatibility checking, and migration framework
//! for evolving the GameDNA schema over time.

use crate::{errors::VersionError, GameDNA};

/// Current schema version
pub const CURRENT_VERSION: &str = "0.1.0";

/// Minimum version that the current implementation can read
pub const MINIMUM_COMPATIBLE_VERSION: &str = "0.1.0";

/// GameDNA wrapper that includes version information for serialization
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct WrappedGameDNA {
    #[serde(flatten)]
    pub(super) dna: GameDNA,
    #[serde(default)]
    pub(super) schema_version: String,
}

impl WrappedGameDNA {
    pub fn new(dna: GameDNA) -> Self {
        Self {
            dna,
            schema_version: CURRENT_VERSION.to_string(),
        }
    }
    
    pub fn validate(self) -> Result<GameDNA, crate::errors::SerializationError> {
        let version = self.schema_version.as_str();
        let version_manager = VersionManager::new();
        
        if version_manager.is_compatible(version) {
            Ok(self.dna)
        } else {
            Err(self.create_version_error(version, version_manager))
        }
    }
    
    fn create_version_error(
        &self,
        version: &str,
        manager: VersionManager,
    ) -> crate::errors::SerializationError {
        let latest_compatible = manager.latest_compatible_version();
        crate::errors::SerializationError::JsonDeserialization {
            reason: format!(
                "Incompatible schema version: {} (current: {}). \
                 Use a newer version of entropic-dna-core (>= {}) to read this GameDNA.",
                version, CURRENT_VERSION, latest_compatible
            ),
        }
    }
}

/// Manages schema versions and migrations
pub struct VersionManager {
    from_version: SemanticVersion,
}

impl VersionManager {
    /// Creates a new version manager for the current version
    pub fn new() -> Self {
        Self {
            from_version: CURRENT_VERSION.parse()
                .unwrap_or_else(|_| SemanticVersion::new(0, 1, 0)),
        }
    }
    
    /// Checks if a version is compatible with the current version
    pub fn is_compatible(&self, version: &str) -> bool {
        version == CURRENT_VERSION
    }
    
    /// Gets the latest version that this implementation can read
    pub fn latest_compatible_version(&self) -> &str {
        CURRENT_VERSION
    }
    
    /// Checks if a GameDNA instance is using the current schema version
    pub fn check_schema_version(&self, dna: &GameDNA) -> Result<(), VersionError> {
        let schema_version = dna.version.to_string();
        
        if schema_version == CURRENT_VERSION {
            Ok(())
        } else {
            Err(VersionError::VersionMismatch {
                current_version: CURRENT_VERSION.to_string(),
                target_version: schema_version.clone(),
                help: format!("Please upgrade your entropic-dna-core library to version {schema_version} to work with this GameDNA. Current version: {CURRENT_VERSION}")
            })
        }
    }
    
    /// Validates that a version string is properly formatted
    pub fn validate_version_format(version: &str) -> Result<(), VersionError> {
        let parts: Vec<&str> = version.split('.').collect();
        
        if parts.len() != 3 {
            return Err(VersionError::InvalidVersionFormat {
                version: version.to_string(),
                reason: "Version must have exactly three parts (MAJOR.MINOR.PATCH)".to_string(),
                expected_format: "MAJOR.MINOR.PATCH where each part is a non-negative integer".to_string(),
            });
        }
        
        for (i, part) in parts.iter().enumerate() {
            if part.parse::<u32>().is_err() {
                let part_name = match i {
                    0 => "MAJOR",
                    1 => "MINOR",
                    _ => "PATCH",
                };
                return Err(VersionError::InvalidVersionFormat {
                    version: version.to_string(),
                    reason: format!("{part_name} version must be a non-negative integer"),
                    expected_format: "MAJOR.MINOR.PATCH where each part is a non-negative integer".to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Compares two version strings
    /// 
    /// # Returns
    /// 
    /// * `Ok(Ordering)` - Comparison result
    /// * `Err(VersionError)` - If version format is invalid
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use entropic_dna_core::version::VersionManager;
    /// use std::cmp::Ordering;
    /// 
    /// let manager = VersionManager::new();
    /// assert_eq!(manager.compare_versions("0.2.0", "0.1.0").unwrap(), Ordering::Greater);
    /// assert_eq!(manager.compare_versions("0.1.0", "0.2.0").unwrap(), Ordering::Less);
    /// assert_eq!(manager.compare_versions("0.1.0", "0.1.0").unwrap(), Ordering::Equal);
    /// ```
    pub fn compare_versions(&self, v1: &str, v2: &str) -> Result<std::cmp::Ordering, VersionError> {
        Self::validate_version_format(v1)?;
        Self::validate_version_format(v2)?;
        
        let v1_parts: Vec<u32> = v1.split('.').map(|p| p.parse().unwrap()).collect();
        let v2_parts: Vec<u32> = v2.split('.').map(|p| p.parse().unwrap()).collect();
        
        // Compare major version
        match v1_parts[0].cmp(&v2_parts[0]) {
            std::cmp::Ordering::Equal => {},
            other => return Ok(other),
        }
        
        // Compare minor version
        match v1_parts[1].cmp(&v2_parts[1]) {
            std::cmp::Ordering::Equal => {},
            other => return Ok(other),
        }
        
        // Compare patch version
        Ok(v1_parts[2].cmp(&v2_parts[2]))
    }
    
    /// Checks if upgrading from one version to another would be a breaking change
    /// 
    /// In semantic versioning:
    /// - MAJOR version changes are breaking
    /// - MINOR and PATCH changes are non-breaking
    pub fn is_breaking_change(&self, from: &str, to: &str) -> Result<bool, VersionError> {
        let ordering = self.compare_versions(from, to)?;
        
        match ordering {
            std::cmp::Ordering::Less => {
                // Upgrade case
                let from_major = from.split('.').next().unwrap().parse::<u32>().unwrap();
                let to_major = to.split('.').next().unwrap().parse::<u32>().unwrap();
                Ok(from_major < to_major)
            },
            std::cmp::Ordering::Greater => {
                // Downgrade
                Err(VersionError::VersionDowngrade {
                    from: from.to_string(),
                    to: to.to_string(),
                })
            },
            std::cmp::Ordering::Equal => Ok(false), // Same version, no change
        }
    }
}

impl Default for VersionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for migrating GameDNA between schema versions
/// 
/// Implement this trait to provide custom migrations when the schema evolves.
/// Each migration should handle converting a GameDNA from one version to another.
pub trait Migration: Send + Sync {
    /// Source version for this migration
    fn from_version(&self) -> &str;
    
    /// Target version for this migration
    fn to_version(&self) -> &str;
    
    /// Performs the migration
    fn migrate(&self, dna: GameDNA) -> Result<GameDNA, VersionError>;
}

/// Manages a collection of migrations
pub struct MigrationManager {
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationManager {
    /// Creates a new migration manager
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }
    
    /// Adds a migration to the manager
    pub fn add_migration<M: Migration + 'static>(&mut self, migration: M) {
        self.migrations.push(Box::new(migration));
    }
    
    /// Finds a migration path from one version to another
    pub fn find_migration_path(&self, from: &str, to: &str) -> Option<Vec<&dyn Migration>> {
        // Simple implementation for direct migrations
        self.migrations
            .iter()
            .find(|m| m.from_version() == from && m.to_version() == to)
            .map(|m| vec![m.as_ref()])
    }
    
    /// Checks if a direct migration exists between versions
    pub fn has_migration(&self, from: &str, to: &str) -> bool {
        self.migrations
            .iter()
            .any(|m| m.from_version() == from && m.to_version() == to)
    }
}

impl Default for MigrationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_constants() {
        assert_eq!(CURRENT_VERSION, "0.1.0");
        assert_eq!(MINIMUM_COMPATIBLE_VERSION, "0.1.0");
    }
    
    #[test]
    fn test_version_manager_creation() {
        let manager = VersionManager::new();
        assert!(manager.is_compatible(CURRENT_VERSION));
        assert!(!manager.is_compatible("0.0.1"));
    }
    
    #[test]
    fn test_latest_compatible_version() {
        let manager = VersionManager::new();
        assert_eq!(manager.latest_compatible_version(), CURRENT_VERSION);
    }
    
    #[test]
    fn test_validate_version_format() {
        assert!(VersionManager::validate_version_format("1.2.3").is_ok());
        assert!(VersionManager::validate_version_format("0.1.0").is_ok());
        assert!(VersionManager::validate_version_format("10.20.30").is_ok());
        
        assert!(VersionManager::validate_version_format("1.2").is_err());
        assert!(VersionManager::validate_version_format("1.2.3.4").is_err());
        assert!(VersionManager::validate_version_format("a.b.c").is_err());
        assert!(VersionManager::validate_version_format("").is_err());
        assert!(VersionManager::validate_version_format("1.2.x").is_err());
    }
    
    #[test]
    fn test_compare_versions() {
        let manager = VersionManager::new();
        
        use std::cmp::Ordering;
        
        assert_eq!(manager.compare_versions("1.0.0", "1.0.0").unwrap(), Ordering::Equal);
        assert_eq!(manager.compare_versions("1.1.0", "1.0.0").unwrap(), Ordering::Greater);
        assert_eq!(manager.compare_versions("1.0.0", "1.1.0").unwrap(), Ordering::Less);
        assert_eq!(manager.compare_versions("2.0.0", "1.9.9").unwrap(), Ordering::Greater);
        assert_eq!(manager.compare_versions("0.1.0", "0.2.0").unwrap(), Ordering::Less);
    }
    
    #[test]
    fn test_is_breaking_change() {
        let manager = VersionManager::new();
        
        // Major version upgrade is breaking
        assert!(manager.is_breaking_change("1.0.0", "2.0.0").unwrap());
        
        // Minor version upgrade is not breaking
        assert!(!manager.is_breaking_change("1.0.0", "1.1.0").unwrap());
        
        // Patch version upgrade is not breaking
        assert!(!manager.is_breaking_change("1.0.0", "1.0.1").unwrap());
        
        // Downgrade should fail
        assert!(manager.is_breaking_change("2.0.0", "1.0.0").is_err());
        
        // Same version is not breaking
        assert!(!manager.is_breaking_change("1.0.0", "1.0.0").unwrap());
    }
    
    #[test]
    fn test_migration_manager() {
        let mut manager = MigrationManager::new();
        
        assert!(!manager.has_migration("0.1.0", "0.2.0"));
        
        // Add a test migration
        struct TestMigration;
        impl Migration for TestMigration {
            fn from_version(&self) -> &str { "0.1.0" }
            fn to_version(&self) -> &str { "0.2.0" }
            fn migrate(&self, dna: GameDNA) -> Result<GameDNA, VersionError> {
                Ok(dna)
            }
        }
        
        manager.add_migration(TestMigration);
        assert!(manager.has_migration("0.1.0", "0.2.0"));
        
        let path = manager.find_migration_path("0.1.0", "0.2.0");
        assert!(path.is_some());
        assert_eq!(path.unwrap().len(), 1);
    }
}