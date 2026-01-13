use entropic_spatial_engine::{CollisionDetector, SpatialQueries, Pathfinder};
use entropic_world_core::{Entity, ChunkCoord, World, EntityType};

fn main() {
    println!("=== Pathfinding Demo ===\n");

    // Create test world
    let mut world = World::new("Test World".to_string(), "game1".to_string(), 5, 5);
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

    // Test 1: Short path
    println!("=== Test 1: Short Path ===");
    let start = (0.0, 0.0);
    let goal = (50.0, 50.0);

    println!("Finding path from ({}, {}) to ({}, {})...", start.0, start.1, goal.0, goal.1);

    let path = Pathfinder::find_path(&world, start, goal, 1000);
    if let Some(p) = path {
        println!("✓ Path found with {} waypoints", p.len());
        println!("  Path length: {:.1}m", Pathfinder::path_length(&p));
        println!("  Start: ({:.1}, {:.1})", p[0].0, p[0].1);
        if p.len() > 1 {
            println!("  End: ({:.1}, {:.1})", p.last().unwrap().0, p.last().unwrap().1);
        }
    } else {
        println!("✗ No path found");
    }

    // Test 2: Long path
    println!("\n=== Test 2: Long Path ===");
    let start2 = (0.0, 0.0);
    let goal2 = (200.0, 200.0);

    println!("Finding path from ({}, {}) to ({}, {})...", start2.0, start2.1, goal2.0, goal2.1);

    let path2 = Pathfinder::find_path(&world, start2, goal2, 5000);
    if let Some(p) = path2 {
        println!("✓ Path found with {} waypoints", p.len());
        println!("  Path length: {:.1}m", Pathfinder::path_length(&p));
    } else {
        println!("✗ No path found");
    }

    // Test 3: Path exists check
    println!("\n=== Test 3: Path Exists ===");
    let exists = Pathfinder::path_exists(&world, (0.0, 0.0), (100.0, 100.0));
    println!("Path from (0, 0) to (100, 100) exists: {}", exists);

    // Test 4: Path simplification
    println!("\n=== Test 4: Path Simplification ===");
    let complex_path = vec![
        (0.0, 0.0),
        (10.0, 10.0),
        (20.0, 20.0),
        (30.0, 30.0),
        (40.0, 40.0),
        (50.0, 50.0),
    ];

    println!("Original path: {} waypoints", complex_path.len());
    let simplified = Pathfinder::simplify_path(&complex_path, 1.0);
    println!("Simplified path: {} waypoints", simplified.len());
    println!("  Removed {} waypoints", complex_path.len() - simplified.len());

    // Test 5: Same position
    println!("\n=== Test 5: Same Position ===");
    let same = Pathfinder::find_path(&world, (50.0, 50.0), (50.0, 50.0), 100);
    if let Some(p) = same {
        println!("✓ Same position path: {} waypoint(s)", p.len());
        assert_eq!(p.len(), 1, "Same position should have 1 waypoint");
    }

    // Test 6: Custom heuristic weight
    println!("\n=== Test 6: Custom Heuristic ===");
    let weighted = Pathfinder::find_path_with_weight(&world, (0.0, 0.0), (100.0, 100.0), 1000, 2.0);
    println!("Path with heuristic weight 2.0: {} waypoints", weighted.map_or(0, |p| p.len()));

    // Performance test
    println!("\n=== Performance Test ===");
    let perf_start = std::time::Instant::now();
    let perf_path = Pathfinder::find_path(&world, (0.0, 0.0), (250.0, 250.0), 10000);
    let perf_duration = perf_start.elapsed();

    if let Some(p) = perf_path {
        println!("Path found in {:?}", perf_duration);
        println!("  Waypoints: {}", p.len());
        println!("  Average time per iteration: {:.2}μs",
            perf_duration.as_micros() as f64 / 10000.0);

        // Check if within target
        let target_ms = 5.0;
        let actual_ms = perf_duration.as_millis() as f64;
        if actual_ms <= target_ms {
            println!("  ✓ Within target (< {}ms)", target_ms);
        } else {
            println!("  ✗ Exceeds target ({}ms > {}ms)", actual_ms, target_ms);
        }
    } else {
        println!("No path found in {:?}", perf_duration);
    }

    println!("\n=== Demo Complete ===");
}
