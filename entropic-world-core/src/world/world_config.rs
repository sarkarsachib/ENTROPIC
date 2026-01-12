use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldConfig {
    pub width_chunks: u32,
    pub height_chunks: u32,
    pub time_scale: f32,
    pub weather_enabled: bool,
    pub seasons_enabled: bool,
    pub day_night_cycle_enabled: bool,
    pub economy_enabled: bool,
    pub ai_enabled: bool,
    pub persistent: bool,
    pub seed: Option<u64>,
}

impl WorldConfig {
    /// Creates a `WorldConfig` with the given chunk dimensions and sensible defaults for world generation.
    ///
    /// The returned configuration uses the crate's default time scale, enables weather, seasons,
    /// day-night cycle, economy, and AI, sets persistence to `true`, and leaves `seed` as `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = WorldConfig::new(64, 64);
    /// assert_eq!(cfg.width_chunks, 64);
    /// assert_eq!(cfg.height_chunks, 64);
    /// assert!(cfg.weather_enabled);
    /// ```
    pub fn new(width_chunks: u32, height_chunks: u32) -> Self {
        Self {
            width_chunks,
            height_chunks,
            time_scale: crate::constants::DEFAULT_TIME_SCALE,
            weather_enabled: true,
            seasons_enabled: true,
            day_night_cycle_enabled: true,
            economy_enabled: true,
            ai_enabled: true,
            persistent: true,
            seed: None,
        }
    }

    /// Sets the seed used for deterministic world generation.
    ///
    /// The seed will be stored in the returned `WorldConfig` as `Some(seed)`.
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = WorldConfig::new(32, 32).with_seed(42);
    /// assert_eq!(cfg.seed, Some(42));
    /// ```
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the world's time scale for the configuration.
    ///
    /// The `time_scale` is the multiplier applied to in-world time (e.g., `1.0` represents normal real-time).
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = WorldConfig::new(16, 16).with_time_scale(2.0);
    /// assert_eq!(cfg.time_scale, 2.0);
    /// ```
    ///
    /// # Returns
    ///
    /// `Self` with `time_scale` set to the provided value.
    pub fn with_time_scale(mut self, time_scale: f32) -> Self {
        self.time_scale = time_scale;
        self
    }

    /// Disables weather for the world configuration and returns the updated config.
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = WorldConfig::new(10, 10).disable_weather();
    /// assert!(!cfg.weather_enabled);
    /// ```
    pub fn disable_weather(mut self) -> Self {
        self.weather_enabled = false;
        self
    }

    /// Disable seasonal simulation in this `WorldConfig`.
    ///
    /// Returns the updated `WorldConfig` with `seasons_enabled` set to `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = crate::world::world_config::WorldConfig::new(10, 10).disable_seasons();
    /// assert!(!cfg.seasons_enabled);
    /// ```
    pub fn disable_seasons(mut self) -> Self {
        self.seasons_enabled = false;
        self
    }

    /// Disables the day–night cycle for this `WorldConfig`.
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = WorldConfig::new(16, 16).disable_day_night_cycle();
    /// assert!(!cfg.day_night_cycle_enabled);
    /// ```
    pub fn disable_day_night_cycle(mut self) -> Self {
        self.day_night_cycle_enabled = false;
        self
    }

    /// Disables the economy feature on this `WorldConfig` and returns the modified config.
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = WorldConfig::new(10, 10).disable_economy();
    /// assert!(!cfg.economy_enabled);
    /// ```
    ///
    /// # Returns
    ///
    /// `Self` with `economy_enabled` set to `false`.
    pub fn disable_economy(mut self) -> Self {
        self.economy_enabled = false;
        self
    }

    /// Disables AI for the world configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = WorldConfig::new(10, 10).disable_ai();
    /// assert!(!cfg.ai_enabled);
    /// ```
    ///
    /// Returns the updated `WorldConfig` with `ai_enabled` set to `false`.
    pub fn disable_ai(mut self) -> Self {
        self.ai_enabled = false;
        self
    }

    /// Marks the configuration as non-persistent and returns the modified config.
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = WorldConfig::new(10, 10).non_persistent();
    /// assert!(!cfg.persistent);
    /// ```
    pub fn non_persistent(mut self) -> Self {
        self.persistent = false;
        self
    }
}

impl Default for WorldConfig {
    /// Constructs a WorldConfig initialized with the standard default dimensions and settings.
    ///
    /// The default configuration represents a 64×64 chunk world with the module's standard time scale,
    /// all feature toggles enabled, persistence enabled, and no seed.
    ///
    /// # Examples
    ///
    /// ```
    /// let cfg = crate::world::world_config::WorldConfig::default();
    /// assert_eq!(cfg.width_chunks, 64);
    /// assert_eq!(cfg.height_chunks, 64);
    /// assert!(cfg.weather_enabled);
    /// ```
    fn default() -> Self {
        Self::new(64, 64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_config_default() {
        let config = WorldConfig::default();
        assert_eq!(config.width_chunks, 64);
        assert_eq!(config.height_chunks, 64);
        assert!(config.weather_enabled);
    }

    #[test]
    fn test_world_config_builder() {
        let config = WorldConfig::new(100, 100)
            .with_seed(42)
            .with_time_scale(2.0)
            .disable_weather();

        assert_eq!(config.seed, Some(42));
        assert_eq!(config.time_scale, 2.0);
        assert!(!config.weather_enabled);
    }
}