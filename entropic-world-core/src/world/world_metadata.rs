use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemanticVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }

        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;

        Some(Self::new(major, minor, patch))
    }
}

impl Default for SemanticVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldMetadata {
    pub id: String,
    pub name: String,
    pub game_dna_id: String,
    pub version: SemanticVersion,
    pub created_at: DateTime<Utc>,
    pub last_simulated: DateTime<Utc>,
    pub description: Option<String>,
    pub author: Option<String>,
}

impl WorldMetadata {
    pub fn new(id: String, name: String, game_dna_id: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            game_dna_id,
            version: SemanticVersion::default(),
            created_at: now,
            last_simulated: now,
            description: None,
            author: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_version() {
        let version = SemanticVersion::new(1, 2, 3);
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_version_from_string() {
        let version = SemanticVersion::from_string("2.5.1").unwrap();
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 5);
        assert_eq!(version.patch, 1);
    }

    #[test]
    fn test_metadata_creation() {
        let metadata = WorldMetadata::new(
            "world_1".to_string(),
            "Test World".to_string(),
            "game_dna_1".to_string(),
        );
        assert_eq!(metadata.name, "Test World");
        assert_eq!(metadata.version, SemanticVersion::default());
    }
}
