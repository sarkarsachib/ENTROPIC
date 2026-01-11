use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

pub type EntityId = String;

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
    pub fn new() -> Self {
        Self::with_grid_size(crate::constants::DEFAULT_GRID_SIZE)
    }

    pub fn with_grid_size(grid_size: f32) -> Self {
        Self {
            grid: HashMap::new(),
            grid_size,
        }
    }

    fn get_cell(&self, x: f32, y: f32) -> (i32, i32) {
        (
            (x / self.grid_size).floor() as i32,
            (y / self.grid_size).floor() as i32,
        )
    }

    pub fn insert(&mut self, entity_id: EntityId, x: f32, y: f32) {
        let cell = self.get_cell(x, y);
        self.grid.entry(cell).or_insert_with(Vec::new).push(entity_id);
    }

    pub fn remove(&mut self, entity_id: &EntityId, x: f32, y: f32) {
        let cell = self.get_cell(x, y);
        if let Some(entities) = self.grid.get_mut(&cell) {
            entities.retain(|e| e != entity_id);
            if entities.is_empty() {
                self.grid.remove(&cell);
            }
        }
    }

    pub fn update(&mut self, entity_id: EntityId, old_x: f32, old_y: f32, new_x: f32, new_y: f32) {
        let old_cell = self.get_cell(old_x, old_y);
        let new_cell = self.get_cell(new_x, new_y);

        if old_cell != new_cell {
            self.remove(&entity_id, old_x, old_y);
            self.insert(entity_id, new_x, new_y);
        }
    }

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

    pub fn clear(&mut self) {
        self.grid.clear();
    }
}

impl Default for SpatialIndex {
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
