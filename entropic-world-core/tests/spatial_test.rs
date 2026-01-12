use entropic_world_core::world::World;
use entropic_world_core::population::{Entity, EntityType};
use entropic_world_core::spatial::coordinates::ChunkCoord;

#[test]
fn test_entity_addition() {
    let mut world = World::new(
        "Entity Test".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
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
    assert_eq!(world.total_entities(), 1);
}

#[test]
fn test_entity_removal() {
    let mut world = World::new(
        "Entity Test".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
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
    assert_eq!(world.total_entities(), 1);

    world.remove_entity(&"entity_1".to_string());
    assert_eq!(world.total_entities(), 0);
}

#[test]
fn test_spatial_index_query() {
    let mut world = World::new(
        "Spatial Test".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
    );
    world.initialize_chunks();

    for i in 0..10 {
        let entity = Entity::new(
            format!("entity_{}", i),
            EntityType::NPC,
            i as f32 * 10.0,
            i as f32 * 10.0,
            0.0,
            ChunkCoord::new(0, 0),
        );
        world.add_entity(entity);
    }

    let results = world.spatial_index.query_radius(50.0, 50.0, 100.0);
    assert!(!results.is_empty());
}

#[test]
fn test_chunk_entity_tracking() {
    let mut world = World::new(
        "Chunk Test".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
    );
    world.initialize_chunks();

    let coord = ChunkCoord::new(5, 5);
    let entity = Entity::new(
        "entity_1".to_string(),
        EntityType::NPC,
        100.0,
        200.0,
        0.0,
        coord,
    );

    world.add_entity(entity);

    let chunk = world.get_chunk(&coord).unwrap();
    assert!(chunk.entities.contains(&"entity_1".to_string()));
}

#[test]
fn test_multiple_entities_same_chunk() {
    let mut world = World::new(
        "Multi-Entity Test".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
    );
    world.initialize_chunks();

    let coord = ChunkCoord::new(0, 0);

    for i in 0..5 {
        let entity = Entity::new(
            format!("entity_{}", i),
            EntityType::NPC,
            i as f32 * 5.0,
            i as f32 * 5.0,
            0.0,
            coord,
        );
        world.add_entity(entity);
    }

    let chunk = world.get_chunk(&coord).unwrap();
    assert_eq!(chunk.entities.len(), 5);
}
