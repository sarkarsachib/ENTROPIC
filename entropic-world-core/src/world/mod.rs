pub mod world_config;
pub mod world_metadata;
pub mod world_state;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

use crate::spatial::{Chunk, ChunkCoord, SpatialIndex};
use crate::temporal::time::WorldTime;
use crate::population::{Entity, EntityId, NPC, NpcId, Faction, FactionId};
use crate::economy::{Market, Settlement, SettlementId, TradeRoute};
use crate::ecosystem::{Species, SpeciesId};
use crate::events::{WorldEvent, EventQueue};

pub use world_config::WorldConfig;
pub use world_metadata::{SemanticVersion, WorldMetadata};
pub use world_state::WorldState;

/// Serializes a map of chunk coordinates to chunks as a sequence of `(ChunkCoord, Chunk)` pairs.
///
/// This helper converts the provided `HashMap<ChunkCoord, Chunk>` into a `Vec<(&ChunkCoord, &Chunk)>`
/// and then delegates serialization to Serde so the map is stored as a sequence of pairs.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use serde_json;
///
/// let mut map: HashMap<i32, i32> = HashMap::new();
/// map.insert(1, 10);
/// map.insert(2, 20);
///
/// // Serialize the map as a sequence of (key, value) pairs by collecting into a Vec
/// let vec: Vec<(&i32, &i32)> = map.iter().collect();
/// let json = serde_json::to_string(&vec).unwrap();
/// assert!(json.starts_with('['));
/// ```
fn serialize_chunk_map<S>(
    chunks: &HashMap<ChunkCoord, Chunk>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let vec: Vec<(&ChunkCoord, &Chunk)> = chunks.iter().collect();
    vec.serialize(serializer)
}

/// Deserializes a sequence of `(ChunkCoord, Chunk)` pairs into a `HashMap<ChunkCoord, Chunk>`.
///
/// The deserializer expects a sequence/array of two-element tuples where the first element is a `ChunkCoord` and the second is a `Chunk`; each tuple becomes an entry in the returned `HashMap`.
///
/// # Examples
///
/// ```
/// // Deserialize a JSON array of [ [coord, chunk], ... ] into the chunk map.
/// // let json = r#"[ [ [0,0], { /* chunk fields */ } ] ]"#;
/// // let chunks: std::collections::HashMap<ChunkCoord, Chunk> = serde_json::from_str(json).unwrap();
/// // assert!(chunks.get(&ChunkCoord::new(0, 0)).is_some());
/// ```
fn deserialize_chunk_map<'de, D>(
    deserializer: D,
) -> Result<HashMap<ChunkCoord, Chunk>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Vec<(ChunkCoord, Chunk)> = Vec::deserialize(deserializer)?;
    Ok(vec.into_iter().collect())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct World {
    pub id: String,
    pub name: String,
    pub game_dna_id: String,
    pub version: SemanticVersion,
    
    pub current_tick: u64,
    pub current_time: WorldTime,
    pub created_at: chrono::DateTime<Utc>,
    pub last_simulated: chrono::DateTime<Utc>,
    
    pub width_chunks: u32,
    pub height_chunks: u32,
    #[serde(serialize_with = "serialize_chunk_map", deserialize_with = "deserialize_chunk_map")]
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub spatial_index: SpatialIndex,
    
    pub entities: HashMap<EntityId, Entity>,
    pub npcs: HashMap<NpcId, NPC>,
    pub factions: HashMap<FactionId, Faction>,
    
    pub markets: HashMap<String, Market>,
    pub settlements: HashMap<SettlementId, Settlement>,
    pub trade_routes: Vec<TradeRoute>,
    
    pub species: HashMap<SpeciesId, Species>,
    pub animal_populations: HashMap<SpeciesId, u32>,
    
    pub event_queue: EventQueue,
    pub event_history: Vec<WorldEvent>,
    
    pub time_scale: f32,
    pub weather_enabled: bool,
    pub seasons_enabled: bool,
    pub day_night_cycle_enabled: bool,
    pub economy_enabled: bool,
    pub ai_enabled: bool,
    pub persistent: bool,
}

impl World {
    /// Creates a new World with the supplied name, game DNA identifier, and chunk grid size, initialized to default runtime state and feature flags enabled.
    ///
    /// The returned World has a generated UUID `id`, version set to `SemanticVersion::default()`, `current_tick` set to 0, timestamps (`created_at` and `last_simulated`) set to the current UTC time, empty collections for chunks, entities, NPCs, factions, markets, settlements, species and events, a fresh `SpatialIndex`, `time_scale` set to `DEFAULT_TIME_SCALE`, and feature toggles (weather, seasons, day/night cycle, economy, AI, persistence) enabled by default.
    ///
    /// # Examples
    ///
    /// ```
    /// let world = crate::world::World::new(
    ///     "Test World".to_string(),
    ///     "game-dna-xyz".to_string(),
    ///     10,
    ///     10,
    /// );
    /// assert_eq!(world.name, "Test World");
    /// assert_eq!(world.width_chunks, 10);
    /// assert_eq!(world.height_chunks, 10);
    /// assert_eq!(world.current_tick, 0);
    /// assert!(world.id.len() > 0);
    /// ```
    pub fn new(
        name: String,
        game_dna_id: String,
        width_chunks: u32,
        height_chunks: u32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            game_dna_id,
            version: SemanticVersion::default(),
            current_tick: 0,
            current_time: WorldTime::default(),
            created_at: now,
            last_simulated: now,
            width_chunks,
            height_chunks,
            chunks: HashMap::new(),
            spatial_index: SpatialIndex::new(),
            entities: HashMap::new(),
            npcs: HashMap::new(),
            factions: HashMap::new(),
            markets: HashMap::new(),
            settlements: HashMap::new(),
            trade_routes: Vec::new(),
            species: HashMap::new(),
            animal_populations: HashMap::new(),
            event_queue: EventQueue::new(),
            event_history: Vec::new(),
            time_scale: crate::constants::DEFAULT_TIME_SCALE,
            weather_enabled: true,
            seasons_enabled: true,
            day_night_cycle_enabled: true,
            economy_enabled: true,
            ai_enabled: true,
            persistent: true,
        }
    }

    /// Creates a new `World` configured from the provided `WorldConfig`.
    ///
    /// The resulting `World` uses `name` and `game_dna_id` and applies configuration overrides
    /// from `config` (dimensions, time scale, feature flags, and persistence).
    ///
    /// # Returns
    ///
    /// The newly constructed `World` with fields set according to `config`.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = WorldConfig {
    ///     width_chunks: 5,
    ///     height_chunks: 5,
    ///     time_scale: 1.5,
    ///     weather_enabled: false,
    ///     seasons_enabled: true,
    ///     day_night_cycle_enabled: true,
    ///     economy_enabled: true,
    ///     ai_enabled: true,
    ///     persistent: false,
    /// };
    /// let world = World::from_config("My World".to_string(), "game-dna-001".to_string(), config);
    /// assert_eq!(world.width_chunks, 5);
    /// assert_eq!(world.time_scale, 1.5);
    /// assert!(!world.weather_enabled);
    /// ```
    pub fn from_config(name: String, game_dna_id: String, config: WorldConfig) -> Self {
        let mut world = Self::new(name, game_dna_id, config.width_chunks, config.height_chunks);
        world.time_scale = config.time_scale;
        world.weather_enabled = config.weather_enabled;
        world.seasons_enabled = config.seasons_enabled;
        world.day_night_cycle_enabled = config.day_night_cycle_enabled;
        world.economy_enabled = config.economy_enabled;
        world.ai_enabled = config.ai_enabled;
        world.persistent = config.persistent;
        world
    }

    /// Populates the world's chunk map with newly created chunks covering its configured dimensions.
    ///
    /// Iterates over the range [0, width_chunks) × [0, height_chunks) and inserts a new `Chunk` at
    /// each `ChunkCoord`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = World::new("W".into(), "dna".into(), 3, 2);
    /// world.initialize_chunks();
    /// assert_eq!(world.total_chunks(), 3 * 2);
    /// ```
    pub fn initialize_chunks(&mut self) {
        for x in 0..self.width_chunks {
            for y in 0..self.height_chunks {
                let coord = ChunkCoord::new(x, y);
                let chunk = Chunk::new(coord);
                self.chunks.insert(coord, chunk);
            }
        }
    }

    /// Fetches an immutable reference to the chunk at the provided chunk coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = World::new("test".to_string(), "game_dna".to_string(), 1, 1);
    /// world.initialize_chunks();
    /// let coord = ChunkCoord::new(0, 0);
    /// assert!(world.get_chunk(&coord).is_some());
    /// ```
    ///
    /// # Returns
    ///
    /// `Some(&Chunk)` containing the chunk at `coord` if present, `None` otherwise.
    pub fn get_chunk(&self, coord: &ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(coord)
    }

    /// Retrieve a mutable reference to the chunk at the provided coordinate.
    ///
    /// # Returns
    ///
    /// `Some(&mut Chunk)` if a chunk exists at `coord`, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// // assuming `world` is a mutable `World` and `coord` is a `ChunkCoord`
    /// if let Some(chunk) = world.get_chunk_mut(&coord) {
    ///     // modify the chunk here
    /// }
    /// ```
    pub fn get_chunk_mut(&mut self, coord: &ChunkCoord) -> Option<&mut Chunk> {
        self.chunks.get_mut(coord)
    }

    /// Adds an entity to the world, updating the spatial index, the containing chunk (if present), and the world's entity map.
    ///
    /// This inserts the entity into `self.entities`, records its position in `self.spatial_index`, and appends its id to the corresponding chunk's entity list when that chunk exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use entropic_world_core::world::World;
    /// # use entropic_world_core::population::Entity;
    /// # use entropic_world_core::spatial::ChunkCoord;
    /// let mut world = World::new("test".into(), "dna".into(), 5, 5);
    /// world.initialize_chunks();
    ///
    /// // Construct an Entity with an id, position and chunk. Adjust the constructor to your actual Entity API.
    /// let entity = Entity::new("entity-1".into(), 0, 0, ChunkCoord::new(0, 0));
    /// world.add_entity(entity);
    ///
    /// assert_eq!(world.total_entities(), 1);
    /// ```
    pub fn add_entity(&mut self, entity: Entity) {
        let entity_id = entity.id.clone();
        self.spatial_index.insert(entity_id.clone(), entity.x, entity.y);
        
        if let Some(chunk) = self.chunks.get_mut(&entity.chunk) {
            chunk.add_entity(entity_id.clone());
        }
        
        self.entities.insert(entity_id, entity);
    }

    /// Removes the entity identified by `entity_id` from the world.
    ///
    /// This removes the entity from the world's entity map, the spatial index (using the
    /// entity's last-known coordinates), and the chunk that contained the entity if that
    /// chunk is present.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = World::new("demo".into(), "dna".into(), 10, 10);
    /// // assume `entity_id` is an EntityId previously inserted into `world`
    /// // world.remove_entity(&entity_id);
    /// ```
    pub fn remove_entity(&mut self, entity_id: &EntityId);
    pub fn remove_entity(&mut self, entity_id: &EntityId) {
        if let Some(entity) = self.entities.remove(entity_id) {
            self.spatial_index.remove(entity_id, entity.x, entity.y);
            
            if let Some(chunk) = self.chunks.get_mut(&entity.chunk) {
                chunk.remove_entity(entity_id);
            }
        }
    }

    /// Adds an NPC to the world's NPC registry.
    ///
    /// The NPC is stored in the world's `npcs` map keyed by the NPC's `id`.
    /// If an NPC with the same `id` already exists, it will be replaced.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = World::new("Test".into(), "game".into(), 1, 1);
    /// let npc = NPC { id: "npc1".into(), ..Default::default() };
    /// world.add_npc(npc);
    /// assert!(world.npcs.contains_key("npc1"));
    /// ```
    pub fn add_npc(&mut self, npc: NPC) {
        self.npcs.insert(npc.id.clone(), npc);
    }

    /// Adds a faction to the world's faction registry.
    ///
    /// The provided `Faction` is consumed and stored in the world's internal `factions` map keyed by the faction's `id`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = World::new("My World".into(), "game_dna".into(), 10, 10);
    /// let faction = /* construct a Faction with an `id` field */ ;
    /// world.add_faction(faction);
    /// ```
    pub fn add_faction(&mut self, faction: Faction) {
        self.factions.insert(faction.id.clone(), faction);
    }

    /// Adds a settlement to the world, storing it in the world's settlement map keyed by the settlement's id.
    ///
    /// The provided `settlement` is inserted into `self.settlements` using `settlement.id.clone()` as the key.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut world = World::new("MyWorld".into(), "game-dna".into(), 10, 10);
    /// let settlement = Settlement { id: "settlement-1".into(), /* ... */ };
    /// world.add_settlement(settlement);
    /// ```
    pub fn add_settlement(&mut self, settlement: Settlement) {
        self.settlements.insert(settlement.id.clone(), settlement);
    }

    /// Inserts the given market into the world's market registry, keyed by the market's `id`.
    ///
    /// The market is stored in `self.markets` using `market.id` as the key.
    pub fn add_market(&mut self, market: Market) {
        self.markets.insert(market.id.clone(), market);
    }

    /// Registers a species in the world and sets its initial animal population.
    ///
    /// The species is stored in the world's `species` map keyed by its `id`, and
    /// the species' `base_population` is inserted into `animal_populations` under
    /// the same `id`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = World::new("Test".to_string(), "dna".to_string(), 10, 10);
    /// let species = Species { id: "rabbit".to_string(), base_population: 50, ..Default::default() };
    /// world.add_species(species);
    /// assert_eq!(world.animal_populations.get("rabbit"), Some(&50));
    /// ```
    pub fn add_species(&mut self, species: Species) {
        let species_id = species.id.clone();
        let base_population = species.base_population;
        self.species.insert(species_id.clone(), species);
        self.animal_populations.insert(species_id, base_population);
    }

    /// Advances the world state by one simulation tick.
    ///
    /// This increments the world's tick counter, advances its time by the configured
    /// tick rate, updates the `last_simulated` timestamp to now, and moves any events
    /// scheduled for the new tick from the event queue into the world's event history.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = World::new("test".into(), "dna".into(), 5, 5);
    /// assert_eq!(world.current_tick, 0);
    /// world.advance_tick();
    /// assert_eq!(world.current_tick, 1);
    /// ```
    pub fn advance_tick(&mut self) {
        self.current_tick += 1;
        self.current_time.advance_tick(crate::constants::DEFAULT_TICKS_PER_SECOND);
        self.last_simulated = Utc::now();
        
        let events = self.event_queue.get_events_at_tick(self.current_tick);
        for event in events {
            self.event_history.push(event);
        }
    }

    /// Creates a WorldMetadata snapshot containing the world's identifying fields.
    ///
    /// The returned metadata includes the world's `id`, `name`, `game_dna_id`, `version`,
    /// `created_at`, and `last_simulated`; `description` and `author` are set to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let world = World::new("TestWorld".to_string(), "game-dna".to_string(), 1, 1);
    /// let meta = world.get_metadata();
    /// assert_eq!(meta.name, "TestWorld");
    /// assert_eq!(meta.game_dna_id, "game-dna");
    /// ```
    pub fn get_metadata(&self) -> WorldMetadata {
        WorldMetadata {
            id: self.id.clone(),
            name: self.name.clone(),
            game_dna_id: self.game_dna_id.clone(),
            version: self.version,
            created_at: self.created_at,
            last_simulated: self.last_simulated,
            description: None,
            author: None,
        }
    }

    /// Report the total number of chunks currently stored in the world.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = World::new("test".into(), "dna".into(), 2, 2);
    /// world.initialize_chunks();
    /// assert_eq!(world.total_chunks(), 4);
    /// ```
    ///
    /// # Returns
    ///
    /// The number of chunks contained in this world.
    pub fn total_chunks(&self) -> usize {
        self.chunks.len()
    }

    /// Reports the number of entities currently stored in the world.
    ///
    /// # Returns
    ///
    /// The number of entities in the world as `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// let world = World::new("test".into(), "dna".into(), 1, 1);
    /// assert_eq!(world.total_entities(), 0);
    /// ```
    pub fn total_entities(&self) -> usize {
        self.entities.len()
    }

    /// Counts the NPCs currently stored in the world.
    ///
    /// # Returns
    ///
    /// The number of NPCs present in the world's `npcs` collection.
    ///
    /// # Examples
    ///
    /// ```
    /// let world = World::new("test".to_string(), "dna".to_string(), 1, 1);
    /// assert_eq!(world.total_npcs(), 0);
    /// ```
    pub fn total_npcs(&self) -> usize {
        self.npcs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_creation() {
        let world = World::new(
            "Test World".to_string(),
            "game_dna_1".to_string(),
            10,
            10,
        );
        assert_eq!(world.name, "Test World");
        assert_eq!(world.width_chunks, 10);
        assert_eq!(world.height_chunks, 10);
    }

    #[test]
    fn test_initialize_chunks() {
        let mut world = World::new(
            "Test World".to_string(),
            "game_dna_1".to_string(),
            5,
            5,
        );
        world.initialize_chunks();
        assert_eq!(world.total_chunks(), 25);
    }

    #[test]
    fn test_add_entity() {
        let mut world = World::new(
            "Test World".to_string(),
            "game_dna_1".to_string(),
            10,
            10,
        );
        world.initialize_chunks();

        let entity = Entity::new(
            "entity_1".to_string(),
            crate::population::EntityType::NPC,
            100.0,
            200.0,
            0.0,
            ChunkCoord::new(0, 0),
        );

        world.add_entity(entity);
        assert_eq!(world.total_entities(), 1);
    }

    #[test]
    fn test_advance_tick() {
        let mut world = World::new(
            "Test World".to_string(),
            "game_dna_1".to_string(),
            10,
            10,
        );

        world.advance_tick();
        assert_eq!(world.current_tick, 1);
    }

    #[test]
    fn test_from_config() {
        let config = WorldConfig::new(100, 100)
            .with_time_scale(2.0)
            .disable_weather();

        let world = World::from_config(
            "Test World".to_string(),
            "game_dna_1".to_string(),
            config,
        );

        assert_eq!(world.width_chunks, 100);
        assert_eq!(world.time_scale, 2.0);
        assert!(!world.weather_enabled);
    }
}