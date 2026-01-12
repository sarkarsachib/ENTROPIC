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
    /// Creates a `Memory` initialized with the default capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// let mem = Memory::new();
    /// assert!(mem.recent_events.is_empty());
    /// assert!(mem.long_term_knowledge.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_MEMORY_CAPACITY)
    }

    /// Creates a `Memory` with the specified capacity for recent events.
    ///
    /// The `capacity` sets the maximum number of recent events the memory will retain.
    /// The returned `Memory` has an empty `recent_events` vector and an empty
    /// `long_term_knowledge` map.
    ///
    /// # Examples
    ///
    /// ```
    /// let mem = entropic_world_core::population::memory::Memory::with_capacity(5);
    /// assert!(mem.recent_events.is_empty());
    /// assert!(mem.long_term_knowledge.is_empty());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            recent_events: Vec::new(),
            long_term_knowledge: HashMap::new(),
            capacity,
        }
    }

    /// Appends a new memory event and enforces the memory's capacity, keeping the highest-importance events.
    ///
    /// The provided `importance` is clamped to the range 0.0 through 1.0 before storing. If adding the event causes
    /// the number of recent events to exceed the memory's capacity, the stored events are reduced so only the top
    /// important events (up to capacity) remain.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mem = Memory::with_capacity(2);
    /// mem.add_event("note A".to_string(), WorldTime::default(), 0.8);
    /// mem.add_event("note B".to_string(), WorldTime::default(), 0.9);
    /// mem.add_event("note C".to_string(), WorldTime::default(), 1.2); // importance will be clamped to 1.0
    /// assert!(mem.get_recent_events(2).len() <= 2);
    /// ```
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

    /// Stores or updates a long-term knowledge entry identified by `key`.
    ///
    /// Inserts the provided key-value pair into the memory's long-term knowledge,
    /// replacing any existing value for the same key.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mem = Memory::new();
    /// mem.add_knowledge("favorite_color".to_string(), "blue".to_string());
    /// assert_eq!(mem.get_knowledge("favorite_color").map(|s| s.as_str()), Some("blue"));
    /// ```
    pub fn add_knowledge(&mut self, key: String, value: String) {
        self.long_term_knowledge.insert(key, value);
    }

    /// Retrieves the stored long-term knowledge value for the given key, if any.
    ///
    /// # Returns
    ///
    /// `Some(&String)` containing the value associated with `key`, `None` if the key is not present.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mem = Memory::new();
    /// mem.add_knowledge("favorite_color".into(), "blue".into());
    /// assert_eq!(mem.get_knowledge("favorite_color"), Some(&"blue".to_string()));
    /// ```
    pub fn get_knowledge(&self, key: &str) -> Option<&String> {
        self.long_term_knowledge.get(key)
    }

    /// Removes the long-term knowledge entry associated with `key`.
    ///
    /// # Params
    ///
    /// - `key`: The key of the knowledge entry to remove.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut m = Memory::with_capacity(1);
    /// m.add_knowledge("favorite_color".to_string(), "blue".to_string());
    /// m.forget("favorite_color");
    /// assert!(m.get_knowledge("favorite_color").is_none());
    /// ```
    pub fn forget(&mut self, key: &str) {
        self.long_term_knowledge.remove(key);
    }

    /// Get a slice of the most recent memory events up to the requested count.
    ///
    /// # Parameters
    ///
    /// - `count`: Maximum number of events to return.
    ///
    /// # Returns
    ///
    /// A slice containing up to `count` of the most recent `MemoryEvent` entries; if fewer events are stored, the slice will be shorter.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mem = Memory::with_capacity(4);
    /// mem.add_event("a".into(), WorldTime::default(), 0.5);
    /// mem.add_event("b".into(), WorldTime::default(), 0.7);
    /// let recent = mem.get_recent_events(1);
    /// assert_eq!(recent.len(), 1);
    /// assert_eq!(recent[0].description, "a");
    /// ```
    pub fn get_recent_events(&self, count: usize) -> &[MemoryEvent] {
        let end = count.min(self.recent_events.len());
        &self.recent_events[..end]
    }
}

impl Default for Memory {
    /// Creates a Memory initialized with the module's default capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// let m = Memory::default();
    /// assert!(m.recent_events.is_empty());
    /// assert!(m.long_term_knowledge.is_empty());
    /// ```
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