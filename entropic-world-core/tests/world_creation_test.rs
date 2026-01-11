use entropic_world_core::world::{World, WorldConfig};

#[test]
fn test_basic_world_creation() {
    let world = World::new(
        "Test World".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
    );

    assert_eq!(world.name, "Test World");
    assert_eq!(world.game_dna_id, "game_dna_1");
    assert_eq!(world.width_chunks, 10);
    assert_eq!(world.height_chunks, 10);
    assert_eq!(world.current_tick, 0);
    assert!(world.weather_enabled);
    assert!(world.persistent);
}

#[test]
fn test_world_from_config() {
    let config = WorldConfig::new(50, 50)
        .with_seed(42)
        .with_time_scale(2.0)
        .disable_weather()
        .disable_economy();

    let world = World::from_config(
        "Config World".to_string(),
        "game_dna_2".to_string(),
        config,
    );

    assert_eq!(world.width_chunks, 50);
    assert_eq!(world.height_chunks, 50);
    assert_eq!(world.time_scale, 2.0);
    assert!(!world.weather_enabled);
    assert!(!world.economy_enabled);
}

#[test]
fn test_chunk_initialization() {
    let mut world = World::new(
        "Chunk Test".to_string(),
        "game_dna_3".to_string(),
        5,
        5,
    );

    assert_eq!(world.total_chunks(), 0);
    
    world.initialize_chunks();
    
    assert_eq!(world.total_chunks(), 25);
}

#[test]
fn test_world_id_uniqueness() {
    let world1 = World::new(
        "World 1".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
    );

    let world2 = World::new(
        "World 2".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
    );

    assert_ne!(world1.id, world2.id);
}

#[test]
fn test_world_tick_advancement() {
    let mut world = World::new(
        "Tick Test".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
    );

    assert_eq!(world.current_tick, 0);

    for i in 1..=100 {
        world.advance_tick();
        assert_eq!(world.current_tick, i);
    }
}

#[test]
fn test_world_metadata() {
    let world = World::new(
        "Metadata Test".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
    );

    let metadata = world.get_metadata();
    
    assert_eq!(metadata.id, world.id);
    assert_eq!(metadata.name, world.name);
    assert_eq!(metadata.game_dna_id, world.game_dna_id);
}
