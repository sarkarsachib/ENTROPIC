use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::temporal::time::WorldTime;
use crate::constants::DEFAULT_MEMORY_CAPACITY;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Memory {
    pub recent_events: Vec<MemoryEvent>,
    pub long_term_knowledge: HashMap<String, String>,
    capacity: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryEvent {
    pub description: String,
    pub timestamp: WorldTime,
    pub importance: f32,
}

impl Memory {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_MEMORY_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            recent_events: Vec::new(),
            long_term_knowledge: HashMap::new(),
            capacity,
        }
    }

    pub fn add_event(&mut self, description: String, timestamp: WorldTime, importance: f32) {
        let event = MemoryEvent {
            description,
            timestamp,
            importance: importance.clamp(0.0, 1.0),
        };

        self.recent_events.push(event);

        if self.recent_events.len() > self.capacity {
            self.recent_events.sort_by(|a, b| {
                b.importance
                    .partial_cmp(&a.importance)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            self.recent_events.truncate(self.capacity);
        }
    }

    pub fn add_knowledge(&mut self, key: String, value: String) {
        self.long_term_knowledge.insert(key, value);
    }

    pub fn get_knowledge(&self, key: &str) -> Option<&String> {
        self.long_term_knowledge.get(key)
    }

    pub fn forget(&mut self, key: &str) {
        self.long_term_knowledge.remove(key);
    }

    pub fn get_recent_events(&self, count: usize) -> &[MemoryEvent] {
        let end = count.min(self.recent_events.len());
        &self.recent_events[..end]
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let memory = Memory::new();
        assert!(memory.recent_events.is_empty());
        assert!(memory.long_term_knowledge.is_empty());
    }

    #[test]
    fn test_add_event() {
        let mut memory = Memory::with_capacity(3);
        memory.add_event("Event 1".to_string(), WorldTime::default(), 0.5);
        memory.add_event("Event 2".to_string(), WorldTime::default(), 0.8);

        assert_eq!(memory.recent_events.len(), 2);
    }

    #[test]
    fn test_memory_capacity() {
        let mut memory = Memory::with_capacity(2);
        memory.add_event("Event 1".to_string(), WorldTime::default(), 0.3);
        memory.add_event("Event 2".to_string(), WorldTime::default(), 0.8);
        memory.add_event("Event 3".to_string(), WorldTime::default(), 0.5);

        assert_eq!(memory.recent_events.len(), 2);
        assert_eq!(memory.recent_events[0].importance, 0.8);
    }

    #[test]
    fn test_knowledge() {
        let mut memory = Memory::new();
        memory.add_knowledge("favorite_color".to_string(), "blue".to_string());

        assert_eq!(
            memory.get_knowledge("favorite_color"),
            Some(&"blue".to_string())
        );

        memory.forget("favorite_color");
        assert_eq!(memory.get_knowledge("favorite_color"), None);
    }
}
