use entropic_world_core::world::{World, WorldConfig};
use entropic_world_core::population::{Entity, EntityType, NPC, Faction};
use entropic_world_core::economy::{Settlement, Market, ResourceType};
use entropic_world_core::ecosystem::{Species, Diet};
use entropic_world_core::events::{WorldEvent, EventType};
use entropic_world_core::spatial::coordinates::ChunkCoord;
use entropic_world_core::temporal::time::WorldTime;
use entropic_world_core::serialization::{serialize_to_json, deserialize_from_json};

#[test]
fn test_full_world_setup() {
    let config = WorldConfig::new(32, 32)
        .with_seed(42)
        .with_time_scale(1.0);

    let mut world = World::from_config(
        "Integration Test World".to_string(),
        "game_dna_test".to_string(),
        config,
    );

    world.initialize_chunks();
    assert_eq!(world.total_chunks(), 1024);

    let entity = Entity::new(
        "entity_1".to_string(),
        EntityType::NPC,
        1000.0,
        1000.0,
        0.0,
        ChunkCoord::new(4, 4),
    );
    world.add_entity(entity);

    let npc = NPC::new(
        "npc_1".to_string(),
        "Merchant".to_string(),
        "entity_1".to_string(),
    );
    world.add_npc(npc);

    let faction = Faction::new(
        "faction_1".to_string(),
        "Guild".to_string(),
        "npc_1".to_string(),
    );
    world.add_faction(faction);

    let settlement = Settlement::new(
        "settlement_1".to_string(),
        "Town".to_string(),
        "faction_1".to_string(),
        5000.0,
        5000.0,
    );
    world.add_settlement(settlement);

    let mut market = Market::new("market_1".to_string(), "settlement_1".to_string());
    market.add_resource(ResourceType::Food, 1000, 500);
    world.add_market(market);

    let species = Species::new(
        "species_1".to_string(),
        "Deer".to_string(),
        Diet::Herbivore,
    );
    world.add_species(species);

    assert_eq!(world.total_entities(), 1);
    assert_eq!(world.total_npcs(), 1);
    assert_eq!(world.factions.len(), 1);
    assert_eq!(world.settlements.len(), 1);
    assert_eq!(world.markets.len(), 1);
    assert_eq!(world.species.len(), 1);
}

#[test]
fn test_world_simulation() {
    let mut world = World::new(
        "Simulation Test".to_string(),
        "game_dna_sim".to_string(),
        10,
        10,
    );
    world.initialize_chunks();

    let event = WorldEvent::new(
        "event_1".to_string(),
        EventType::Discovery,
        WorldTime::default(),
        (100.0, 200.0),
        "Test event".to_string(),
    );

    world.event_queue.schedule(100, event);

    for _ in 0..100 {
        world.advance_tick();
    }

    assert_eq!(world.current_tick, 100);
    assert_eq!(world.event_history.len(), 1);
}

#[test]
fn test_world_serialization_round_trip() {
    let config = WorldConfig::new(16, 16).with_seed(123);
    let mut world = World::from_config(
        "Serialization Test".to_string(),
        "game_dna_serial".to_string(),
        config,
    );

    world.initialize_chunks();

    let entity = Entity::new(
        "entity_1".to_string(),
        EntityType::NPC,
        100.0,
        200.0,
        0.0,
        ChunkCoord::new(0, 0),
    );
    world.add_entity(entity);

    for _ in 0..50 {
        world.advance_tick();
    }

    let json = serialize_to_json(&world).unwrap();
    let deserialized = deserialize_from_json(&json).unwrap();

    assert_eq!(world.id, deserialized.id);
    assert_eq!(world.name, deserialized.name);
    assert_eq!(world.width_chunks, deserialized.width_chunks);
    assert_eq!(world.height_chunks, deserialized.height_chunks);
    assert_eq!(world.current_tick, deserialized.current_tick);
    assert_eq!(world.total_chunks(), deserialized.total_chunks());
    assert_eq!(world.total_entities(), deserialized.total_entities());
}

#[test]
fn test_economy_system() {
    let mut world = World::new(
        "Economy Test".to_string(),
        "game_dna_econ".to_string(),
        10,
        10,
    );

    let mut settlement = Settlement::new(
        "settlement_1".to_string(),
        "Town".to_string(),
        "faction_1".to_string(),
        0.0,
        0.0,
    );

    settlement.add_population(100);
    settlement.add_wealth(1000);
    settlement.add_resource(ResourceType::Food, 500);

    assert_eq!(settlement.population, 100);
    assert_eq!(settlement.wealth, 1000);
    assert_eq!(settlement.get_resource(&ResourceType::Food), 500);

    assert!(settlement.spend_wealth(500));
    assert_eq!(settlement.wealth, 500);

    assert!(settlement.consume_resource(ResourceType::Food, 100));
    assert_eq!(settlement.get_resource(&ResourceType::Food), 400);

    world.add_settlement(settlement);
    assert_eq!(world.settlements.len(), 1);
}

#[test]
fn test_ecosystem_system() {
    let mut world = World::new(
        "Ecosystem Test".to_string(),
        "game_dna_eco".to_string(),
        10,
        10,
    );

    let mut deer = Species::new(
        "deer".to_string(),
        "White-tailed Deer".to_string(),
        Diet::Herbivore,
    );
    deer.base_population = 500;

    let mut wolf = Species::new(
        "wolf".to_string(),
        "Gray Wolf".to_string(),
        Diet::Carnivore,
    );
    wolf.base_population = 50;
    wolf.add_prey("deer".to_string());

    world.add_species(deer);
    world.add_species(wolf);

    assert_eq!(world.species.len(), 2);
    assert_eq!(world.animal_populations.get("deer"), Some(&500));
    assert_eq!(world.animal_populations.get("wolf"), Some(&50));
}

#[test]
fn test_spatial_queries_integration() {
    let mut world = World::new(
        "Spatial Integration Test".to_string(),
        "game_dna_spatial".to_string(),
        64,
        64,
    );
    world.initialize_chunks();

    for i in 0..100 {
        let x = (i % 10) as f32 * 100.0;
        let y = (i / 10) as f32 * 100.0;
        let chunk_x = (x / 256.0) as u32 % world.width_chunks;
        let chunk_y = (y / 256.0) as u32 % world.height_chunks;

        let entity = Entity::new(
            format!("entity_{}", i),
            EntityType::NPC,
            x,
            y,
            0.0,
            ChunkCoord::new(chunk_x, chunk_y),
        );
        world.add_entity(entity);
    }

    assert_eq!(world.total_entities(), 100);

    let results = world.spatial_index.query_radius(500.0, 500.0, 200.0);
    assert!(!results.is_empty());

    let results = world.spatial_index.query_rect(0.0, 0.0, 500.0, 500.0);
    assert!(!results.is_empty());
}
