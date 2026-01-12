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
    /// Creates a new `Relationship` for the given NPC with all metrics initialized to defaults.
    ///
    /// The created relationship has `opinion = 0.0`, `trust = 0.0`, `fear = 0.0`, `attraction = 0.0`,
    /// and no recorded `last_interaction`.
    ///
    /// # Examples
    ///
    /// ```
    /// let rel = Relationship::new("npc_123".to_string());
    /// assert_eq!(rel.target, "npc_123");
    /// assert_eq!(rel.opinion, 0.0);
    /// assert_eq!(rel.trust, 0.0);
    /// assert_eq!(rel.fear, 0.0);
    /// assert_eq!(rel.attraction, 0.0);
    /// assert!(rel.last_interaction.is_none());
    /// ```
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

    /// Adjusts the relationship's opinion by the given delta and clamps the result to the range [-1.0, 1.0].
    ///
    /// Positive `delta` increases opinion; negative `delta` decreases opinion. This method mutates `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut r = Relationship::new("npc_a".to_string());
    /// r.adjust_opinion(0.6);
    /// assert!(r.opinion > 0.5);
    /// r.adjust_opinion(1.0);
    /// assert_eq!(r.opinion, 1.0);
    /// r.adjust_opinion(-2.5);
    /// assert_eq!(r.opinion, -1.0);
    /// ```
    pub fn adjust_opinion(&mut self, delta: f32) {
        self.opinion = (self.opinion + delta).clamp(-1.0, 1.0);
    }

    /// Adjusts the relationship's trust by the given delta and clamps the value to the range 0.0 through 1.0.
    ///
    /// - `delta`: amount to add to the current trust; positive values increase trust, negative values decrease it.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut rel = Relationship::new("npc_1".into());
    /// rel.adjust_trust(0.5);
    /// assert_eq!(rel.trust, 0.5);
    ///
    /// rel.adjust_trust(1.0);
    /// assert_eq!(rel.trust, 1.0); // clamped to upper bound
    ///
    /// rel.adjust_trust(-2.0);
    /// assert_eq!(rel.trust, 0.0); // clamped to lower bound
    /// ```
    pub fn adjust_trust(&mut self, delta: f32) {
        self.trust = (self.trust + delta).clamp(0.0, 1.0);
    }

    /// Adjusts the relationship's fear by the given amount, clamping the result to the range 0.0..=1.0.
    ///
    /// # Parameters
    ///
    /// * `delta` — Amount to add to the current fear value; positive increases fear, negative decreases it.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut rel = Relationship::new("npc_1".to_string());
    /// rel.adjust_fear(0.5);
    /// assert_eq!(rel.fear, 0.5);
    /// rel.adjust_fear(-0.8);
    /// assert_eq!(rel.fear, 0.0);
    /// ```
    pub fn adjust_fear(&mut self, delta: f32) {
        self.fear = (self.fear + delta).clamp(0.0, 1.0);
    }

    /// Adjusts the attraction value by `delta`, ensuring the result stays between 0.0 and 1.0 inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut r = Relationship::new("npc_1".to_string());
    /// r.adjust_attraction(0.2);
    /// assert_eq!(r.attraction, 0.2_f32);
    /// r.adjust_attraction(-0.5);
    /// assert_eq!(r.attraction, 0.0_f32); // clamped
    /// r.adjust_attraction(2.0);
    /// assert_eq!(r.attraction, 1.0_f32); // clamped
    /// ```
    pub fn adjust_attraction(&mut self, delta: f32) {
        self.attraction = (self.attraction + delta).clamp(0.0, 1.0);
    }

    /// Record a timestamp for the most recent interaction.
    ///
    /// Sets the relationship's `last_interaction` to the provided `time`.
    ///
    /// # Examples
    ///
    /// ```
    /// use entropic_world_core::population::relationship::Relationship;
    /// use entropic_world_core::worldtime::WorldTime;
    ///
    /// let mut rel = Relationship::new("npc_1".to_string());
    /// let t = WorldTime::from_seconds(0);
    /// rel.update_interaction(t);
    /// assert!(rel.last_interaction.is_some());
    /// ```
    pub fn update_interaction(&mut self, time: WorldTime) {
        self.last_interaction = Some(time);
    }

    /// Determines whether the relationship is considered friendly.
    ///
    /// # Returns
    ///
    /// `true` if `opinion` is greater than 0.3, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut rel = Relationship::new("npc_1".to_string());
    /// rel.adjust_opinion(0.5);
    /// assert!(rel.is_friendly());
    /// ```
    pub fn is_friendly(&self) -> bool {
        self.opinion > 0.3
    }

    /// Determines whether the relationship is considered hostile.
    ///
    /// Returns `true` if the stored opinion is less than -0.3, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut r = Relationship::new("npc_b".to_string());
    /// r.adjust_opinion(-0.5);
    /// assert!(r.is_hostile());
    /// ```
    pub fn is_hostile(&self) -> bool {
        self.opinion < -0.3
    }

    /// Determines whether the relationship is neutral (neither friendly nor hostile).
    ///
    /// # Returns
    ///
    /// `true` if `opinion` is greater than or equal to -0.3 and less than or equal to 0.3, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut r = Relationship::new("npc".to_string());
    /// assert!(r.is_neutral());
    /// r.adjust_opinion(0.5);
    /// assert!(!r.is_neutral());
    /// ```
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