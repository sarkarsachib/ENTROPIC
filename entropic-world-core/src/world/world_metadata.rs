use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemanticVersion {
    /// Creates a semantic version from its numeric components.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = SemanticVersion::new(1, 2, 3);
    /// assert_eq!(v.major, 1);
    /// assert_eq!(v.minor, 2);
    /// assert_eq!(v.patch, 3);
    /// assert_eq!(v.to_string(), "1.2.3");
    /// ```
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Format the semantic version as "major.minor.patch".
    ///
    /// Returns a `String` containing the major, minor, and patch components separated by dots (e.g., `"1.2.3"`).
    ///
    /// # Examples
    ///
    /// ```
    /// let v = SemanticVersion::new(1, 2, 3);
    /// assert_eq!(v.to_string(), "1.2.3");
    /// ```
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    /// Parses a semantic version string in `major.minor.patch` format into a `SemanticVersion`.
    ///
    /// Returns `Some(SemanticVersion)` if the input contains exactly three dot-separated numeric components,
    /// `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = SemanticVersion::from_string("2.5.1").unwrap();
    /// assert_eq!(v.major, 2);
    /// assert_eq!(v.minor, 5);
    /// assert_eq!(v.patch, 1);
    /// ```
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
    /// Creates the default semantic version 1.0.0.
    ///
    /// # Examples
    ///
    /// ```
    /// let v = SemanticVersion::default();
    /// assert_eq!(v.major, 1);
    /// assert_eq!(v.minor, 0);
    /// assert_eq!(v.patch, 0);
    /// assert_eq!(v.to_string(), "1.0.0");
    /// ```
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
    /// Creates a new WorldMetadata with the given `id`, `name`, and `game_dna_id`.
    ///
    /// The returned metadata uses `SemanticVersion::default()` for `version`, sets `created_at` and
    /// `last_simulated` to the current UTC time, and initializes `description` and `author` to `None`.
    ///
    /// # Parameters
    /// - `id`: Unique identifier for the world.
    /// - `name`: Display name of the world.
    /// - `game_dna_id`: Identifier of the game's DNA associated with the world.
    ///
    /// # Examples
    ///
    /// ```
    /// let meta = WorldMetadata::new("world-1".into(), "My World".into(), "game-dna-xyz".into());
    /// assert_eq!(meta.name, "My World");
    /// assert_eq!(meta.version, SemanticVersion::default());
    /// ```
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

    /// Sets the description on the WorldMetadata and returns the modified value for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// let meta = WorldMetadata::new("id".into(), "name".into(), "dna".into())
    ///     .with_description("A tiny world".into());
    /// assert_eq!(meta.description.as_deref(), Some("A tiny world"));
    /// ```
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the author field of the metadata.
    ///
    /// Consumes `self` and returns the updated `WorldMetadata` with `author` set, allowing method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// let meta = WorldMetadata::new("id".into(), "name".into(), "dna".into())
    ///     .with_author("Alice".into());
    /// assert_eq!(meta.author.unwrap(), "Alice");
    /// ```
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