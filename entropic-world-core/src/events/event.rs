use serde::{Deserialize, Serialize};
use crate::population::entity::EntityId;
use crate::temporal::time::WorldTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldEvent {
    pub id: String,
    pub event_type: EventType,
    pub timestamp: WorldTime,
    pub location: (f32, f32),
    pub involved_entities: Vec<EntityId>,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    NPCBirth,
    NPCDeath,
    NPCMarriage,
    FactionWar,
    Settlement,
    TradeCompleted,
    Disaster,
    Discovery,
    Custom(String),
}

impl WorldEvent {
    pub fn new(
        id: String,
        event_type: EventType,
        timestamp: WorldTime,
        location: (f32, f32),
        description: String,
    ) -> Self {
        Self {
            id,
            event_type,
            timestamp,
            location,
            involved_entities: Vec::new(),
            description,
        }
    }

    pub fn with_entities(mut self, entities: Vec<EntityId>) -> Self {
        self.involved_entities = entities;
        self
    }

    pub fn add_entity(&mut self, entity_id: EntityId) {
        if !self.involved_entities.contains(&entity_id) {
            self.involved_entities.push(entity_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = WorldEvent::new(
            "event_1".to_string(),
            EventType::NPCBirth,
            WorldTime::default(),
            (100.0, 200.0),
            "A new NPC was born".to_string(),
        );
        assert_eq!(event.event_type, EventType::NPCBirth);
        assert_eq!(event.location, (100.0, 200.0));
    }

    #[test]
    fn test_event_with_entities() {
        let event = WorldEvent::new(
            "event_1".to_string(),
            EventType::TradeCompleted,
            WorldTime::default(),
            (100.0, 200.0),
            "Trade completed".to_string(),
        )
        .with_entities(vec!["npc_1".to_string(), "npc_2".to_string()]);

        assert_eq!(event.involved_entities.len(), 2);
    }

    #[test]
    fn test_add_entity() {
        let mut event = WorldEvent::new(
            "event_1".to_string(),
            EventType::Discovery,
            WorldTime::default(),
            (100.0, 200.0),
            "New location discovered".to_string(),
        );

        event.add_entity("explorer_1".to_string());
        assert_eq!(event.involved_entities.len(), 1);

        event.add_entity("explorer_1".to_string());
        assert_eq!(event.involved_entities.len(), 1);
    }
}
