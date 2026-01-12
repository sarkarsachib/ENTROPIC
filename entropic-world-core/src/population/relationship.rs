use serde::{Deserialize, Serialize};
use crate::temporal::time::WorldTime;

pub type NpcId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub target: NpcId,
    pub opinion: f32,
    pub trust: f32,
    pub fear: f32,
    pub attraction: f32,
    pub last_interaction: Option<WorldTime>,
}

impl Relationship {
    pub fn new(target: NpcId) -> Self {
        Self {
            target,
            opinion: 0.0,
            trust: 0.0,
            fear: 0.0,
            attraction: 0.0,
            last_interaction: None,
        }
    }

    pub fn adjust_opinion(&mut self, delta: f32) {
        self.opinion = (self.opinion + delta).clamp(-1.0, 1.0);
    }

    pub fn adjust_trust(&mut self, delta: f32) {
        self.trust = (self.trust + delta).clamp(0.0, 1.0);
    }

    pub fn adjust_fear(&mut self, delta: f32) {
        self.fear = (self.fear + delta).clamp(0.0, 1.0);
    }

    pub fn adjust_attraction(&mut self, delta: f32) {
        self.attraction = (self.attraction + delta).clamp(0.0, 1.0);
    }

    pub fn update_interaction(&mut self, time: WorldTime) {
        self.last_interaction = Some(time);
    }

    pub fn is_friendly(&self) -> bool {
        self.opinion > 0.3
    }

    pub fn is_hostile(&self) -> bool {
        self.opinion < -0.3
    }

    pub fn is_neutral(&self) -> bool {
        !self.is_friendly() && !self.is_hostile()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_creation() {
        let rel = Relationship::new("npc_2".to_string());
        assert_eq!(rel.opinion, 0.0);
        assert_eq!(rel.trust, 0.0);
    }

    #[test]
    fn test_adjust_opinion() {
        let mut rel = Relationship::new("npc_2".to_string());
        rel.adjust_opinion(0.5);
        assert_eq!(rel.opinion, 0.5);

        rel.adjust_opinion(1.0);
        assert_eq!(rel.opinion, 1.0);
    }

    #[test]
    fn test_relationship_status() {
        let mut rel = Relationship::new("npc_2".to_string());
        
        rel.adjust_opinion(0.5);
        assert!(rel.is_friendly());

        rel.adjust_opinion(-1.0);
        assert!(rel.is_hostile());

        rel.adjust_opinion(0.5);
        assert!(rel.is_neutral());
    }
}
