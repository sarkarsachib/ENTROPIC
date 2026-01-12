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
    /// Creates a new WorldState initialized to its default runtime values.
    ///
    /// The returned state starts with tick 0, the default `WorldTime`, not paused, and a simulation speed of 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// let ws = WorldState::new();
    /// assert_eq!(ws.current_tick, 0);
    /// assert!(!ws.is_paused);
    /// assert_eq!(ws.simulation_speed, 1.0);
    /// ```
    pub fn new() -> Self {
        Self {
            current_tick: 0,
            current_time: WorldTime::default(),
            is_paused: false,
            simulation_speed: 1.0,
        }
    }

    /// Advances the world state by one tick.
    ///
    /// Increments `current_tick` and advances `current_time` to reflect a single tick of simulation.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ws = WorldState::new();
    /// ws.advance_tick();
    /// assert_eq!(ws.current_tick, 1);
    /// ```
    pub fn advance_tick(&mut self) {
        self.current_tick += 1;
        self.current_time.advance_tick(crate::constants::DEFAULT_TICKS_PER_SECOND);
    }

    /// Pause the world's simulation.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut state = WorldState::new();
    /// state.pause();
    /// assert!(state.is_paused);
    /// ```
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Resumes the simulation by clearing the paused state.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut state = WorldState::new();
    /// state.pause();
    /// assert!(state.is_paused);
    /// state.resume();
    /// assert!(!state.is_paused);
    /// ```
    pub fn resume(&mut self) {
        self.is_paused = false;
    }

    /// Sets the simulation speed, clamping negative values to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut ws = WorldState::new();
    /// ws.set_simulation_speed(2.0);
    /// assert_eq!(ws.simulation_speed, 2.0);
    /// ws.set_simulation_speed(-1.5);
    /// assert_eq!(ws.simulation_speed, 0.0);
    /// ```
    pub fn set_simulation_speed(&mut self, speed: f32) {
        self.simulation_speed = speed.max(0.0);
    }
}

impl Default for WorldState {
    /// Creates a `WorldState` initialized with the module's canonical defaults.
    ///
    /// The resulting state has `current_tick` set to 0, `current_time` set to
    /// `WorldTime::default()`, `is_paused` set to `false`, and `simulation_speed`
    /// set to `1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let ws = WorldState::default();
    /// assert_eq!(ws.current_tick, 0);
    /// assert!(!ws.is_paused);
    /// assert_eq!(ws.simulation_speed, 1.0);
    /// ```
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