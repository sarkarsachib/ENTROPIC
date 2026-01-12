use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventTrigger {
    pub id: String,
    pub condition: TriggerCondition,
    pub event_template: String,
    pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TriggerCondition {
    TimeElapsed(u64),
    PopulationThreshold { settlement_id: String, threshold: u32 },
    ResourceDepletion { resource: String, threshold: u32 },
    EntityDeath(String),
    FactionRelationship { faction_a: String, faction_b: String, threshold: f32 },
    Custom(String),
}

impl EventTrigger {
    /// Creates a new `EventTrigger` with the provided `id`, `condition`, and `event_template`, and marks it active.
    ///
    /// # Examples
    ///
    /// ```
    /// let trigger = EventTrigger::new(
    ///     "trigger_1".to_string(),
    ///     TriggerCondition::TimeElapsed(60),
    ///     "spawn_event".to_string(),
    /// );
    /// assert!(trigger.is_active());
    /// ```
    pub fn new(id: String, condition: TriggerCondition, event_template: String) -> Self {
        Self {
            id,
            condition,
            event_template,
            active: true,
        }
    }

    /// Marks the trigger as active.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut trigger = EventTrigger::new(
    ///     "t1".to_string(),
    ///     TriggerCondition::TimeElapsed(10),
    ///     "event_template".to_string(),
    /// );
    /// trigger.deactivate();
    /// trigger.activate();
    /// assert!(trigger.is_active());
    /// ```
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivates the trigger so it no longer fires.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut t = EventTrigger::new(
    ///     "trigger1".to_string(),
    ///     TriggerCondition::Custom("example".to_string()),
    ///     "template".to_string(),
    /// );
    /// t.deactivate();
    /// assert!(!t.is_active());
    /// ```
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Indicates whether the trigger is currently active.
    ///
    /// # Returns
    ///
    /// `true` if the trigger is active, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let trigger = EventTrigger::new(
    ///     "t1".to_string(),
    ///     TriggerCondition::TimeElapsed(10),
    ///     "event_template".to_string(),
    /// );
    /// assert!(trigger.is_active());
    /// ```
    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_creation() {
        let trigger = EventTrigger::new(
            "trigger_1".to_string(),
            TriggerCondition::TimeElapsed(1000),
            "event_template_1".to_string(),
        );
        assert!(trigger.is_active());
    }

    #[test]
    fn test_trigger_activation() {
        let mut trigger = EventTrigger::new(
            "trigger_1".to_string(),
            TriggerCondition::TimeElapsed(1000),
            "event_template_1".to_string(),
        );

        trigger.deactivate();
        assert!(!trigger.is_active());

        trigger.activate();
        assert!(trigger.is_active());
    }
}