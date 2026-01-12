# Entropic World Core Data Model

Complete reference documentation for all data structures in the Entropic World Core library.

## Table of Contents

1. [World](#world)
2. [Spatial System](#spatial-system)
3. [Temporal System](#temporal-system)
4. [Population System](#population-system)
5. [Economy System](#economy-system)
6. [Ecosystem System](#ecosystem-system)
7. [Event System](#event-system)

---

## World

### World

The root data structure representing the entire game world.

```rust
pub struct World {
    // Metadata
    pub id: String,                              // UUID
    pub name: String,
    pub game_dna_id: String,
    pub version: SemanticVersion,
    
    // Temporal state
    pub current_tick: u64,
    pub current_time: WorldTime,
    pub created_at: DateTime<Utc>,
    pub last_simulated: DateTime<Utc>,
    
    // Spatial state
    pub width_chunks: u32,
    pub height_chunks: u32,
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub spatial_index: SpatialIndex,
    
    // Population
    pub entities: HashMap<EntityId, Entity>,
    pub npcs: HashMap<NpcId, NPC>,
    pub factions: HashMap<FactionId, Faction>,
    
    // Economy
    pub markets: HashMap<String, Market>,
    pub settlements: HashMap<SettlementId, Settlement>,
    pub trade_routes: Vec<TradeRoute>,
    
    // Ecosystem
    pub species: HashMap<SpeciesId, Species>,
    pub animal_populations: HashMap<SpeciesId, u32>,
    
    // Events
    pub event_queue: EventQueue,
    pub event_history: Vec<WorldEvent>,
    
    // Configuration
    pub time_scale: f32,
    pub weather_enabled: bool,
    pub seasons_enabled: bool,
    pub day_night_cycle_enabled: bool,
    pub economy_enabled: bool,
    pub ai_enabled: bool,
    pub persistent: bool,
}
```

### SemanticVersion

```rust
pub struct SemanticVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
```

---

## Spatial System

### Chunk

256x256 meter tile representing a portion of the world.

```rust
pub struct Chunk {
    pub coord: ChunkCoord,
    pub biome: Biome,
    pub elevation: Vec<f32>,              // 256x256 heightmap
    pub vegetation: Vec<u8>,              // 256x256 vegetation density
    pub water_level: f32,
    pub entities: Vec<EntityId>,
    pub structures: Vec<Structure>,
    pub weather: Weather,
    pub loaded: bool,
}
```

### ChunkCoord

```rust
pub struct ChunkCoord {
    pub x: u32,
    pub y: u32,
}
```

### WorldPosition

```rust
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
```

### Biome

```rust
pub enum Biome {
    Forest,
    Desert,
    Mountains,
    Plains,
    Swamp,
    Tundra,
    Ocean,
    Grassland,
    Custom(u32),
}
```

### Structure

```rust
pub struct Structure {
    pub id: StructureId,
    pub structure_type: StructureType,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub owner: Option<String>,
    pub faction: Option<String>,
    pub built_at: WorldTime,
    pub condition: f32,                   // 0.0-1.0
}

pub enum StructureType {
    House,
    Castle,
    Tower,
    Farm,
    Market,
    Temple,
    Barracks,
    Inn,
    Workshop,
    Bridge,
    Custom(u32),
}
```

### SpatialIndex

```rust
pub struct SpatialIndex {
    grid: HashMap<(i32, i32), Vec<EntityId>>,
    grid_size: f32,                       // Default: 16.0 meters
}
```

---

## Temporal System

### WorldTime

```rust
pub struct WorldTime {
    pub year: u32,
    pub month: u8,                        // 1-12
    pub day: u8,                          // 1-31
    pub hour: u8,                         // 0-23
    pub minute: u8,                       // 0-59
    pub second: u8,                       // 0-59
    pub tick: u64,                        // Sub-second ticks
}
```

### Weather

```rust
pub struct Weather {
    pub condition: WeatherCondition,
    pub temperature: f32,                 // Celsius
    pub humidity: f32,                    // 0.0-1.0
    pub wind_speed: f32,                  // m/s
    pub wind_direction: f32,              // 0-360 degrees
    pub precipitation: f32,               // mm/hour
}

pub enum WeatherCondition {
    Clear,
    Cloudy,
    Rainy,
    Stormy,
    Snowy,
    Fog,
}
```

### Calendar

```rust
pub struct Calendar {
    pub year_length_days: u16,
    pub month_names: Vec<String>,
    pub day_names: Vec<String>,
    pub season_names: Vec<String>,
    pub month_lengths: Vec<u8>,
}
```

### Season

```rust
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}
```

---

## Population System

### Entity

Base entity structure for all world objects.

```rust
pub struct Entity {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub chunk: ChunkCoord,
    pub velocity: (f32, f32),
    pub health: f32,                      // 0.0-1.0
    pub is_alive: bool,
    pub created_at: WorldTime,
    pub last_updated: WorldTime,
}

pub enum EntityType {
    NPC,
    Animal,
    Structure,
    Item,
    Effect,
}
```

### NPC

Non-player character with full AI data.

```rust
pub struct NPC {
    pub id: NpcId,
    pub name: String,
    pub entity_id: EntityId,
    pub faction: Option<FactionId>,
    pub personality: Personality,
    pub skills: HashMap<String, f32>,     // skill → proficiency (0-1)
    pub inventory: Vec<Item>,
    pub schedule: Schedule,
    pub memory: Memory,
    pub relationships: HashMap<NpcId, Relationship>,
    pub status: NpcStatus,
    pub age: f32,
    pub gender: Gender,
}
```

### Personality

```rust
pub struct Personality {
    pub aggression: f32,                  // 0.0-1.0
    pub courage: f32,
    pub honesty: f32,
    pub intelligence: f32,
    pub greed: f32,
    pub compassion: f32,
}
```

### NpcStatus

```rust
pub enum NpcStatus {
    Active,
    Idle,
    Working,
    Sleeping,
    Traveling,
    InCombat,
    Dead,
}
```

### Gender

```rust
pub enum Gender {
    Male,
    Female,
    Other,
}
```

### Relationship

```rust
pub struct Relationship {
    pub target: NpcId,
    pub opinion: f32,                     // -1.0 to 1.0
    pub trust: f32,                       // 0.0 to 1.0
    pub fear: f32,
    pub attraction: f32,
    pub last_interaction: Option<WorldTime>,
}
```

### Schedule

```rust
pub struct Schedule {
    pub routines: HashMap<DayType, Vec<ScheduleEntry>>,
}

pub struct ScheduleEntry {
    pub start_hour: u8,
    pub end_hour: u8,
    pub activity: Activity,
    pub location: Option<(f32, f32)>,
}

pub enum Activity {
    Work,
    Sleep,
    Eat,
    Socialize,
    Travel,
    Combat,
    Custom,
}

pub enum DayType {
    Weekday,
    Weekend,
    Festival,
}
```

### Memory

```rust
pub struct Memory {
    pub recent_events: Vec<MemoryEvent>,
    pub long_term_knowledge: HashMap<String, String>,
}

pub struct MemoryEvent {
    pub description: String,
    pub timestamp: WorldTime,
    pub importance: f32,                  // 0.0-1.0
}
```

### Faction

```rust
pub struct Faction {
    pub id: FactionId,
    pub name: String,
    pub leader: NpcId,
    pub members: Vec<NpcId>,
    pub alignment: Alignment,
    pub ideology: String,
    pub wealth: u64,
    pub power: f32,                       // 0.0-1.0
    pub settlements: Vec<SettlementId>,
    pub allied_factions: Vec<FactionId>,
    pub enemy_factions: Vec<FactionId>,
}

pub enum Alignment {
    LawfulGood,
    Neutral,
    Chaotic,
}
```

---

## Economy System

### Market

```rust
pub struct Market {
    pub id: MarketId,
    pub settlement_id: SettlementId,
    pub prices: HashMap<ResourceType, MarketPrice>,
    pub supply: HashMap<ResourceType, u32>,
    pub demand: HashMap<ResourceType, u32>,
}

pub struct MarketPrice {
    pub base_price: u32,
    pub current_price: u32,
    pub volatility: f32,
    pub last_updated: WorldTime,
}
```

### Settlement

```rust
pub struct Settlement {
    pub id: SettlementId,
    pub name: String,
    pub faction: FactionId,
    pub x: f32,
    pub y: f32,
    pub population: u32,
    pub wealth: u64,
    pub buildings: Vec<StructureId>,
    pub markets: Vec<String>,
    pub allegiances: HashMap<FactionId, f32>,
    pub resources: HashMap<ResourceType, u32>,
    pub happiness: f32,                   // 0.0-1.0
}
```

### ResourceType

```rust
pub enum ResourceType {
    Food,
    Wood,
    Metal,
    Stone,
    Cloth,
    Herbs,
    Gold,
    Custom(u32),
}
```

### TradeRoute

```rust
pub struct TradeRoute {
    pub id: String,
    pub from: SettlementId,
    pub to: SettlementId,
    pub resource: ResourceType,
    pub frequency: u32,                   // Trades per year
    pub caravan_size: u32,
    pub active: bool,
}
```

### Item

```rust
pub struct Item {
    pub id: String,
    pub item_type: ItemType,
    pub quantity: u32,
    pub weight: f32,
    pub value: u32,
}

pub enum ItemType {
    Weapon,
    Armor,
    Food,
    Potion,
    Currency,
    Custom(u32),
}
```

---

## Ecosystem System

### Species

```rust
pub struct Species {
    pub id: SpeciesId,
    pub name: String,
    pub diet: Diet,
    pub base_population: u32,
    pub reproduction_rate: f32,           // Births per year per individual
    pub lifespan_years: u16,
    pub preferred_biomes: Vec<Biome>,
    pub hunting_prey: Vec<SpeciesId>,
    pub hunted_by: Vec<SpeciesId>,
}

pub enum Diet {
    Herbivore,
    Carnivore,
    Omnivore,
}
```

### PopulationControl

```rust
pub struct PopulationControl {
    pub species_id: SpeciesId,
    pub current_population: u32,
    pub birth_rate: f32,
    pub death_rate: f32,
    pub carrying_capacity: u32,
}
```

### FoodChain

```rust
pub struct FoodChain {
    relationships: HashMap<SpeciesId, Vec<SpeciesId>>,
}
```

---

## Event System

### WorldEvent

```rust
pub struct WorldEvent {
    pub id: String,
    pub event_type: EventType,
    pub timestamp: WorldTime,
    pub location: (f32, f32),
    pub involved_entities: Vec<EntityId>,
    pub description: String,
}

pub enum EventType {
    NPCBirth,
    NPCDeath,
    NPCMarriage,
    FactionWar,
    Settlement,
    TradeCompleted,
    Disaster,
    Discovery,
    Custom(String),
}
```

### EventQueue

```rust
pub struct EventQueue {
    events: Vec<(u64, WorldEvent)>,       // (tick, event)
}
```

### EventTrigger

```rust
pub struct EventTrigger {
    pub id: String,
    pub condition: TriggerCondition,
    pub event_template: String,
    pub active: bool,
}

pub enum TriggerCondition {
    TimeElapsed(u64),
    PopulationThreshold { settlement_id: String, threshold: u32 },
    ResourceDepletion { resource: String, threshold: u32 },
    EntityDeath(String),
    FactionRelationship { faction_a: String, faction_b: String, threshold: f32 },
    Custom(String),
}
```

---

## Type Aliases

```rust
pub type EntityId = String;
pub type NpcId = String;
pub type FactionId = String;
pub type SettlementId = String;
pub type SpeciesId = String;
pub type StructureId = String;
pub type MarketId = String;
```

---

## Constants

```rust
pub const DEFAULT_CHUNK_SIZE: f32 = 256.0;              // meters
pub const HEIGHTMAP_RESOLUTION: usize = 256;            // 256x256
pub const DEFAULT_GRID_SIZE: f32 = 16.0;                // meters
pub const DEFAULT_TICKS_PER_SECOND: u64 = 20;
pub const DEFAULT_TIME_SCALE: f32 = 1.0;
pub const MAX_ENTITIES_PER_CHUNK: usize = 1000;
pub const DEFAULT_MEMORY_CAPACITY: usize = 100;
pub const DEFAULT_PRICE_VOLATILITY: f32 = 0.15;
```
