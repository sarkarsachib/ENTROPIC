use serde::{Deserialize, Serialize};
use crate::events::event::WorldEvent;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventQueue {
    events: Vec<(u64, WorldEvent)>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn schedule(&mut self, tick: u64, event: WorldEvent) {
        self.events.push((tick, event));
        self.events.sort_by_key(|e| e.0);
    }

    pub fn get_events_at_tick(&mut self, tick: u64) -> Vec<WorldEvent> {
        let mut result = Vec::new();
        let mut i = 0;
        
        while i < self.events.len() {
            if self.events[i].0 == tick {
                result.push(self.events.remove(i).1);
            } else if self.events[i].0 > tick {
                break;
            } else {
                i += 1;
            }
        }
        
        result
    }

    pub fn peek_next(&self) -> Option<(&u64, &WorldEvent)> {
        self.events.first().map(|(tick, event)| (tick, event))
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn get_events_until(&mut self, tick: u64) -> Vec<WorldEvent> {
        let mut result = Vec::new();
        
        while let Some(&(event_tick, _)) = self.events.first() {
            if event_tick <= tick {
                result.push(self.events.remove(0).1);
            } else {
                break;
            }
        }
        
        result
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::event::EventType;
    use crate::temporal::time::WorldTime;

    #[test]
    fn test_event_queue_creation() {
        let queue = EventQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_schedule_event() {
        let mut queue = EventQueue::new();
        let event = WorldEvent::new(
            "event_1".to_string(),
            EventType::NPCBirth,
            WorldTime::default(),
            (0.0, 0.0),
            "Test event".to_string(),
        );

        queue.schedule(100, event);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_get_events_at_tick() {
        let mut queue = EventQueue::new();
        
        for i in 0..3 {
            let event = WorldEvent::new(
                format!("event_{}", i),
                EventType::Custom("test".to_string()),
                WorldTime::default(),
                (0.0, 0.0),
                format!("Event {}", i),
            );
            queue.schedule(100, event);
        }

        let events = queue.get_events_at_tick(100);
        assert_eq!(events.len(), 3);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_event_ordering() {
        let mut queue = EventQueue::new();
        
        let event1 = WorldEvent::new(
            "event_1".to_string(),
            EventType::NPCBirth,
            WorldTime::default(),
            (0.0, 0.0),
            "Event 1".to_string(),
        );
        let event2 = WorldEvent::new(
            "event_2".to_string(),
            EventType::NPCDeath,
            WorldTime::default(),
            (0.0, 0.0),
            "Event 2".to_string(),
        );

        queue.schedule(200, event2);
        queue.schedule(100, event1);

        let next = queue.peek_next();
        assert!(next.is_some());
        assert_eq!(*next.unwrap().0, 100);
    }

    #[test]
    fn test_get_events_until() {
        let mut queue = EventQueue::new();
        
        for i in 0..5 {
            let event = WorldEvent::new(
                format!("event_{}", i),
                EventType::Custom("test".to_string()),
                WorldTime::default(),
                (0.0, 0.0),
                format!("Event {}", i),
            );
            queue.schedule(i * 100, event);
        }

        let events = queue.get_events_until(250);
        assert_eq!(events.len(), 3);
        assert_eq!(queue.len(), 2);
    }
}
