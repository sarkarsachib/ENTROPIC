use criterion::{black_box, criterion_group, criterion_main, Criterion};
use entropic_spatial_engine::ChunkManager;
use entropic_world_core::World;
use std::sync::Arc;

fn bench_chunk_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("chunk_generation");

    group.bench_function("single_chunk", |b| {
        let manager = ChunkManager::new(
            Arc::new(World::new("Test".to_string(), "game1".to_string(), 100, 100)),
            5,
        );
        let coord = entropic_spatial_engine::ChunkCoord::new(5, 5);

        b.iter(|| {
            let _ = manager.generator().generate_chunk(coord).unwrap();
        });
    });

    group.bench_function("multiple_chunks", |b| {
        let manager = ChunkManager::new(
            Arc::new(World::new("Test".to_string(), "game1".to_string(), 100, 100)),
            5,
        );

        b.iter(|| {
            for x in 0..10 {
                for y in 0..10 {
                    let coord = entropic_spatial_engine::ChunkCoord::new(x, y);
                    black_box(manager.generator().generate_chunk(coord).unwrap());
                }
            }
        });
    });

    group.finish();
}

fn bench_chunk_serialization(c: &mut Criterion) {
    use entropic_spatial_engine::ChunkSerializer;

    let mut group = c.benchmark_group("chunk_serialization");

    group.bench_function("serialize", |b| {
        let manager = ChunkManager::new(
            Arc::new(World::new("Test".to_string(), "game1".to_string(), 100, 100)),
            5,
        );
        let coord = entropic_spatial_engine::ChunkCoord::new(5, 5);
        let chunk = manager.generator().generate_chunk(coord).unwrap();

        b.iter(|| {
            black_box(ChunkSerializer::serialize_chunk(&chunk).unwrap());
        });
    });

    group.bench_function("deserialize", |b| {
        let manager = ChunkManager::new(
            Arc::new(World::new("Test".to_string(), "game1".to_string(), 100, 100)),
            5,
        );
        let coord = entropic_spatial_engine::ChunkCoord::new(5, 5);
        let chunk = manager.generator().generate_chunk(coord).unwrap();
        let data = ChunkSerializer::serialize_chunk(&chunk).unwrap();

        b.iter(|| {
            black_box(ChunkSerializer::deserialize_chunk(&data).unwrap());
        });
    });

    group.finish();
}

criterion_group!(benches, bench_chunk_generation, bench_chunk_serialization);
criterion_main!(benches);
