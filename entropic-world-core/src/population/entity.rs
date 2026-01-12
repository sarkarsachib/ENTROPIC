use serde::{Deserialize, Serialize};
use crate::spatial::coordinates::ChunkCoord;
use crate::temporal::time::WorldTime;

pub type EntityId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub entity_type: EntityType,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub chunk: ChunkCoord,
    pub velocity: (f32, f32),
    pub health: f32,
    pub is_alive: bool,
    pub created_at: WorldTime,
    pub last_updated: WorldTime,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityType {
    NPC,
    Animal,
    Structure,
    Item,
    Effect,
}

impl Entity {
    /// Creates a new Entity with the specified id, type, position, and chunk.
    ///
    /// The returned entity has velocity set to (0.0, 0.0), health set to 1.0, `is_alive` set to `true`,
    /// and both `created_at` and `last_updated` initialized to the current `WorldTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// let chunk = ChunkCoord(0, 0);
    /// let e = Entity::new("ent-1".to_string(), EntityType::NPC, 100.0, 200.0, 0.0, chunk);
    /// assert_eq!(e.id, "ent-1");
    /// assert_eq!(e.entity_type, EntityType::NPC);
    /// assert_eq!(e.velocity, (0.0, 0.0));
    /// assert_eq!(e.health, 1.0);
    /// assert!(e.is_alive);
    /// ```
    pub fn new(id: EntityId, entity_type: EntityType, x: f32, y: f32, z: f32, chunk: ChunkCoord) -> Self {
        let now = WorldTime::default();
        Self {
            id,
            entity_type,
            x,
            y,
            z,
            chunk,
            velocity: (0.0, 0.0),
            health: 1.0,
            is_alive: true,
            created_at: now,
            last_updated: now,
        }
    }

    /// Updates the entity's world position and associated chunk.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut e = Entity::new("e1".to_string(), EntityType::NPC, 0.0, 0.0, 0.0, (0, 0));
    /// e.update_position(10.0, 20.0, 5.0, (1, 2));
    /// assert_eq!(e.x, 10.0);
    /// assert_eq!(e.y, 20.0);
    /// assert_eq!(e.z, 5.0);
    /// assert_eq!(e.chunk, (1, 2));
    /// ```
    pub fn update_position(&mut self, x: f32, y: f32, z: f32, chunk: ChunkCoord) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.chunk = chunk;
    }

    /// Reduces the entity's health by the given amount.
    ///
    /// Health is clamped at 0.0. If health reaches 0.0 or below, the entity is marked as not alive.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut e = Entity::new("ent1".into(), EntityType::NPC, 0.0, 0.0, 0.0, (0, 0));
    /// assert_eq!(e.health, 1.0);
    /// e.take_damage(0.5);
    /// assert_eq!(e.health, 0.5);
    /// e.take_damage(1.0);
    /// assert_eq!(e.health, 0.0);
    /// assert!(!e.is_alive);
    /// ```
    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
        if self.health <= 0.0 {
            self.is_alive = false;
        }
    }

    /// Increases the entity's health by a given amount, capped at 1.0.
    ///
    /// The health is raised by `amount` and clamped to a maximum of `1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut e = Entity::new("entity1".into(), EntityType::NPC, 0.0, 0.0, 0.0, ChunkCoord::new(0, 0));
    /// e.take_damage(0.5);
    /// e.heal(0.3);
    /// assert_eq!(e.health, 0.8);
    /// e.heal(0.5);
    /// assert_eq!(e.health, 1.0);
    /// ```
    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new(
            "entity1".to_string(),
            EntityType::NPC,
            100.0,
            200.0,
            0.0,
            ChunkCoord::new(0, 0),
        );
        assert_eq!(entity.entity_type, EntityType::NPC);
        assert!(entity.is_alive);
        assert_eq!(entity.health, 1.0);
    }

    #[test]
    fn test_entity_damage() {
        let mut entity = Entity::new(
            "entity1".to_string(),
            EntityType::NPC,
            0.0,
            0.0,
            0.0,
            ChunkCoord::new(0, 0),
        );
        entity.take_damage(0.5);
        assert_eq!(entity.health, 0.5);

        entity.take_damage(1.0);
        assert_eq!(entity.health, 0.0);
        assert!(!entity.is_alive);
    }

    #[test]
    fn test_entity_heal() {
        let mut entity = Entity::new(
            "entity1".to_string(),
            EntityType::NPC,
            0.0,
            0.0,
            0.0,
            ChunkCoord::new(0, 0),
        );
        entity.take_damage(0.5);
        entity.heal(0.3);
        assert_eq!(entity.health, 0.8);

        entity.heal(0.5);
        assert_eq!(entity.health, 1.0);
    }
}