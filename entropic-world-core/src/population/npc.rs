use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::population::entity::EntityId;
use crate::population::relationship::{NpcId, Relationship};
use crate::population::schedule::Schedule;
use crate::population::memory::Memory;
use crate::economy::item::Item;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NPC {
    pub id: NpcId,
    pub name: String,
    pub entity_id: EntityId,
    pub faction: Option<String>,
    pub personality: Personality,
    pub skills: HashMap<String, f32>,
    pub inventory: Vec<Item>,
    pub schedule: Schedule,
    pub memory: Memory,
    pub relationships: HashMap<NpcId, Relationship>,
    pub status: NpcStatus,
    pub age: f32,
    pub gender: Gender,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Personality {
    pub aggression: f32,
    pub courage: f32,
    pub honesty: f32,
    pub intelligence: f32,
    pub greed: f32,
    pub compassion: f32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NpcStatus {
    Active,
    Idle,
    Working,
    Sleeping,
    Traveling,
    InCombat,
    Dead,
}

impl NPC {
    pub fn new(id: NpcId, name: String, entity_id: EntityId) -> Self {
        Self {
            id,
            name,
            entity_id,
            faction: None,
            personality: Personality::default(),
            skills: HashMap::new(),
            inventory: Vec::new(),
            schedule: Schedule::default(),
            memory: Memory::new(),
            relationships: HashMap::new(),
            status: NpcStatus::Idle,
            age: 20.0,
            gender: Gender::Other,
        }
    }

    pub fn add_skill(&mut self, skill: String, proficiency: f32) {
        self.skills.insert(skill, proficiency.clamp(0.0, 1.0));
    }

    pub fn get_skill(&self, skill: &str) -> f32 {
        *self.skills.get(skill).unwrap_or(&0.0)
    }

    pub fn improve_skill(&mut self, skill: &str, amount: f32) {
        let current = self.get_skill(skill);
        self.add_skill(skill.to_string(), current + amount);
    }

    pub fn add_item(&mut self, item: Item) {
        self.inventory.push(item);
    }

    pub fn get_relationship(&self, npc_id: &NpcId) -> Option<&Relationship> {
        self.relationships.get(npc_id)
    }

    pub fn get_relationship_mut(&mut self, npc_id: &NpcId) -> Option<&mut Relationship> {
        self.relationships.get_mut(npc_id)
    }

    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.insert(relationship.target.clone(), relationship);
    }

    pub fn set_status(&mut self, status: NpcStatus) {
        self.status = status;
    }

    pub fn is_alive(&self) -> bool {
        self.status != NpcStatus::Dead
    }
}

impl Personality {
    pub fn new(
        aggression: f32,
        courage: f32,
        honesty: f32,
        intelligence: f32,
        greed: f32,
        compassion: f32,
    ) -> Self {
        Self {
            aggression: aggression.clamp(0.0, 1.0),
            courage: courage.clamp(0.0, 1.0),
            honesty: honesty.clamp(0.0, 1.0),
            intelligence: intelligence.clamp(0.0, 1.0),
            greed: greed.clamp(0.0, 1.0),
            compassion: compassion.clamp(0.0, 1.0),
        }
    }
}

impl Default for Personality {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.5, 0.5, 0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npc_creation() {
        let npc = NPC::new("npc_1".to_string(), "Alice".to_string(), "entity_1".to_string());
        assert_eq!(npc.name, "Alice");
        assert_eq!(npc.status, NpcStatus::Idle);
        assert!(npc.is_alive());
    }

    #[test]
    fn test_npc_skills() {
        let mut npc = NPC::new("npc_1".to_string(), "Bob".to_string(), "entity_1".to_string());
        npc.add_skill("archery".to_string(), 0.7);
        assert_eq!(npc.get_skill("archery"), 0.7);

        npc.improve_skill("archery", 0.2);
        assert_eq!(npc.get_skill("archery"), 0.9);
    }

    #[test]
    fn test_personality() {
        let personality = Personality::new(0.8, 0.3, 0.9, 0.6, 0.2, 0.7);
        assert_eq!(personality.aggression, 0.8);
        assert_eq!(personality.compassion, 0.7);
    }
}
