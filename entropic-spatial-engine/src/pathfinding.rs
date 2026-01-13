use crate::constants::*;
use crate::collision::CollisionDetector;
use crate::errors::SpatialError;
use crate::World;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
struct PathfindingState {
    cost: u32,
    position: (i32, i32),
}

impl Ord for PathfindingState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PathfindingState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A* pathfinding on world terrain
pub struct Pathfinder;

impl Pathfinder {
    /// Find path from start to goal
    ///
    /// Returns a vector of (x, y) positions from start to goal, inclusive.
    /// Returns None if no path is found within max_iterations.
    pub fn find_path(
        world: &World,
        start: (f32, f32),
        goal: (f32, f32),
        max_iterations: u32,
    ) -> Option<Vec<(f32, f32)>> {
        Self::find_path_internal(world, start, goal, max_iterations, None)
    }

    /// Find path from start to goal with custom heuristic weight
    ///
    /// `heuristic_weight` controls how much the algorithm prioritizes
    /// the distance to goal vs. actual cost (default 1.0).
    pub fn find_path_with_weight(
        world: &World,
        start: (f32, f32),
        goal: (f32, f32),
        max_iterations: u32,
        heuristic_weight: f32,
    ) -> Option<Vec<(f32, f32)>> {
        Self::find_path_internal(world, start, goal, max_iterations, Some(heuristic_weight))
    }

    fn find_path_internal(
        world: &World,
        start: (f32, f32),
        goal: (f32, f32),
        max_iterations: u32,
        heuristic_weight: Option<f32>,
    ) -> Option<Vec<(f32, f32)>> {
        let start_grid = (
            (start.0 / PATHFINDING_GRID_SIZE).floor() as i32,
            (start.1 / PATHFINDING_GRID_SIZE).floor() as i32,
        );
        let goal_grid = (
            (goal.0 / PATHFINDING_GRID_SIZE).floor() as i32,
            (goal.1 / PATHFINDING_GRID_SIZE).floor() as i32,
        );

        // Check if goal is walkable
        if !CollisionDetector::is_walkable(world, goal.0, goal.1) {
            return None;
        }

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut g_score: HashMap<(i32, i32), u32> = HashMap::new();
        let mut f_score: HashMap<(i32, i32), u32> = HashMap::new();

        open_set.push(PathfindingState {
            cost: 0,
            position: start_grid,
        });

        g_score.insert(start_grid, 0);
        f_score.insert(
            start_grid,
            (Self::heuristic(start_grid, goal_grid) as f32 * heuristic_weight.unwrap_or(1.0)) as u32,
        );

        let mut iterations = 0;
        while !open_set.is_empty() && iterations < max_iterations {
            iterations += 1;

            let PathfindingState {
                position: current,
                ..
            } = open_set.pop().unwrap();

            if current == goal_grid {
                // Reconstruct path
                return Some(Self::reconstruct_path(came_from, current, start_grid));
            }

            // Check neighbors (8-directional movement)
            for (dx, dy) in &[
                (0, 1),
                (1, 0),
                (0, -1),
                (-1, 0),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ] {
                let neighbor = (current.0 + dx, current.1 + dy);

                // Convert back to world coordinates
                let world_x = neighbor.0 as f32 * PATHFINDING_GRID_SIZE;
                let world_y = neighbor.1 as f32 * PATHFINDING_GRID_SIZE;

                // Check if walkable
                if !CollisionDetector::is_walkable(world, world_x, world_y) {
                    continue;
                }

                // Calculate cost (diagonal movement costs sqrt(2))
                let move_cost = if dx.abs() + dy.abs() == 2 {
                    1414 // sqrt(2) * 1000
                } else {
                    1000 // 1.0 * 1000
                };

                let tentative_g = g_score.get(&current).copied().unwrap_or(u32::MAX) + move_cost;

                if tentative_g < g_score.get(&neighbor).copied().unwrap_or(u32::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g);

                    let h = Self::heuristic(neighbor, goal_grid) as f32 * heuristic_weight.unwrap_or(1.0);
                    let f = tentative_g + (h * 1000.0) as u32;
                    f_score.insert(neighbor, f);

                    open_set.push(PathfindingState {
                        cost: f,
                        position: neighbor,
                    });
                }
            }
        }

        None
    }

    /// Manhattan distance heuristic
    fn heuristic(pos: (i32, i32), goal: (i32, i32)) -> f32 {
        let dx = (pos.0 - goal.0).abs() as f32;
        let dy = (pos.1 - goal.1).abs() as f32;
        dx + dy
    }

    /// Reconstruct path from came_from map
    fn reconstruct_path(
        came_from: HashMap<(i32, i32), (i32, i32)>,
        current: (i32, i32),
        start: (i32, i32),
    ) -> Vec<(f32, f32)> {
        let mut path = vec![];

        // Convert grid to world coordinates
        let current_world = (
            current.0 as f32 * PATHFINDING_GRID_SIZE + PATHFINDING_GRID_SIZE / 2.0,
            current.1 as f32 * PATHFINDING_GRID_SIZE + PATHFINDING_GRID_SIZE / 2.0,
        );
        path.push(current_world);

        let mut node = current;
        while let Some(&prev) = came_from.get(&node) {
            let prev_world = (
                prev.0 as f32 * PATHFINDING_GRID_SIZE + PATHFINDING_GRID_SIZE / 2.0,
                prev.1 as f32 * PATHFINDING_GRID_SIZE + PATHFINDING_GRID_SIZE / 2.0,
            );
            path.push(prev_world);
            node = prev;

            if node == start {
                break;
            }
        }

        path.reverse();
        path
    }

    /// Check if a path exists between two points (without computing full path)
    pub fn path_exists(world: &World, start: (f32, f32), goal: (f32, f32)) -> bool {
        Self::find_path(world, start, goal, MAX_PATHFINDING_ITERATIONS).is_some()
    }

    /// Get path length in world units
    pub fn path_length(path: &[(f32, f32)]) -> f32 {
        if path.len() < 2 {
            return 0.0;
        }

        let mut total = 0.0;
        for window in path.windows(2) {
            let dx = window[1].0 - window[0].0;
            let dy = window[1].1 - window[0].1;
            total += (dx * dx + dy * dy).sqrt();
        }

        total
    }

    /// Simplify path by removing unnecessary waypoints
    pub fn simplify_path(path: &[(f32, f32)], tolerance: f32) -> Vec<(f32, f32)> {
        if path.len() <= 2 {
            return path.to_vec();
        }

        let mut simplified = vec![path[0]];

        for i in 1..path.len() - 1 {
            let prev = simplified.last().unwrap();
            let curr = path[i];
            let next = path[i + 1];

            // Check if current point is necessary
            if Self::deviation(prev, &curr, next) > tolerance {
                simplified.push(curr);
            }
        }

        simplified.push(path[path.len() - 1]);
        simplified
    }

    /// Calculate deviation of middle point from line segment
    fn deviation(a: &(f32, f32), b: &(f32, f32), c: &(f32, f32)) -> f32 {
        // Distance from point b to line segment ac
        let ab = (b.0 - a.0, b.1 - a.1);
        let ac = (c.0 - a.0, c.1 - a.1);

        let cross = ab.0 * ac.1 - ab.1 * ac.0;
        let ac_length = (ac.0 * ac.0 + ac.1 * ac.1).sqrt();

        if ac_length == 0.0 {
            0.0
        } else {
            (cross.abs() / ac_length)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_world() -> World {
        let mut world = World::new("Test".to_string(), "game1".to_string(), 5, 5);
        world.initialize_chunks();

        // Make terrain walkable
        for chunk in world.chunks.values_mut() {
            chunk.water_level = -100.0; // No water
            for i in 0..HEIGHTMAP_RESOLUTION {
                for j in 0..HEIGHTMAP_RESOLUTION {
                    chunk.set_elevation_at(i, j, 100.0);
                }
            }
        }

        world
    }

    #[test]
    fn test_find_path() {
        let world = create_test_world();
        let path = Pathfinder::find_path(&world, (0.0, 0.0), (100.0, 100.0), 1000);

        assert!(path.is_some());
        assert!(path.unwrap().len() > 1);
    }

    #[test]
    fn test_find_path_same_position() {
        let world = create_test_world();
        let path = Pathfinder::find_path(&world, (50.0, 50.0), (50.0, 50.0), 1000);

        assert!(path.is_some());
        assert_eq!(path.unwrap().len(), 1);
    }

    #[test]
    fn test_path_length() {
        let path = vec![(0.0, 0.0), (3.0, 0.0), (3.0, 4.0)];
        let length = Pathfinder::path_length(&path);

        assert!((length - 7.0).abs() < 0.01); // 3 + 4 = 7
    }

    #[test]
    fn test_simplify_path() {
        let path = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0)];
        let simplified = Pathfinder::simplify_path(&path, 1.0);

        // Should reduce to start and end
        assert_eq!(simplified.len(), 2);
        assert_eq!(simplified[0], (0.0, 0.0));
        assert_eq!(simplified[1], (3.0, 3.0));
    }

    #[test]
    fn test_path_exists() {
        let world = create_test_world();
        assert!(Pathfinder::path_exists(&world, (0.0, 0.0), (100.0, 100.0)));
    }
}
