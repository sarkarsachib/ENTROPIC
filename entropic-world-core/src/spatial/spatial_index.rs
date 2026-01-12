use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

pub type EntityId = String;

/// Serializes a grid map as a vector of (cell_coord, entity_ids) pairs.
///
/// The function collects entries from the provided `HashMap<(i32, i32), Vec<EntityId>>`
/// into a `Vec` of pairs and delegates serialization to the supplied `Serializer`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use serde_json;
/// use entropic_world_core::spatial::EntityId;
/// // Build a small grid
/// let mut grid: HashMap<(i32, i32), Vec<EntityId>> = HashMap::new();
/// grid.insert((0, 0), vec!["e1".to_string(), "e2".to_string()]);
///
/// // Serialize using serde_json's Serializer into a byte buffer
/// let mut buf = Vec::new();
/// let mut ser = serde_json::Serializer::new(&mut buf);
/// // Call the helper directly (it accepts any `Serializer`)
/// super::serialize_grid(&grid, &mut ser).unwrap();
/// let s = String::from_utf8(buf).unwrap();
/// assert!(s.contains("\"(0,0)\"") || s.contains("[(0,0")); // representation may vary by serializer
/// ```
fn serialize_grid<S>(
    grid: &HashMap<(i32, i32), Vec<EntityId>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let vec: Vec<(&(i32, i32), &Vec<EntityId>)> = grid.iter().collect();
    vec.serialize(serializer)
}

/// Deserializes a sequence of `( (i32, i32), Vec<EntityId> )` pairs into a `HashMap` keyed by the `(i32, i32)` cell coordinates.
///
/// The deserializer is expected to produce a sequence (e.g. a JSON array) of two-element tuples where the first element is a coordinate pair and the second element is a vector of `EntityId`s. The function converts that sequence into a `HashMap<(i32, i32), Vec<EntityId>>`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// // `EntityId` is a `String` alias; examples can use `String` directly.
/// let json = r#"[ [[0,1], ["e1"]], [[2,3], ["e2","e3"]] ]"#;
/// let vec: Vec<((i32, i32), Vec<String>)> = serde_json::from_str(json).unwrap();
/// let map: HashMap<(i32, i32), Vec<String>> = vec.into_iter().collect();
/// assert_eq!(map.get(&(0, 1)).unwrap(), &vec!["e1".to_string()]);
/// ```
fn deserialize_grid<'de, D>(
    deserializer: D,
) -> Result<HashMap<(i32, i32), Vec<EntityId>>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Vec<((i32, i32), Vec<EntityId>)> = Vec::deserialize(deserializer)?;
    Ok(vec.into_iter().collect())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpatialIndex {
    #[serde(serialize_with = "serialize_grid", deserialize_with = "deserialize_grid")]
    grid: HashMap<(i32, i32), Vec<EntityId>>,
    grid_size: f32,
}

impl SpatialIndex {
    /// Creates a new SpatialIndex using the default grid cell size.
    ///
    /// # Examples
    ///
    /// ```
    /// let idx = SpatialIndex::new();
    /// assert!(idx.query_rect(0.0, 0.0, 0.0, 0.0).is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_grid_size(crate::constants::DEFAULT_GRID_SIZE)
    }

    /// Creates a SpatialIndex with an empty grid and the specified grid cell size.
    ///
    /// `grid_size` is the linear size of a single grid cell; coordinates are mapped to integer cell
    /// coordinates by dividing by this value and flooring.
    ///
    /// # Examples
    ///
    /// ```
    /// let idx = SpatialIndex::with_grid_size(2.0);
    /// assert_eq!(idx.grid_size, 2.0);
    /// assert!(idx.grid.is_empty());
    /// ```
    pub fn with_grid_size(grid_size: f32) -> Self {
        Self {
            grid: HashMap::new(),
            grid_size,
        }
    }

    /// Converts world coordinates into integer grid cell indices based on the index's grid size.
    ///
    /// # Examples
    ///
    /// ```
    /// let idx = SpatialIndex::with_grid_size(10.0);
    /// assert_eq!(idx.get_cell(0.0, 0.0), (0, 0));
    /// assert_eq!(idx.get_cell(9.9, 9.9), (0, 0));
    /// assert_eq!(idx.get_cell(10.0, 10.0), (1, 1));
    /// ```
    ///
    /// @returns A tuple `(cell_x, cell_y)` containing the integer grid cell coordinates.
    fn get_cell(&self, x: f32, y: f32) -> (i32, i32) {
        (
            (x / self.grid_size).floor() as i32,
            (y / self.grid_size).floor() as i32,
        )
    }

    /// Inserts an entity into the spatial index at the given world coordinates.
    ///
    /// The entity id is added to the grid cell that contains (x, y). If that cell does not yet
    /// exist, it is created.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut idx = SpatialIndex::with_grid_size(1.0);
    /// let id = "ent1".to_string();
    /// idx.insert(id.clone(), 0.5, -0.2);
    /// assert!(idx.query_radius(0.5, -0.2, 0.1).contains(&id));
    /// ```
    pub fn insert(&mut self, entity_id: EntityId, x: f32, y: f32) {
        let cell = self.get_cell(x, y);
        self.grid.entry(cell).or_insert_with(Vec::new).push(entity_id);
    }

    /// Removes the specified entity from the grid cell that contains the world coordinates `(x, y)`.
    ///
    /// If the cell becomes empty after removal, the cell is removed from the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut idx = SpatialIndex::with_grid_size(1.0);
    /// let id: EntityId = "entity1".into();
    /// idx.insert(id.clone(), 0.5, 0.5);
    /// idx.remove(&id, 0.5, 0.5);
    /// assert!(idx.query_radius(0.5, 0.5, 0.1).is_empty());
    /// ```
    pub fn remove(&mut self, entity_id: &EntityId, x: f32, y: f32) {
        let cell = self.get_cell(x, y);
        if let Some(entities) = self.grid.get_mut(&cell) {
            entities.retain(|e| e != entity_id);
            if entities.is_empty() {
                self.grid.remove(&cell);
            }
        }
    }

    /// Moves an entity to the grid cell corresponding to its new world coordinates if the cell changed.
    ///
    /// If the grid cell computed from (old_x, old_y) differs from the cell computed from (new_x, new_y),
    /// the entity is removed from the old cell and placed into the new cell; if the cell is the same, no change occurs.
    ///
    /// # Parameters
    ///
    /// - `entity_id`: The identifier of the entity to move; ownership is taken and transferred into the index when inserting.
    /// - `old_x`, `old_y`: The entity's previous world coordinates.
    /// - `new_x`, `new_y`: The entity's new world coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut idx = SpatialIndex::new();
    /// let id = "entity1".to_string();
    /// idx.insert(id.clone(), 0.0, 0.0);
    /// idx.update(id.clone(), 0.0, 0.0, 10.0, 10.0);
    /// assert!(!idx.query_radius(0.0, 0.0, 1.0).contains(&id));
    /// assert!(idx.query_radius(10.0, 10.0, 1.0).contains(&id));
    /// ```
    pub fn update(&mut self, entity_id: EntityId, old_x: f32, old_y: f32, new_x: f32, new_y: f32) {
        let old_cell = self.get_cell(old_x, old_y);
        let new_cell = self.get_cell(new_x, new_y);

        if old_cell != new_cell {
            self.remove(&entity_id, old_x, old_y);
            self.insert(entity_id, new_x, new_y);
        }
    }

    /// Collects entity IDs from grid cells within a radius (in world units) around a point.
    ///
    /// This searches neighboring grid cells up to ceil(radius / grid_size) from the center cell and aggregates their entity IDs.
    ///
    /// # Returns
    ///
    /// `Vec<EntityId>` containing all entity IDs from the neighboring grid cells within the computed cell radius.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut idx = SpatialIndex::with_grid_size(1.0);
    /// idx.insert("a".to_string(), 0.2, 0.2);
    /// idx.insert("b".to_string(), 5.0, 5.0);
    /// let found = idx.query_radius(0.0, 0.0, 1.0);
    /// assert!(found.contains(&"a".to_string()));
    /// assert!(!found.contains(&"b".to_string()));
    /// ```
    pub fn query_radius(&self, x: f32, y: f32, radius: f32) -> Vec<EntityId> {
        let mut result = Vec::new();
        let grid_radius = (radius / self.grid_size).ceil() as i32;
        let center_cell = self.get_cell(x, y);

        for dx in -grid_radius..=grid_radius {
            for dy in -grid_radius..=grid_radius {
                let cell = (center_cell.0 + dx, center_cell.1 + dy);
                if let Some(entities) = self.grid.get(&cell) {
                    result.extend(entities.iter().cloned());
                }
            }
        }

        result
    }

    /// Collects all entity IDs within the axis-aligned rectangle defined by the given bounds.
    ///
    /// Iterates over all grid cells that overlap the rectangle [min_x, min_y] to [max_x, max_y]
    /// and aggregates the entity IDs stored in those cells.
    ///
    /// # Returns
    ///
    /// A vector of entity IDs found within the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut idx = SpatialIndex::new();
    /// idx.insert("a".to_string(), 0.0, 0.0);
    /// idx.insert("b".to_string(), 5.0, 5.0);
    /// let found = idx.query_rect(-1.0, -1.0, 1.0, 1.0);
    /// assert!(found.contains(&"a".to_string()));
    /// assert!(!found.contains(&"b".to_string()));
    /// ```
    pub fn query_rect(&self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Vec<EntityId> {
        let mut result = Vec::new();
        let min_cell = self.get_cell(min_x, min_y);
        let max_cell = self.get_cell(max_x, max_y);

        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                if let Some(entities) = self.grid.get(&(x, y)) {
                    result.extend(entities.iter().cloned());
                }
            }
        }

        result
    }

    /// Removes all entities and cells from the spatial index.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut idx = SpatialIndex::new();
    /// idx.insert("e1".to_string(), 0.0, 0.0);
    /// idx.clear();
    /// assert!(idx.query_radius(0.0, 0.0, 1.0).is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.grid.clear();
    }
}

impl Default for SpatialIndex {
    /// Creates a new SpatialIndex using the crate's default grid size.
    ///
    /// # Examples
    ///
    /// ```
    /// let _idx = SpatialIndex::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_index_insert_query() {
        let mut index = SpatialIndex::new();
        index.insert("entity1".to_string(), 10.0, 10.0);
        index.insert("entity2".to_string(), 20.0, 20.0);
        index.insert("entity3".to_string(), 100.0, 100.0);

        let results = index.query_radius(10.0, 10.0, 20.0);
        assert!(results.contains(&"entity1".to_string()));
        assert!(!results.contains(&"entity3".to_string()));
    }

    #[test]
    fn test_spatial_index_remove() {
        let mut index = SpatialIndex::new();
        index.insert("entity1".to_string(), 10.0, 10.0);
        index.remove(&"entity1".to_string(), 10.0, 10.0);

        let results = index.query_radius(10.0, 10.0, 20.0);
        assert!(!results.contains(&"entity1".to_string()));
    }

    #[test]
    fn test_spatial_index_update() {
        let mut index = SpatialIndex::new();
        index.insert("entity1".to_string(), 10.0, 10.0);
        index.update("entity1".to_string(), 10.0, 10.0, 100.0, 100.0);

        let results = index.query_radius(10.0, 10.0, 20.0);
        assert!(!results.contains(&"entity1".to_string()));

        let results = index.query_radius(100.0, 100.0, 20.0);
        assert!(results.contains(&"entity1".to_string()));
    }
}