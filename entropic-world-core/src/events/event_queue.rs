use serde::{Deserialize, Serialize};
use crate::events::event::WorldEvent;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventQueue {
    events: Vec<(u64, WorldEvent)>,
}

impl EventQueue {
    /// Creates an empty EventQueue.
    ///
    /// Returns an `EventQueue` with no scheduled events.
    ///
    /// # Examples
    ///
    /// ```
    /// let q = EventQueue::new();
    /// assert!(q.is_empty());
    /// assert_eq!(q.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Schedules a `WorldEvent` to be executed at the specified tick.
    ///
    /// The event is appended to the queue and the queue is kept sorted in ascending tick order.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut q = EventQueue::new();
    /// let event = /* construct a WorldEvent here */;
    /// q.schedule(100, event);
    /// assert_eq!(q.len(), 1);
    /// ```
    pub fn schedule(&mut self, tick: u64, event: WorldEvent) {
        self.events.push((tick, event));
        self.events.sort_by_key(|e| e.0);
    }

    /// Removes and returns all events scheduled exactly at `tick`.
    ///
    /// Scans the queue from the front and removes any events whose tick equals the given value.
    /// Scanning stops once an event with a greater tick is encountered.
    ///
    /// # Returns
    ///
    /// A `Vec<WorldEvent>` containing the removed events; empty if no events were scheduled at `tick`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut q = EventQueue::new();
    /// assert!(q.get_events_at_tick(0).is_empty());
    /// ```
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

    /// Returns a reference to the next scheduled event (the entry with the smallest tick) without removing it.
    ///
    /// # Returns
    ///
    /// `Some((&u64, &WorldEvent))` containing the tick and corresponding `WorldEvent` for the earliest scheduled event, `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let q = EventQueue::new();
    /// // q.peek_next() returns None for an empty queue
    /// assert!(q.peek_next().is_none());
    /// // After scheduling, peek_next yields the earliest tick and event reference
    /// // q.schedule(10, some_event);
    /// // let (tick, event) = q.peek_next().unwrap();
    /// // assert_eq!(*tick, 10);
    /// ```
    pub fn peek_next(&self) -> Option<(&u64, &WorldEvent)> {
        self.events.first().map(|(tick, event)| (tick, event))
    }

    /// Get the number of scheduled events in the queue.
    ///
    /// # Returns
    ///
    /// `usize` representing the number of scheduled events.
    ///
    /// # Examples
    ///
    /// ```
    /// let q = EventQueue::new();
    /// assert_eq!(q.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Checks whether the event queue contains no scheduled events.
    ///
    /// # Examples
    ///
    /// ```
    /// let q = EventQueue::new();
    /// assert!(q.is_empty());
    /// ```
    ///
    /// # Returns
    ///
    /// `true` if there are no scheduled events, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Removes all scheduled events from the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut q = EventQueue::new();
    /// assert_eq!(q.len(), 0);
    /// q.clear();
    /// assert_eq!(q.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Removes and returns all scheduled events whose tick is less than or equal to `tick`.
    ///
    /// The returned vector contains the removed `WorldEvent` items in the same chronological order
    /// they were scheduled.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut q = EventQueue::new();
    /// // let evt = /* construct a WorldEvent value */;
    /// // q.schedule(5, evt);
    /// let taken = q.get_events_until(10);
    /// // `taken` now contains all events scheduled at or before tick 10
    /// ```
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
    /// Constructs a new EventQueue.
    ///
    /// # Examples
    ///
    /// ```
    /// let q = EventQueue::default();
    /// assert!(q.is_empty());
    /// ```
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