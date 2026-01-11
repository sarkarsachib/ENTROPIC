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

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_time_scale(mut self, time_scale: f32) -> Self {
        self.time_scale = time_scale;
        self
    }

    pub fn disable_weather(mut self) -> Self {
        self.weather_enabled = false;
        self
    }

    pub fn disable_seasons(mut self) -> Self {
        self.seasons_enabled = false;
        self
    }

    pub fn disable_day_night_cycle(mut self) -> Self {
        self.day_night_cycle_enabled = false;
        self
    }

    pub fn disable_economy(mut self) -> Self {
        self.economy_enabled = false;
        self
    }

    pub fn disable_ai(mut self) -> Self {
        self.ai_enabled = false;
        self
    }

    pub fn non_persistent(mut self) -> Self {
        self.persistent = false;
        self
    }
}

impl Default for WorldConfig {
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
