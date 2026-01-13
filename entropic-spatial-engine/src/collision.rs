use crate::constants::*;
use crate::errors::SpatialError;
use crate::{ChunkCoord, World};
use entropic_world_core::constants::HEIGHTMAP_RESOLUTION;

/// Collision detection system
pub struct CollisionDetector;

impl CollisionDetector {
    /// Check if point is walkable
    pub fn is_walkable(world: &World, x: f32, y: f32) -> bool {
        let chunk_x = (x / CHUNK_SIZE).floor() as u32;
        let chunk_y = (y / CHUNK_SIZE).floor() as u32;
        let coord = ChunkCoord {
            x: chunk_x,
            y: chunk_y,
        };

        if let Some(chunk) = world.chunks.get(&coord) {
            // Check height (no walking on water or extreme slopes)
            let local_x = (x % CHUNK_SIZE) as usize;
            let local_y = (y % CHUNK_SIZE) as usize;

            if local_x < HEIGHTMAP_RESOLUTION && local_y < HEIGHTMAP_RESOLUTION {
                let height = chunk.elevation[local_x * HEIGHTMAP_RESOLUTION + local_y];
                return height > chunk.water_level && height < 200.0;
            }
        }

        false
    }

    /// Check collision between two circles
    pub fn circle_collision(
        x1: f32,
        y1: f32,
        r1: f32,
        x2: f32,
        y2: f32,
        r2: f32,
    ) -> bool {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dist = (dx * dx + dy * dy).sqrt();
        dist < r1 + r2
    }

    /// Check if point is inside circle
    pub fn point_in_circle(px: f32, py: f32, cx: f32, cy: f32, radius: f32) -> bool {
        let dx = px - cx;
        let dy = py - cy;
        (dx * dx + dy * dy).sqrt() < radius
    }

    /// Check if two axis-aligned bounding boxes overlap
    pub fn aabb_collision(
        x1_min: f32,
        y1_min: f32,
        x1_max: f32,
        y1_max: f32,
        x2_min: f32,
        y2_min: f32,
        x2_max: f32,
        y2_max: f32,
    ) -> bool {
        x1_min < x2_max && x1_max > x2_min && y1_min < y2_max && y1_max > y2_min
    }

    /// Check if point is inside AABB
    pub fn point_in_aabb(
        px: f32,
        py: f32,
        x_min: f32,
        y_min: f32,
        x_max: f32,
        y_max: f32,
    ) -> bool {
        px >= x_min && px <= x_max && py >= y_min && py <= y_max
    }

    /// Get terrain height at position
    pub fn get_terrain_height(world: &World, x: f32, y: f32) -> Option<f32> {
        let chunk_x = (x / CHUNK_SIZE).floor() as u32;
        let chunk_y = (y / CHUNK_SIZE).floor() as u32;
        let coord = ChunkCoord {
            x: chunk_x,
            y: chunk_y,
        };

        world.chunks.get(&coord).and_then(|chunk| {
            let local_x = (x % CHUNK_SIZE) as usize;
            let local_y = (y % CHUNK_SIZE) as usize;

            if local_x < HEIGHTMAP_RESOLUTION && local_y < HEIGHTMAP_RESOLUTION {
                Some(chunk.elevation[local_x * HEIGHTMAP_RESOLUTION + local_y])
            } else {
                None
            }
        })
    }

    /// Check if point is underwater
    pub fn is_underwater(world: &World, x: f32, y: f32) -> bool {
        let chunk_x = (x / CHUNK_SIZE).floor() as u32;
        let chunk_y = (y / CHUNK_SIZE).floor() as u32;
        let coord = ChunkCoord {
            x: chunk_x,
            y: chunk_y,
        };

        if let Some(chunk) = world.chunks.get(&coord) {
            if let Some(height) = Self::get_terrain_height(world, x, y) {
                return height < chunk.water_level;
            }
        }

        false
    }

    /// Check line of sight between two points (simple check)
    pub fn has_line_of_sight(world: &World, x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let distance = (dx * dx + dy * dy).sqrt();
        let steps = (distance / 10.0).ceil() as i32;

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = x1 + dx * t;
            let y = y1 + dy * t;

            if !Self::is_walkable(world, x, y) {
                return false;
            }
        }

        true
    }

    /// Find closest walkable point within radius
    pub fn find_closest_walkable(
        world: &World,
        x: f32,
        y: f32,
        search_radius: f32,
        step_size: f32,
    ) -> Option<(f32, f32)> {
        // Check center point first
        if Self::is_walkable(world, x, y) {
            return Some((x, y));
        }

        // Search in expanding circles
        let mut radius = step_size;
        while radius <= search_radius {
            let circumference = 2.0 * std::f32::consts::PI * radius;
            let steps = (circumference / step_size).ceil() as i32;

            for i in 0..steps {
                let angle = 2.0 * std::f32::consts::PI * i as f32 / steps as f32;
                let check_x = x + radius * angle.cos();
                let check_y = y + radius * angle.sin();

                if Self::is_walkable(world, check_x, check_y) {
                    return Some((check_x, check_y));
                }
            }

            radius += step_size;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_world() -> World {
        let mut world = World::new("Test".to_string(), "game1".to_string(), 2, 2);
        world.initialize_chunks();

        // Set some terrain
        if let Some(chunk) = world.chunks.get_mut(&ChunkCoord::new(0, 0)) {
            for i in 0..100 {
                chunk.set_elevation_at(i, i, 150.0);
            }
        }

        world
    }

    #[test]
    fn test_circle_collision() {
        // Overlapping circles
        assert!(CollisionDetector::circle_collision(0.0, 0.0, 5.0, 3.0, 0.0, 3.0));

        // Non-overlapping circles
        assert!(!CollisionDetector::circle_collision(0.0, 0.0, 1.0, 10.0, 10.0, 1.0));
    }

    #[test]
    fn test_point_in_circle() {
        // Point inside
        assert!(CollisionDetector::point_in_circle(1.0, 1.0, 0.0, 0.0, 5.0));

        // Point outside
        assert!(!CollisionDetector::point_in_circle(10.0, 10.0, 0.0, 0.0, 5.0));
    }

    #[test]
    fn test_aabb_collision() {
        // Overlapping AABBs
        assert!(CollisionDetector::aabb_collision(0.0, 0.0, 10.0, 10.0, 5.0, 5.0, 15.0, 15.0));

        // Non-overlapping AABBs
        assert!(!CollisionDetector::aabb_collision(
            0.0, 0.0, 10.0, 10.0, 20.0, 20.0, 30.0, 30.0,
        ));
    }

    #[test]
    fn test_point_in_aabb() {
        // Point inside
        assert!(CollisionDetector::point_in_aabb(5.0, 5.0, 0.0, 0.0, 10.0, 10.0));

        // Point outside
        assert!(!CollisionDetector::point_in_aabb(15.0, 15.0, 0.0, 0.0, 10.0, 10.0));
    }

    #[test]
    fn test_get_terrain_height() {
        let world = create_test_world();
        let height = CollisionDetector::get_terrain_height(&world, 5.0, 5.0);

        assert!(height.is_some());
        assert_eq!(height.unwrap(), 150.0);
    }
}
