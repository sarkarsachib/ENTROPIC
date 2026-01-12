use serde::{Deserialize, Serialize};
use crate::population::relationship::NpcId;

pub type FactionId = String;
pub type SettlementId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Faction {
    pub id: FactionId,
    pub name: String,
    pub leader: NpcId,
    pub members: Vec<NpcId>,
    pub alignment: Alignment,
    pub ideology: String,
    pub wealth: u64,
    pub power: f32,
    pub settlements: Vec<SettlementId>,
    pub allied_factions: Vec<FactionId>,
    pub enemy_factions: Vec<FactionId>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Alignment {
    LawfulGood,
    Neutral,
    Chaotic,
}

impl Faction {
    /// Constructs a new Faction with the given id, name, and leader, using sensible defaults for other fields.
    ///
    /// The created faction has no members, settlements, allies, or enemies; its alignment is `Neutral`,
    /// ideology is an empty string, wealth is `0`, and power is `0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = Faction::new("faction_1".into(), "The Guild".into(), "npc_leader".into());
    /// assert_eq!(f.name, "The Guild");
    /// assert_eq!(f.alignment, Alignment::Neutral);
    /// assert!(f.members.is_empty());
    /// ```
    pub fn new(id: FactionId, name: String, leader: NpcId) -> Self {
        Self {
            id,
            name,
            leader,
            members: Vec::new(),
            alignment: Alignment::Neutral,
            ideology: String::new(),
            wealth: 0,
            power: 0.0,
            settlements: Vec::new(),
            allied_factions: Vec::new(),
            enemy_factions: Vec::new(),
        }
    }

    /// Adds an NPC to the faction's members if the NPC is not already present.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut f = Faction::new("f1".into(), "Guild".into(), "leader".into());
    /// f.add_member("npc_a".into());
    /// f.add_member("npc_a".into()); // duplicate is ignored
    /// assert_eq!(f.member_count(), 1);
    /// ```
    pub fn add_member(&mut self, npc_id: NpcId) {
        if !self.members.contains(&npc_id) {
            self.members.push(npc_id);
        }
    }

    /// Removes all occurrences of the given NPC from the faction's member list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut faction = Faction::new("f1".to_string(), "Guild".to_string(), "npc_leader".to_string());
    /// faction.members.push("npc_a".to_string());
    /// faction.members.push("npc_b".to_string());
    /// faction.members.push("npc_a".to_string());
    /// faction.remove_member(&"npc_a".to_string());
    /// assert_eq!(faction.members, vec!["npc_b".to_string()]);
    /// ```
    pub fn remove_member(&mut self, npc_id: &NpcId) {
        self.members.retain(|id| id != npc_id);
    }

    /// Adds a faction to this faction's list of allies.
    ///
    /// If the given `faction_id` is already present, the method has no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut f = Faction::new("f1".into(), "The Guild".into(), "npc_leader".into());
    /// f.add_ally("f2".into());
    /// assert!(f.is_allied_with(&"f2".into()));
    /// // Adding the same ally again does not duplicate it
    /// f.add_ally("f2".into());
    /// assert_eq!(f.member_count(), 0); // unrelated, just ensuring members unaffected
    /// assert!(f.is_allied_with(&"f2".into()));
    /// ```
    pub fn add_ally(&mut self, faction_id: FactionId) {
        if !self.allied_factions.contains(&faction_id) {
            self.allied_factions.push(faction_id);
        }
    }

    /// Adds the given faction to this faction's list of enemies if it is not already present.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut f = Faction::new("f1".into(), "Red Guards".into(), "npc_leader".into());
    /// f.add_enemy("f2".into());
    /// assert!(f.is_enemy_of(&"f2".into()));
    /// ```
    pub fn add_enemy(&mut self, faction_id: FactionId) {
        if !self.enemy_factions.contains(&faction_id) {
            self.enemy_factions.push(faction_id);
        }
    }

    /// Checks whether this faction is allied with the given faction.
    ///
    /// `true` if the faction's allied list contains `faction_id`, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut f = Faction::new("f1".to_string(), "Faction".to_string(), "npc1".to_string());
    /// let ally = "f2".to_string();
    /// f.add_ally(ally.clone());
    /// assert!(f.is_allied_with(&ally));
    /// ```
    pub fn is_allied_with(&self, faction_id: &FactionId) -> bool {
        self.allied_factions.contains(faction_id)
    }

    /// Checks whether this faction lists another faction as an enemy.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut f = Faction::new("f1".into(), "Faction One".into(), "leader".into());
    /// let other = "f2".to_string();
    /// f.add_enemy(other.clone());
    /// assert!(f.is_enemy_of(&other));
    /// ```
    ///
    /// # Returns
    ///
    /// `true` if `faction_id` is in this faction's enemy list, `false` otherwise.
    pub fn is_enemy_of(&self, faction_id: &FactionId) -> bool {
        self.enemy_factions.contains(faction_id)
    }

    /// Get the number of members in the faction.
    ///
    /// # Returns
    ///
    /// The number of members in the faction as `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut f = Faction::new("f1".into(), "Test".into(), "npc_leader".into());
    /// f.add_member("npc_a".into());
    /// f.add_member("npc_b".into());
    /// assert_eq!(f.member_count(), 2);
    /// ```
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faction_creation() {
        let faction = Faction::new(
            "faction_1".to_string(),
            "The Guild".to_string(),
            "npc_1".to_string(),
        );
        assert_eq!(faction.name, "The Guild");
        assert_eq!(faction.alignment, Alignment::Neutral);
    }

    #[test]
    fn test_faction_members() {
        let mut faction = Faction::new(
            "faction_1".to_string(),
            "The Guild".to_string(),
            "npc_1".to_string(),
        );
        
        faction.add_member("npc_2".to_string());
        faction.add_member("npc_3".to_string());
        assert_eq!(faction.member_count(), 2);

        faction.remove_member(&"npc_2".to_string());
        assert_eq!(faction.member_count(), 1);
    }

    #[test]
    fn test_faction_relations() {
        let mut faction = Faction::new(
            "faction_1".to_string(),
            "The Guild".to_string(),
            "npc_1".to_string(),
        );

        faction.add_ally("faction_2".to_string());
        faction.add_enemy("faction_3".to_string());

        assert!(faction.is_allied_with(&"faction_2".to_string()));
        assert!(faction.is_enemy_of(&"faction_3".to_string()));
    }
}