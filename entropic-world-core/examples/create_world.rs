use entropic_world_core::world::{World, WorldConfig};
use entropic_world_core::population::{Entity, EntityType, NPC, Faction, Alignment};
use entropic_world_core::economy::{Settlement, Market, ResourceType};
use entropic_world_core::ecosystem::{Species, Diet};
use entropic_world_core::spatial::coordinates::ChunkCoord;

fn main() {
    println!("=== Creating ENTROPIC World ===\n");

    let config = WorldConfig::new(32, 32)
        .with_seed(42)
        .with_time_scale(1.0);

    let mut world = World::from_config(
        "Example World".to_string(),
        "example_game_dna".to_string(),
        config,
    );

    println!("World created: {}", world.name);
    println!("World ID: {}", world.id);
    println!("Dimensions: {}x{} chunks\n", world.width_chunks, world.height_chunks);

    println!("Initializing chunks...");
    world.initialize_chunks();
    println!("✓ {} chunks initialized\n", world.total_chunks());

    println!("Adding entities...");
    let entity = Entity::new(
        "player_entity".to_string(),
        EntityType::NPC,
        1000.0,
        1000.0,
        0.0,
        ChunkCoord::new(4, 4),
    );
    world.add_entity(entity);
    println!("✓ {} entities added\n", world.total_entities());

    println!("Creating NPCs...");
    let npc = NPC::new(
        "npc_merchant_1".to_string(),
        "Marcus the Merchant".to_string(),
        "player_entity".to_string(),
    );
    world.add_npc(npc);
    println!("✓ {} NPCs created\n", world.total_npcs());

    println!("Establishing factions...");
    let faction = Faction::new(
        "faction_merchants".to_string(),
        "Merchant's Guild".to_string(),
        "npc_merchant_1".to_string(),
    );
    world.add_faction(faction);
    println!("✓ {} factions established\n", world.factions.len());

    println!("Building settlements...");
    let settlement = Settlement::new(
        "settlement_riverside".to_string(),
        "Riverside Town".to_string(),
        "faction_merchants".to_string(),
        5000.0,
        5000.0,
    );
    world.add_settlement(settlement);
    println!("✓ {} settlements built\n", world.settlements.len());

    println!("Creating markets...");
    let mut market = Market::new(
        "market_riverside".to_string(),
        "settlement_riverside".to_string(),
    );
    market.add_resource(ResourceType::Food, 1000, 500);
    market.add_resource(ResourceType::Wood, 500, 300);
    world.add_market(market);
    println!("✓ {} markets created\n", world.markets.len());

    println!("Populating wildlife...");
    let mut deer = Species::new(
        "species_deer".to_string(),
        "White-tailed Deer".to_string(),
        Diet::Herbivore,
    );
    deer.base_population = 500;
    deer.reproduction_rate = 0.15;
    world.add_species(deer);
    
    let mut wolf = Species::new(
        "species_wolf".to_string(),
        "Gray Wolf".to_string(),
        Diet::Carnivore,
    );
    wolf.base_population = 50;
    wolf.reproduction_rate = 0.08;
    world.add_species(wolf);
    println!("✓ {} species added\n", world.species.len());

    println!("Simulating time...");
    for _ in 0..100 {
        world.advance_tick();
    }
    println!("✓ Simulated {} ticks\n", world.current_tick);

    println!("=== World Statistics ===");
    println!("Total Chunks: {}", world.total_chunks());
    println!("Total Entities: {}", world.total_entities());
    println!("Total NPCs: {}", world.total_npcs());
    println!("Total Factions: {}", world.factions.len());
    println!("Total Settlements: {}", world.settlements.len());
    println!("Total Markets: {}", world.markets.len());
    println!("Total Species: {}", world.species.len());
    println!("Current Tick: {}", world.current_tick);
    println!("World Time: Year {}, Month {}, Day {}, Hour {}:{:02}",
        world.current_time.year,
        world.current_time.month,
        world.current_time.day,
        world.current_time.hour,
        world.current_time.minute,
    );
    
    println!("\n✓ World creation complete!");
}
