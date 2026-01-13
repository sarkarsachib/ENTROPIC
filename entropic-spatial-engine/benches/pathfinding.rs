use criterion::{black_box, criterion_group, criterion_main, Criterion};
use entropic_spatial_engine::{CollisionDetector, Pathfinder};
use entropic_world_core::World;

fn bench_pathfinding(c: &mut Criterion) {
    let mut world = World::new("Test".to_string(), "game1".to_string(), 10, 10);
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

    let mut group = c.benchmark_group("pathfinding");

    group.bench_function("short_path", |b| {
        b.iter(|| {
            black_box(Pathfinder::find_path(&world, (0.0, 0.0), (50.0, 50.0), 1000));
        });
    });

    group.bench_function("long_path", |b| {
        b.iter(|| {
            black_box(Pathfinder::find_path(&world, (0.0, 0.0), (200.0, 200.0), 5000));
        });
    });

    group.bench_function("same_position", |b| {
        b.iter(|| {
            black_box(Pathfinder::find_path(&world, (50.0, 50.0), (50.0, 50.0), 100));
        });
    });

    group.bench_function("path_exists", |b| {
        b.iter(|| {
            black_box(Pathfinder::path_exists(&world, (0.0, 0.0), (100.0, 100.0)));
        });
    });

    group.bench_function("simplify_path", |b| {
        let path = vec![
            (0.0, 0.0),
            (10.0, 10.0),
            (20.0, 20.0),
            (30.0, 30.0),
            (40.0, 40.0),
            (50.0, 50.0),
        ];

        b.iter(|| {
            black_box(Pathfinder::simplify_path(&path, 1.0));
        });
    });

    group.finish();
}

fn bench_pathfinding_performance(c: &mut Criterion) {
    let mut world = World::new("Test".to_string(), "game1".to_string(), 10, 10);
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

    let mut group = c.benchmark_group("pathfinding_performance");

    // Test different iteration counts
    group.bench_function("1000_iterations", |b| {
        b.iter(|| {
            black_box(Pathfinder::find_path(&world, (0.0, 0.0), (250.0, 250.0), 1000));
        });
    });

    group.bench_function("5000_iterations", |b| {
        b.iter(|| {
            black_box(Pathfinder::find_path(&world, (0.0, 0.0), (250.0, 250.0), 5000));
        });
    });

    group.bench_function("10000_iterations", |b| {
        b.iter(|| {
            black_box(Pathfinder::find_path(&world, (0.0, 0.0), (250.0, 250.0), 10000));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_pathfinding, bench_pathfinding_performance);
criterion_main!(benches);
