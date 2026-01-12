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

    pub fn initialize_chunks(&mut self) {
        for x in 0..self.width_chunks {
            for y in 0..self.height_chunks {
                let coord = ChunkCoord::new(x, y);
                let chunk = Chunk::new(coord);
                self.chunks.insert(coord, chunk);
            }
        }
    }

    pub fn get_chunk(&self, coord: &ChunkCoord) -> Option<&Chunk> {
        self.chunks.get(coord)
    }

    pub fn get_chunk_mut(&mut self, coord: &ChunkCoord) -> Option<&mut Chunk> {
        self.chunks.get_mut(coord)
    }

    pub fn add_entity(&mut self, entity: Entity) {
        let entity_id = entity.id.clone();
        self.spatial_index.insert(entity_id.clone(), entity.x, entity.y);
        
        if let Some(chunk) = self.chunks.get_mut(&entity.chunk) {
            chunk.add_entity(entity_id.clone());
        }
        
        self.entities.insert(entity_id, entity);
    }

    pub fn remove_entity(&mut self, entity_id: &EntityId) {
        if let Some(entity) = self.entities.remove(entity_id) {
            self.spatial_index.remove(entity_id, entity.x, entity.y);
            
            if let Some(chunk) = self.chunks.get_mut(&entity.chunk) {
                chunk.remove_entity(entity_id);
            }
        }
    }

    pub fn add_npc(&mut self, npc: NPC) {
        self.npcs.insert(npc.id.clone(), npc);
    }

    pub fn add_faction(&mut self, faction: Faction) {
        self.factions.insert(faction.id.clone(), faction);
    }

    pub fn add_settlement(&mut self, settlement: Settlement) {
        self.settlements.insert(settlement.id.clone(), settlement);
    }

    pub fn add_market(&mut self, market: Market) {
        self.markets.insert(market.id.clone(), market);
    }

    pub fn add_species(&mut self, species: Species) {
        let species_id = species.id.clone();
        let base_population = species.base_population;
        self.species.insert(species_id.clone(), species);
        self.animal_populations.insert(species_id, base_population);
    }

    pub fn advance_tick(&mut self) {
        self.current_tick += 1;
        self.current_time.advance_tick(crate::constants::DEFAULT_TICKS_PER_SECOND);
        self.last_simulated = Utc::now();
        
        let events = self.event_queue.get_events_at_tick(self.current_tick);
        for event in events {
            self.event_history.push(event);
        }
    }

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

    pub fn total_chunks(&self) -> usize {
        self.chunks.len()
    }

    pub fn total_entities(&self) -> usize {
        self.entities.len()
    }

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
