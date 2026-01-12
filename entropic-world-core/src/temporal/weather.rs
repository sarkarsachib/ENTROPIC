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
    /// Creates a Weather instance populated with sensible default values.
    ///
    /// The default weather is `Clear` with a temperature of 15.0°C, 0.5 humidity,
    /// 5.0 m/s wind speed, 0.0° wind direction, and 0.0 precipitation.
    ///
    /// # Examples
    ///
    /// ```
    /// let w = Weather::default();
    /// assert_eq!(w.condition, WeatherCondition::Clear);
    /// assert_eq!(w.temperature, 15.0);
    /// ```
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
    /// Creates a Weather with the given condition and temperature.
    ///
    /// Other fields are set to their default values (humidity = 0.5, wind_speed = 5.0,
    /// wind_direction = 0.0, precipitation = 0.0).
    ///
    /// # Parameters
    ///
    /// - `condition`: The weather condition variant to set.
    /// - `temperature`: The temperature in degrees Celsius.
    ///
    /// # Returns
    ///
    /// A `Weather` instance with the specified `condition` and `temperature`.
    ///
    /// # Examples
    ///
    /// ```
    /// let w = Weather::new(WeatherCondition::Rainy, 10.0);
    /// assert_eq!(w.condition, WeatherCondition::Rainy);
    /// assert_eq!(w.temperature, 10.0);
    /// ```
    pub fn new(condition: WeatherCondition, temperature: f32) -> Self {
        Self {
            condition,
            temperature,
            ..Default::default()
        }
    }

    /// Reports whether the weather represents rain or a storm.
    ///
    /// # Returns
    ///
    /// `true` if the condition is `WeatherCondition::Rainy` or `WeatherCondition::Stormy`, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let rainy = Weather::new(WeatherCondition::Rainy, 10.0);
    /// assert!(rainy.is_raining());
    ///
    /// let clear = Weather::default();
    /// assert!(!clear.is_raining());
    /// ```
    pub fn is_raining(&self) -> bool {
        matches!(
            self.condition,
            WeatherCondition::Rainy | WeatherCondition::Stormy
        )
    }

    /// Checks whether the weather condition is clear.
    ///
    /// # Examples
    ///
    /// ```
    /// let w = Weather::default();
    /// assert!(w.is_clear());
    /// ```
    ///
    /// # Returns
    ///
    /// `true` if the condition is `WeatherCondition::Clear`, `false` otherwise.
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