use criterion::{black_box, criterion_group, criterion_main, Criterion};
use entropic_spatial_engine::{CollisionDetector, SpatialQueries};
use entropic_world_core::{Entity, ChunkCoord, World, EntityType};

fn bench_spatial_queries(c: &mut Criterion) {
    let mut world = World::new("Test".to_string(), "game1".to_string(), 10, 10);
    world.initialize_chunks();

    // Add test entities
    for i in 0..1000 {
        let entity = Entity {
            id: format!("entity_{}", i),
            entity_type: EntityType::NPC,
            x: (i as f32 * 10.0) % 1000.0,
            y: (i as f32 * 7.0) % 1000.0,
            z: 0.0,
            chunk: ChunkCoord::new(0, 0),
            ..Default::default()
        };
        world.entities.insert(entity.id.clone(), entity);
    }

    let mut group = c.benchmark_group("spatial_queries");

    group.bench_function("query_radius", |b| {
        b.iter(|| {
            black_box(SpatialQueries::query_radius(&world, 500.0, 500.0, 100.0));
        });
    });

    group.bench_function("query_radius_sorted", |b| {
        b.iter(|| {
            black_box(SpatialQueries::query_radius_sorted(&world, 500.0, 500.0, 100.0));
        });
    });

    group.bench_function("nearest_entity", |b| {
        b.iter(|| {
            black_box(SpatialQueries::nearest_entity(&world, 500.0, 500.0, 100.0));
        });
    });

    group.bench_function("raycast", |b| {
        b.iter(|| {
            black_box(SpatialQueries::raycast(&world, 0.0, 0.0, 1.0, 0.0, 1000.0));
        });
    });

    group.bench_function("count_entities", |b| {
        b.iter(|| {
            black_box(SpatialQueries::count_entities_in_radius(&world, 500.0, 500.0, 100.0));
        });
    });

    group.finish();
}

fn bench_collision_detection(c: &mut Criterion) {
    let mut world = World::new("Test".to_string(), "game1".to_string(), 5, 5);
    world.initialize_chunks();

    // Make terrain walkable
    for chunk in world.chunks.values_mut() {
        chunk.water_level = -100.0;
        for i in 0..256 {
            for j in 0..256 {
                chunk.set_elevation_at(i, j, 100.0);
            }
        }
    }

    let mut group = c.benchmark_group("collision");

    group.bench_function("is_walkable", |b| {
        b.iter(|| {
            black_box(CollisionDetector::is_walkable(&world, 100.0, 100.0));
        });
    });

    group.bench_function("circle_collision", |b| {
        b.iter(|| {
            black_box(CollisionDetector::circle_collision(100.0, 100.0, 5.0, 110.0, 100.0, 5.0));
        });
    });

    group.bench_function("get_terrain_height", |b| {
        b.iter(|| {
            black_box(CollisionDetector::get_terrain_height(&world, 100.0, 100.0));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_spatial_queries, bench_collision_detection);
criterion_main!(benches);
