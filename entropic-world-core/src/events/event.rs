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
    /// Constructs a `WorldEvent` with the given metadata and an empty list of involved entities.
    ///
    /// # Examples
    ///
    /// ```
    /// # use entropic_world_core::events::{WorldEvent, EventType};
    /// # use entropic_world_core::time::WorldTime;
    /// let evt = WorldEvent::new(
    ///     "evt-001".to_string(),
    ///     EventType::Discovery,
    ///     WorldTime::default(),
    ///     (10.0_f32, 20.0_f32),
    ///     "Ancient ruins discovered".to_string(),
    /// );
    /// assert!(evt.involved_entities.is_empty());
    /// ```
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

    /// Sets the event's involved entities and returns the updated `WorldEvent`.
    ///
    /// Replaces the event's `involved_entities` list with the provided vector of entity IDs.
    ///
    /// # Parameters
    ///
    /// - `entities`: A vector of `EntityId` values to assign as the event's involved participants.
    ///
    /// # Returns
    ///
    /// The updated `WorldEvent` with `involved_entities` set to `entities`.
    ///
    /// # Examples
    ///
    /// ```
    /// use entropic_world_core::events::{EventType, WorldEvent};
    /// use entropic_world_core::types::{EntityId, WorldTime};
    ///
    /// let event = WorldEvent::new(
    ///     "evt_1".into(),
    ///     EventType::Discovery,
    ///     WorldTime::now(),
    ///     (10.0, 20.0),
    ///     "Found a hidden grotto".into(),
    /// ).with_entities(vec!["npc_1".into(), "npc_2".into()]);
    ///
    /// assert_eq!(event.involved_entities.len(), 2);
    /// ```
    pub fn with_entities(mut self, entities: Vec<EntityId>) -> Self {
        self.involved_entities = entities;
        self
    }

    /// Adds an entity to this event's list of involved entities if it is not already present.
    ///
    /// This method ensures `involved_entities` contains at most one instance of the given `entity_id`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use crate::events::{WorldEvent, EventType};
    /// use crate::entities::EntityId;
    /// use crate::time::WorldTime;
    ///
    /// let mut event = WorldEvent::new(
    ///     "evt-1".to_string(),
    ///     EventType::Discovery,
    ///     WorldTime::now(),
    ///     (0.0, 0.0),
    ///     "Found a relic".to_string(),
    /// );
    ///
    /// let entity = EntityId::new("npc-123");
    /// event.add_entity(entity.clone());
    /// event.add_entity(entity);
    /// assert_eq!(event.involved_entities.len(), 1);
    /// ```
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