use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: u32,
    pub y: u32,
}

impl ChunkCoord {
    /// Creates a ChunkCoord with the given x and y coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let c = ChunkCoord::new(3, 4);
    /// assert_eq!(c.x, 3);
    /// assert_eq!(c.y, 4);
    /// ```
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Computes the Euclidean distance between two chunk coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = ChunkCoord::new(0, 0);
    /// let b = ChunkCoord::new(3, 4);
    /// assert_eq!(a.distance_to(&b), 5.0);
    /// ```
    ///
    /// Returns the Euclidean distance between `self` and `other` as an `f32`.
    pub fn distance_to(&self, other: &ChunkCoord) -> f32 {
        let dx = (self.x as i64 - other.x as i64) as f32;
        let dy = (self.y as i64 - other.y as i64) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    /// Returns the existing neighboring chunk coordinates surrounding this chunk.
    ///
    /// Produces a vector containing up to eight adjacent `ChunkCoord` values (the 8-connected neighborhood),
    /// excluding the coordinate itself. Coordinates that would be negative are omitted.
    ///
    /// # Examples
    ///
    /// ```
    /// let c = ChunkCoord::new(5, 5);
    /// let n = c.neighbors();
    /// assert_eq!(n.len(), 8);
    /// assert!(n.contains(&ChunkCoord::new(4, 4)));
    /// assert!(n.contains(&ChunkCoord::new(6, 6)));
    /// ```
    pub fn neighbors(&self) -> Vec<ChunkCoord> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = self.x as i32 + dx;
                let ny = self.y as i32 + dy;
                if nx >= 0 && ny >= 0 {
                    neighbors.push(ChunkCoord::new(nx as u32, ny as u32));
                }
            }
        }
        neighbors
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WorldPosition {
    /// Creates a new WorldPosition from the given x, y, and z coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let pos = WorldPosition::new(1.0, 2.0, 3.0);
    /// assert_eq!(pos.x, 1.0);
    /// assert_eq!(pos.y, 2.0);
    /// assert_eq!(pos.z, 3.0);
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Computes the Euclidean distance between two 3D world positions.
    ///
    /// Returns the straight-line distance between `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = crate::spatial::coordinates::WorldPosition::new(0.0, 0.0, 0.0);
    /// let b = crate::spatial::coordinates::WorldPosition::new(3.0, 4.0, 0.0);
    /// assert_eq!(a.distance_to(&b), 5.0);
    /// ```
    pub fn distance_to(&self, other: &WorldPosition) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Computes the Euclidean distance between two positions in the XY plane.
    ///
    /// Returns the distance as an `f32`.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = WorldPosition::new(0.0, 0.0, 0.0);
    /// let b = WorldPosition::new(3.0, 4.0, 0.0);
    /// assert_eq!(a.distance_2d(&b), 5.0);
    /// ```
    pub fn distance_2d(&self, other: &WorldPosition) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_coord_distance() {
        let c1 = ChunkCoord::new(0, 0);
        let c2 = ChunkCoord::new(3, 4);
        assert_eq!(c1.distance_to(&c2), 5.0);
    }

    #[test]
    fn test_chunk_neighbors() {
        let coord = ChunkCoord::new(5, 5);
        let neighbors = coord.neighbors();
        assert_eq!(neighbors.len(), 8);
        assert!(neighbors.contains(&ChunkCoord::new(4, 4)));
        assert!(neighbors.contains(&ChunkCoord::new(6, 6)));
    }

    #[test]
    fn test_world_position_distance() {
        let p1 = WorldPosition::new(0.0, 0.0, 0.0);
        let p2 = WorldPosition::new(3.0, 4.0, 0.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }
}