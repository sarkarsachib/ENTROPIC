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

    pub fn add_member(&mut self, npc_id: NpcId) {
        if !self.members.contains(&npc_id) {
            self.members.push(npc_id);
        }
    }

    pub fn remove_member(&mut self, npc_id: &NpcId) {
        self.members.retain(|id| id != npc_id);
    }

    pub fn add_ally(&mut self, faction_id: FactionId) {
        if !self.allied_factions.contains(&faction_id) {
            self.allied_factions.push(faction_id);
        }
    }

    pub fn add_enemy(&mut self, faction_id: FactionId) {
        if !self.enemy_factions.contains(&faction_id) {
            self.enemy_factions.push(faction_id);
        }
    }

    pub fn is_allied_with(&self, faction_id: &FactionId) -> bool {
        self.allied_factions.contains(faction_id)
    }

    pub fn is_enemy_of(&self, faction_id: &FactionId) -> bool {
        self.enemy_factions.contains(faction_id)
    }

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
