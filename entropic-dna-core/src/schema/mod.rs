//! Core Game DNA schema types and enums
//! 
//! This module defines the canonical structure for representing game configurations
//! in the ENTROPIC game engine. All game types are type-safe and deterministic.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the genre classification of a game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Genre {
    /// First Person Shooter
    FPS,
    /// Role Playing Game
    RPG,
    /// Third Person Shooter
    TPS,
    /// Strategy game
    Strategy,
    /// Casual game
    Casual,
    /// Horror game
    Horror,
    /// Racing game
    Racing,
    /// Simulation game
    Simulation,
    /// Puzzle game
    Puzzle,
    /// Educational game
    Educational,
    /// Custom genre with descriptor
    #[serde(rename = "Custom")]
    CustomGenre(String),
}

/// Represents the camera perspective/view mode
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CameraMode {
    /// 2D perspective
    Perspective2D,
    /// 2.5D perspective (2D with depth)
    Perspective2_5D,
    /// 3D perspective
    Perspective3D,
    /// Isometric view
    Isometric,
    /// Virtual Reality
    #[serde(rename = "VR")]
    VR,
    /// Custom camera mode
    #[serde(rename = "Custom")]
    CustomCamera(String),
}

/// Represents the visual and gameplay tone/style
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tone {
    /// Realistic tone
    Realistic,
    /// Arcade-style tone
    Arcade,
    /// Cinematic presentation
    Cinematic,
    /// Stylized visuals
    Stylized,
    /// Minimalist design
    Minimalist,
    /// Custom tone
    #[serde(rename = "Custom")]
    CustomTone(String),
}

/// Represents the scale of the game world
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldScale {
    /// Small enclosed level
    TinyLevel,
    /// Small level
    SmallLevel,
    /// Medium-sized level
    MediumLevel,
    /// Large level
    LargeLevel,
    /// Open world environment
    OpenWorld,
    /// Planet-scale world
    Planet,
    /// Galaxy-scale world
    Galaxy,
    /// Custom world scale
    #[serde(rename = "Custom")]
    CustomScale(String),
}

/// Represents target platforms for the game
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetPlatform {
    /// Mobile devices (iOS/Android)
    Mobile,
    /// Personal computers (Windows/Linux/Mac)
    PC,
    /// Console platforms (PlayStation, Xbox, Switch)
    Console,
    /// Extended Reality (AR/VR/MR)
    XR,
    /// Cloud-streamed game
    CloudStreamed,
    /// Multi-platform release
    MultiPlatform,
}

/// Represents the game's monetization model
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonetizationModel {
    /// Free to play with optional purchases
    #[serde(rename = "FreeToPlay")]
    FreeToPlay,
    /// Premium purchase required
    #[serde(rename = "PremiumBuy")]
    PremiumBuy,
    /// Subscription-based access
    Subscription,
    /// One-time purchase
    #[serde(rename = "OneTimePay")]
    OneTimePay,
    /// Hybrid monetization model
    Hybrid,
    /// Custom monetization model
    #[serde(rename = "Custom")]
    Custom(String),
}

/// Represents physics simulation profile
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhysicsProfile {
    /// Arcade-style physics (fast, forgiving)
    Arcade,
    /// Semi-realistic physics
    SemiRealistic,
    /// Realistic physics simulation
    Realistic,
    /// Custom physics profile
    #[serde(rename = "Custom")]
    CustomPhysics(String),
}

/// Represents difficulty configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DifficultyMode {
    /// Easy difficulty setting
    Easy,
    /// Medium difficulty setting
    Medium,
    /// Hard difficulty setting
    Hard,
    /// Dynamic/adaptive difficulty
    Dynamic,
    /// Custom difficulty configuration
    #[serde(rename = "Custom")]
    CustomDifficulty(String),
}

/// Represents a semantic version (MAJOR.MINOR.PATCH)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticVersion {
    /// Major version (breaking changes)
    pub major: u32,
    /// Minor version (new features, backward compatible)
    pub minor: u32,
    /// Patch version (bug fixes, backward compatible)
    pub patch: u32,
}

impl SemanticVersion {
    /// Creates a new semantic version
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
    
    /// Default version for new Game DNA
    pub fn default_version() -> Self {
        Self::new(0, 1, 0)
    }
}

impl std::fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// The core Game DNA struct representing complete game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDNA {
    /// Unique identifier (UUID)
    pub id: String,
    /// Game name
    pub name: String,
    /// Semantic version
    pub version: SemanticVersion,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modification timestamp
    pub last_modified: chrono::DateTime<chrono::Utc>,
    
    /// Core configuration
    /// Game genre
    pub genre: Genre,
    /// Camera perspective mode
    pub camera: CameraMode,
    /// Visual and gameplay tone
    pub tone: Tone,
    /// World scale/size
    pub world_scale: WorldScale,
    /// Target platforms
    pub target_platforms: Vec<TargetPlatform>,
    
    /// Gameplay configuration
    /// Physics simulation profile
    pub physics_profile: PhysicsProfile,
    /// Maximum number of supported players
    pub max_players: u32,
    /// Whether the game supports competitive play
    pub is_competitive: bool,
    /// Whether the game supports cooperative play
    pub supports_coop: bool,
    /// Default difficulty mode
    pub difficulty: DifficultyMode,
    
    /// Monetization and business model
    /// Monetization strategy
    pub monetization: MonetizationModel,
    /// Target audience description
    pub target_audience: String,
    /// ESRB rating (optional)
    pub esrb_rating: Option<String>,
    
    /// Performance constraints
    /// Target frames per second
    pub target_fps: u32,
    /// Maximum draw distance (in world units)
    pub max_draw_distance: f32,
    /// Maximum number of entities
    pub max_entities: u32,
    /// Maximum NPC count
    pub max_npc_count: u32,
    
    /// World simulation
    /// Time scale (1.0 = real-time)
    pub time_scale: f32,
    /// Whether weather simulation is enabled
    pub weather_enabled: bool,
    /// Whether seasonal cycles are enabled
    pub seasons_enabled: bool,
    /// Whether day/night cycle is enabled
    pub day_night_cycle: bool,
    /// Whether the world is persistent
    pub persistent_world: bool,
    
    /// AI and NPC configuration
    /// Number of NPCs
    pub npc_count: u32,
    /// Whether AI is enabled
    pub ai_enabled: bool,
    /// Whether AI difficulty scales
    pub ai_difficulty_scaling: bool,
    
    /// Narrative features
    /// Whether the game has a campaign/story mode
    pub has_campaign: bool,
    /// Whether the game has side quests
    pub has_side_quests: bool,
    /// Whether dynamic quests are enabled
    pub dynamic_quests: bool,
    
    /// Metadata and extensibility
    /// Descriptive tags
    pub tags: Vec<String>,
    /// Custom properties for engine-specific extensions
    pub custom_properties: HashMap<String, String>,
}

impl GameDNA {
    /// Creates a new GameDNA instance builder with default values
    pub fn builder() -> GameDNABuilder {
        GameDNABuilder::new()
    }
    
    /// Creates a minimal valid GameDNA instance with required fields
    pub fn minimal(name: String, genre: Genre, platforms: Vec<TargetPlatform>) -> Self {
        Self::builder()
            .name(name)
            .genre(genre)
            .target_platforms(platforms)
            .build()
            .expect("Minimal GameDNA should be valid")
    }
    
    /// Validates the GameDNA configuration
    pub fn validate(&self) -> Result<(), crate::errors::SchemaError> {
        if self.name.is_empty() {
            return Err(crate::errors::SchemaError::invalid_field(
                "name".to_string(),
                "Game name cannot be empty".to_string()
            ));
        }
        
        if self.id.is_empty() {
            return Err(crate::errors::SchemaError::invalid_field(
                "id".to_string(),
                "Game ID cannot be empty".to_string()
            ));
        }
        
        if self.target_platforms.is_empty() {
            return Err(crate::errors::SchemaError::invalid_field(
                "target_platforms".to_string(),
                "At least one target platform must be specified".to_string()
            ));
        }
        
        if self.target_fps == 0 || self.target_fps > 1000 {
            return Err(crate::errors::SchemaError::invalid_field(
                "target_fps".to_string(),
                "Target FPS must be between 1 and 1000".to_string()
            ));
        }
        
        if self.time_scale <= 0.0 || self.time_scale > 1000.0 {
            return Err(crate::errors::SchemaError::invalid_field(
                "time_scale".to_string(),
                "Time scale must be positive and reasonable".to_string()
            ));
        }
        
        Ok(())
    }
}

/// Builder for GameDNA with sensible defaults
pub struct GameDNABuilder {
    name: String,
    id: Option<String>,
    version: SemanticVersion,
    genre: Genre,
    camera: CameraMode,
    tone: Tone,
    world_scale: WorldScale,
    target_platforms: Vec<TargetPlatform>,
    physics_profile: PhysicsProfile,
    max_players: u32,
    is_competitive: bool,
    supports_coop: bool,
    difficulty: DifficultyMode,
    monetization: MonetizationModel,
    target_audience: String,
    esrb_rating: Option<String>,
    target_fps: u32,
    max_draw_distance: f32,
    max_entities: u32,
    max_npc_count: u32,
    time_scale: f32,
    weather_enabled: bool,
    seasons_enabled: bool,
    day_night_cycle: bool,
    persistent_world: bool,
    npc_count: u32,
    ai_enabled: bool,
    ai_difficulty_scaling: bool,
    has_campaign: bool,
    has_side_quests: bool,
    dynamic_quests: bool,
    tags: Vec<String>,
    custom_properties: HashMap<String, String>,
}

impl GameDNABuilder {
    /// Creates a new GameDNA builder with default values
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        Self {
            name: String::new(),
            id: None,
            version: SemanticVersion::default_version(),
            genre: Genre::FPS,
            camera: CameraMode::Perspective3D,
            tone: Tone::Realistic,
            world_scale: WorldScale::MediumLevel,
            target_platforms: vec![TargetPlatform::PC],
            physics_profile: PhysicsProfile::SemiRealistic,
            max_players: 1,
            is_competitive: false,
            supports_coop: false,
            difficulty: DifficultyMode::Medium,
            monetization: MonetizationModel::PremiumBuy,
            target_audience: String::from("General Audience"),
            esrb_rating: None,
            target_fps: 60,
            max_draw_distance: 1000.0,
            max_entities: 10000,
            max_npc_count: 1000,
            time_scale: 1.0,
            weather_enabled: false,
            seasons_enabled: false,
            day_night_cycle: false,
            persistent_world: false,
            npc_count: 0,
            ai_enabled: false,
            ai_difficulty_scaling: false,
            has_campaign: false,
            has_side_quests: false,
            dynamic_quests: false,
            tags: Vec::new(),
            custom_properties: HashMap::new(),
        }
    }
    
    /// Sets the game name
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }
    
    /// Sets the game ID (UUID format)
    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }
    
    /// Sets the version
    pub fn version(mut self, version: SemanticVersion) -> Self {
        self.version = version;
        self
    }
    
    /// Sets the genre
    pub fn genre(mut self, genre: Genre) -> Self {
        self.genre = genre;
        self
    }
    
    /// Sets the camera mode
    pub fn camera(mut self, camera: CameraMode) -> Self {
        self.camera = camera;
        self
    }
    
    /// Sets the tone
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }
    
    /// Sets the world scale
    pub fn world_scale(mut self, world_scale: WorldScale) -> Self {
        self.world_scale = world_scale;
        self
    }
    
    /// Sets the target platforms
    pub fn target_platforms(mut self, platforms: Vec<TargetPlatform>) -> Self {
        self.target_platforms = platforms;
        self
    }
    
    /// Adds a single platform to target platforms
    pub fn add_platform(mut self, platform: TargetPlatform) -> Self {
        self.target_platforms.push(platform);
        self
    }
    
    /// Sets the physics profile
    pub fn physics_profile(mut self, physics: PhysicsProfile) -> Self {
        self.physics_profile = physics;
        self
    }
    
    /// Sets the max players
    pub fn max_players(mut self, max_players: u32) -> Self {
        self.max_players = max_players;
        self
    }
    
    /// Sets whether the game is competitive
    pub fn is_competitive(mut self, is_competitive: bool) -> Self {
        self.is_competitive = is_competitive;
        self
    }
    
    /// Sets whether the game supports coop
    pub fn supports_coop(mut self, supports_coop: bool) -> Self {
        self.supports_coop = supports_coop;
        self
    }
    
    /// Sets the difficulty mode
    pub fn difficulty(mut self, difficulty: DifficultyMode) -> Self {
        self.difficulty = difficulty;
        self
    }
    
    /// Sets the monetization model
    pub fn monetization(mut self, monetization: MonetizationModel) -> Self {
        self.monetization = monetization;
        self
    }
    
    /// Sets the target audience
    pub fn target_audience<S: Into<String>>(mut self, audience: S) -> Self {
        self.target_audience = audience.into();
        self
    }
    
    /// Sets the ESRB rating
    pub fn esrb_rating<S: Into<String>>(mut self, rating: S) -> Self {
        self.esrb_rating = Some(rating.into());
        self
    }
    
    /// Sets the target FPS
    pub fn target_fps(mut self, fps: u32) -> Self {
        self.target_fps = fps;
        self
    }
    
    /// Sets the max draw distance
    pub fn max_draw_distance(mut self, distance: f32) -> Self {
        self.max_draw_distance = distance;
        self
    }
    
    /// Sets the max entities
    pub fn max_entities(mut self, entities: u32) -> Self {
        self.max_entities = entities;
        self
    }
    
    /// Sets the max NPC count
    pub fn max_npc_count(mut self, npc_count: u32) -> Self {
        self.max_npc_count = npc_count;
        self
    }
    
    /// Sets the time scale
    pub fn time_scale(mut self, time_scale: f32) -> Self {
        self.time_scale = time_scale;
        self
    }
    
    /// Sets whether weather is enabled
    pub fn weather_enabled(mut self, enabled: bool) -> Self {
        self.weather_enabled = enabled;
        self
    }
    
    /// Sets whether seasons are enabled
    pub fn seasons_enabled(mut self, enabled: bool) -> Self {
        self.seasons_enabled = enabled;
        self
    }
    
    /// Sets whether day/night cycle is enabled
    pub fn day_night_cycle(mut self, enabled: bool) -> Self {
        self.day_night_cycle = enabled;
        self
    }
    
    /// Sets whether world is persistent
    pub fn persistent_world(mut self, persistent: bool) -> Self {
        self.persistent_world = persistent;
        self
    }
    
    /// Sets the NPC count
    pub fn npc_count(mut self, npc_count: u32) -> Self {
        self.npc_count = npc_count;
        self
    }
    
    /// Sets whether AI is enabled
    pub fn ai_enabled(mut self, enabled: bool) -> Self {
        self.ai_enabled = enabled;
        self
    }
    
    /// Sets whether AI difficulty scales
    pub fn ai_difficulty_scaling(mut self, scaling: bool) -> Self {
        self.ai_difficulty_scaling = scaling;
        self
    }
    
    /// Sets whether the game has campaign
    pub fn has_campaign(mut self, has_campaign: bool) -> Self {
        self.has_campaign = has_campaign;
        self
    }
    
    /// Sets whether the game has side quests
    pub fn has_side_quests(mut self, has_quests: bool) -> Self {
        self.has_side_quests = has_quests;
        self
    }
    
    /// Sets whether dynamic quests are enabled
    pub fn dynamic_quests(mut self, dynamic: bool) -> Self {
        self.dynamic_quests = dynamic;
        self
    }
    
    /// Adds a tag
    pub fn tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tags.push(tag.into());
        self
    }
    
    /// Adds custom property
    pub fn custom_property<S: Into<String>>(mut self, key: S, value: S) -> Self {
        self.custom_properties.insert(key.into(), value.into());
        self
    }
    
    /// Builds the GameDNA instance
    pub fn build(self) -> Result<GameDNA, crate::errors::SchemaError> {
        let id = self.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        
        let dna = GameDNA {
            id,
            name: self.name,
            version: self.version,
            created_at: now,
            last_modified: now,
            genre: self.genre,
            camera: self.camera,
            tone: self.tone,
            world_scale: self.world_scale,
            target_platforms: self.target_platforms,
            physics_profile: self.physics_profile,
            max_players: self.max_players,
            is_competitive: self.is_competitive,
            supports_coop: self.supports_coop,
            difficulty: self.difficulty,
            monetization: self.monetization,
            target_audience: self.target_audience,
            esrb_rating: self.esrb_rating,
            target_fps: self.target_fps,
            max_draw_distance: self.max_draw_distance,
            max_entities: self.max_entities,
            max_npc_count: self.max_npc_count,
            time_scale: self.time_scale,
            weather_enabled: self.weather_enabled,
            seasons_enabled: self.seasons_enabled,
            day_night_cycle: self.day_night_cycle,
            persistent_world: self.persistent_world,
            npc_count: self.npc_count,
            ai_enabled: self.ai_enabled,
            ai_difficulty_scaling: self.ai_difficulty_scaling,
            has_campaign: self.has_campaign,
            has_side_quests: self.has_side_quests,
            dynamic_quests: self.dynamic_quests,
            tags: self.tags,
            custom_properties: self.custom_properties,
        };
        
        dna.validate()?;
        Ok(dna)
    }
}

impl Default for GameDNABuilder {
    fn default() -> Self {
        Self::new()
    }
}