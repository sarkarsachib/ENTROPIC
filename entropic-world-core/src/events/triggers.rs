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
    pub fn new(id: String, condition: TriggerCondition, event_template: String) -> Self {
        Self {
            id,
            condition,
            event_template,
            active: true,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

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
