use serde::{Deserialize, Serialize};
use crate::temporal::time::WorldTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldState {
    pub current_tick: u64,
    pub current_time: WorldTime,
    pub is_paused: bool,
    pub simulation_speed: f32,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            current_tick: 0,
            current_time: WorldTime::default(),
            is_paused: false,
            simulation_speed: 1.0,
        }
    }

    pub fn advance_tick(&mut self) {
        self.current_tick += 1;
        self.current_time.advance_tick(crate::constants::DEFAULT_TICKS_PER_SECOND);
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
    }

    pub fn set_simulation_speed(&mut self, speed: f32) {
        self.simulation_speed = speed.max(0.0);
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_state_creation() {
        let state = WorldState::new();
        assert_eq!(state.current_tick, 0);
        assert!(!state.is_paused);
    }

    #[test]
    fn test_advance_tick() {
        let mut state = WorldState::new();
        state.advance_tick();
        assert_eq!(state.current_tick, 1);
    }

    #[test]
    fn test_pause_resume() {
        let mut state = WorldState::new();
        state.pause();
        assert!(state.is_paused);
        state.resume();
        assert!(!state.is_paused);
    }

    #[test]
    fn test_simulation_speed() {
        let mut state = WorldState::new();
        state.set_simulation_speed(2.0);
        assert_eq!(state.simulation_speed, 2.0);
    }
}
