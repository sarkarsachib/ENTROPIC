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
    /// Creates a new NPC with the given id, name, and associated entity id, populated with sensible defaults.
    ///
    /// The created NPC will have:
    /// - `faction` set to `None`
    /// - a default `Personality` (all traits set to 0.5)
    /// - empty `skills`, `inventory`, and `relationships`
    /// - default `Schedule` and `Memory`
    /// - `status` set to `NpcStatus::Idle`
    /// - `age` set to `20.0`
    /// - `gender` set to `Gender::Other`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let npc = NPC::new(1, "Alice".to_string(), 100);
    /// assert_eq!(npc.name, "Alice");
    /// assert_eq!(npc.status, NpcStatus::Idle);
    /// assert!(npc.is_alive());
    /// ```
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

    /// Adds or updates a skill on the NPC.
    ///
    /// The provided `proficiency` is clamped to the range 0.0..=1.0 before being stored under the given skill name in the NPC's skills map.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut npc = NPC::new(1, "Alice".to_string(), 100);
    /// npc.add_skill("archery".to_string(), 1.2);
    /// assert_eq!(npc.get_skill("archery"), 1.0);
    /// npc.add_skill("archery".to_string(), 0.4);
    /// assert_eq!(npc.get_skill("archery"), 0.4);
    /// ```
    pub fn add_skill(&mut self, skill: String, proficiency: f32) {
        self.skills.insert(skill, proficiency.clamp(0.0, 1.0));
    }

    /// Retrieves the proficiency value for a named skill.
    ///
    /// Returns `0.0` when the NPC has no recorded proficiency for the given skill.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut npc = NPC::new(1, "Bob".into(), 100);
    /// assert_eq!(npc.get_skill("swordsmanship"), 0.0);
    /// npc.add_skill("swordsmanship", 0.7);
    /// assert_eq!(npc.get_skill("swordsmanship"), 0.7);
    /// ```
    pub fn get_skill(&self, skill: &str) -> f32 {
        *self.skills.get(skill).unwrap_or(&0.0)
    }

    /// Increases an NPC's skill proficiency by a given amount while keeping the resulting value between 0.0 and 1.0.
    ///
    /// The skill is created with the adjusted proficiency if it does not already exist.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut npc = NPC::new(1, "Test", 100);
    /// npc.improve_skill("sword", 0.3);
    /// assert_eq!(npc.get_skill("sword"), 0.3);
    /// ```
    pub fn improve_skill(&mut self, skill: &str, amount: f32) {
        let current = self.get_skill(skill);
        self.add_skill(skill.to_string(), current + amount);
    }

    /// Adds an item to the NPC's inventory.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut npc = NPC::new(1, "Alice".into(), 42);
    /// let item = Item::new("Rusty Sword");
    /// npc.add_item(item);
    /// assert_eq!(npc.inventory.len(), 1);
    /// ```
    pub fn add_item(&mut self, item: Item) {
        self.inventory.push(item);
    }

    /// Retrieve an immutable reference to the relationship with another NPC by their id.
    ///
    /// # Parameters
    ///
    /// - `npc_id`: The id of the target NPC whose relationship is being queried.
    ///
    /// # Returns
    ///
    /// `Some(&Relationship)` if a relationship exists for the given `npc_id`, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let npc = NPC::new(1, "Alice".into(), 100);
    /// // Assuming no relationships have been added yet:
    /// assert!(npc.get_relationship(&2).is_none());
    /// ```
    pub fn get_relationship(&self, npc_id: &NpcId) -> Option<&Relationship> {
        self.relationships.get(npc_id)
    }

    /// Obtain a mutable reference to the relationship with another NPC.
    ///
    ///
    /// # Parameters
    ///
    /// * `npc_id` - ID of the target NPC whose relationship will be retrieved.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `Relationship` with the given NPC id, or `None` if no relationship exists.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut npc = NPC::new(1, "Alice".into(), 100);
    /// let other_id = 2;
    /// if let Some(rel) = npc.get_relationship_mut(&other_id) {
    ///     rel.trust = (rel.trust + 0.1).min(1.0);
    /// }
    /// ```
    pub fn get_relationship_mut(&mut self, npc_id: &NpcId) -> Option<&mut Relationship> {
        self.relationships.get_mut(npc_id)
    }

    /// Inserts or replaces a relationship in the NPC's relationships map.
    ///
    /// The provided `relationship` is stored using its `target` as the key; if a relationship
    /// with the same target already exists it will be overwritten.
    ///
    /// # Parameters
    ///
    /// - `relationship`: The `Relationship` to add to the NPC's relationships.
    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.insert(relationship.target.clone(), relationship);
    }

    /// Sets the NPC's current status.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut npc = NPC::new(1, "Test".into(), 42);
    /// npc.set_status(NpcStatus::Traveling);
    /// assert_eq!(npc.status, NpcStatus::Traveling);
    /// ```
    pub fn set_status(&mut self, status: NpcStatus) {
        self.status = status;
    }

    /// Indicates whether the NPC is alive.
    ///
    /// # Returns
    ///
    /// `true` if the NPC's status is not `NpcStatus::Dead`, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut npc = NPC::new(1, "Test".into(), 100);
    /// assert!(npc.is_alive());
    /// npc.set_status(NpcStatus::Dead);
    /// assert!(!npc.is_alive());
    /// ```
    pub fn is_alive(&self) -> bool {
        self.status != NpcStatus::Dead
    }
}

impl Personality {
    /// Creates a `Personality` where each trait value is clamped to the range 0.0 to 1.0.
    ///
    /// Values passed outside [0.0, 1.0] are reduced to the nearest bound.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = entropic_world_core::population::npc::Personality::new(1.2, -0.1, 0.5, 0.75, 0.0, 0.9);
    /// assert_eq!(p.aggression, 1.0);
    /// assert_eq!(p.courage, 0.0);
    /// assert_eq!(p.honesty, 0.5);
    /// assert_eq!(p.intelligence, 0.75);
    /// assert_eq!(p.greed, 0.0);
    /// assert_eq!(p.compassion, 0.9);
    /// ```
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
    /// Creates a balanced `Personality` with every trait initialized to 0.5.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Personality::default();
    /// assert_eq!(p.aggression, 0.5);
    /// assert_eq!(p.courage, 0.5);
    /// assert_eq!(p.honesty, 0.5);
    /// assert_eq!(p.intelligence, 0.5);
    /// assert_eq!(p.greed, 0.5);
    /// assert_eq!(p.compassion, 0.5);
    /// ```
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