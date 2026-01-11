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

    pub fn update_position(&mut self, x: f32, y: f32, z: f32, chunk: ChunkCoord) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.chunk = chunk;
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
        if self.health <= 0.0 {
            self.is_alive = false;
        }
    }

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
