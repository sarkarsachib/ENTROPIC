use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Weather {
    pub condition: WeatherCondition,
    pub temperature: f32,
    pub humidity: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub precipitation: f32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeatherCondition {
    Clear,
    Cloudy,
    Rainy,
    Stormy,
    Snowy,
    Fog,
}

impl Default for Weather {
    fn default() -> Self {
        Self {
            condition: WeatherCondition::Clear,
            temperature: 15.0,
            humidity: 0.5,
            wind_speed: 5.0,
            wind_direction: 0.0,
            precipitation: 0.0,
        }
    }
}

impl Weather {
    pub fn new(condition: WeatherCondition, temperature: f32) -> Self {
        Self {
            condition,
            temperature,
            ..Default::default()
        }
    }

    pub fn is_raining(&self) -> bool {
        matches!(
            self.condition,
            WeatherCondition::Rainy | WeatherCondition::Stormy
        )
    }

    pub fn is_clear(&self) -> bool {
        self.condition == WeatherCondition::Clear
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_default() {
        let weather = Weather::default();
        assert_eq!(weather.condition, WeatherCondition::Clear);
        assert_eq!(weather.temperature, 15.0);
    }

    #[test]
    fn test_weather_is_raining() {
        let rainy = Weather::new(WeatherCondition::Rainy, 10.0);
        assert!(rainy.is_raining());

        let clear = Weather::default();
        assert!(!clear.is_raining());
    }
}
