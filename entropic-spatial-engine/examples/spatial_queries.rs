use entropic_spatial_engine::{CollisionDetector, SpatialQueries, Pathfinder};
use entropic_world_core::{Entity, ChunkCoord, World};

fn main() {
    println!("=== Spatial Queries Demo ===\n");

    // Create test world
    let mut world = World::new("Test World".to_string(), "game1".to_string(), 5, 5);
    world.initialize_chunks();

    // Add test entities
    println!("Adding test entities...");
    add_test_entities(&mut world);

    println!("Total entities in world: {}\n", world.entities.len());

    // Radius query
    println!("=== Radius Query ===");
    let entities = SpatialQueries::query_radius(&world, 100.0, 100.0, 150.0);
    println!("Entities within 150m of (100, 100): {}", entities.len());
    for entity in &entities {
        println!("  - {} ({}, {}) type={}", entity.id, entity.x, entity.y, entity.entity_type);
    }

    // Sorted radius query
    println!("\n=== Sorted Radius Query ===");
    let sorted = SpatialQueries::query_radius_sorted(&world, 100.0, 100.0, 200.0);
    println!("Entities within 200m, sorted by distance:");
    for (i, entity) in sorted.iter().enumerate().take(5) {
        let dist = ((entity.x - 100.0).powi(2) + (entity.y - 100.0).powi(2)).sqrt();
        println!("  {}. {} (type: {:?}) - distance: {:.1}m", i + 1, entity.id, entity.entity_type, dist);
    }

    // Nearest entity
    println!("\n=== Nearest Entity Query ===");
    if let Some(nearest) = SpatialQueries::nearest_entity(&world, 100.0, 100.0, 500.0) {
        let dist = ((nearest.x - 100.0).powi(2) + (nearest.y - 100.0).powi(2)).sqrt();
        println!("Nearest entity: {} at distance: {:.1}m", nearest.id, dist);
    }

    // Chunk query
    println!("\n=== Chunk Query ===");
    let chunk_entities = SpatialQueries::query_chunk(&world, ChunkCoord::new(0, 0));
    println!("Entities in chunk (0, 0): {}", chunk_entities.len());

    // Raycast
    println!("\n=== Raycast Query ===");
    let hit_result = SpatialQueries::raycast(&world, 0.0, 100.0, 1.0, 0.0, 1000.0);
    if let Some(hit) = hit_result {
        println!("Raycast hit entity: {} at ({}, {})", hit.id, hit.x, hit.y);
    } else {
        println!("Raycast: No hit");
    }

    // Collision detection
    println!("\n=== Collision Detection ===");
    let pos = (100.0, 100.0);
    let walkable = CollisionDetector::is_walkable(&world, pos.0, pos.1);
    println!("Position ({}, {}) is walkable: {}", pos.0, pos.1, walkable);

    let terrain_height = CollisionDetector::get_terrain_height(&world, 100.0, 100.0);
    println!("Terrain height at ({}, {}): {:?}", pos.0, pos.1, terrain_height);

    // Circle collision
    println!("\n=== Circle Collision ===");
    let collides = CollisionDetector::circle_collision(100.0, 100.0, 5.0, 110.0, 100.0, 5.0);
    println!("Circle (100, 100, r=5) collides with (110, 100, r=5): {}", collides);

    let no_collision = CollisionDetector::circle_collision(100.0, 100.0, 5.0, 150.0, 150.0, 5.0);
    println!("Circle (100, 100, r=5) collides with (150, 150, r=5): {}", no_collision);

    println!("\n=== Demo Complete ===");
}

fn add_test_entities(world: &mut World) {
    use entropic_world_core::EntityType;

    let entities = vec![
        Entity {
            id: "player1".to_string(),
            entity_type: EntityType::NPC,
            x: 100.0,
            y: 100.0,
            z: 0.0,
            chunk: ChunkCoord::new(0, 0),
            ..Default::default()
        },
        Entity {
            id: "npc1".to_string(),
            entity_type: EntityType::NPC,
            x: 150.0,
            y: 110.0,
            z: 0.0,
            chunk: ChunkCoord::new(0, 0),
            ..Default::default()
        },
        Entity {
            id: "npc2".to_string(),
            entity_type: EntityType::NPC,
            x: 80.0,
            y: 90.0,
            z: 0.0,
            chunk: ChunkCoord::new(0, 0),
            ..Default::default()
        },
        Entity {
            id: "enemy1".to_string(),
            entity_type: EntityType::Animal,
            x: 200.0,
            y: 150.0,
            z: 0.0,
            chunk: ChunkCoord::new(0, 0),
            ..Default::default()
        },
        Entity {
            id: "enemy2".to_string(),
            entity_type: EntityType::Animal,
            x: 250.0,
            y: 100.0,
            z: 0.0,
            chunk: ChunkCoord::new(0, 0),
            ..Default::default()
        },
    ];

    for entity in entities {
        world.entities.insert(entity.id.clone(), entity.clone());

        // Add to chunk
        if let Some(chunk) = world.chunks.get_mut(&entity.chunk) {
            chunk.add_entity(entity.id.clone());
        }
    }
}
