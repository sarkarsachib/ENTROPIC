use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl Season {
    /// Determines the season corresponding to a month number.
    ///
    /// Months 3–5 map to Spring, 6–8 to Summer, 9–11 to Fall, and all other values map to Winter.
    ///
    /// # Parameters
    ///
    /// - `month`: Month number where 1 represents January and 12 represents December; values outside 1..=12 are treated as Winter.
    ///
    /// # Returns
    ///
    /// `Season` corresponding to the provided month.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Season::from_month(3), Season::Spring);
    /// assert_eq!(Season::from_month(7), Season::Summer);
    /// assert_eq!(Season::from_month(10), Season::Fall);
    /// assert_eq!(Season::from_month(1), Season::Winter);
    /// ```
    pub fn from_month(month: u8) -> Self {
        match month {
            3 | 4 | 5 => Season::Spring,
            6 | 7 | 8 => Season::Summer,
            9 | 10 | 11 => Season::Fall,
            _ => Season::Winter,
        }
    }

    /// Per-season temperature modifier in degrees Celsius.
    ///
    /// Returns the temperature offset associated with the season:
    /// `0.0` for Spring, `10.0` for Summer, `-5.0` for Fall, and `-15.0` for Winter.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::Season;
    ///
    /// assert_eq!(Season::Summer.temperature_modifier(), 10.0);
    /// assert_eq!(Season::Winter.temperature_modifier(), -15.0);
    /// ```
    pub fn temperature_modifier(&self) -> f32 {
        match self {
            Season::Spring => 0.0,
            Season::Summer => 10.0,
            Season::Fall => -5.0,
            Season::Winter => -15.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_season_from_month() {
        assert_eq!(Season::from_month(3), Season::Spring);
        assert_eq!(Season::from_month(7), Season::Summer);
        assert_eq!(Season::from_month(10), Season::Fall);
        assert_eq!(Season::from_month(1), Season::Winter);
    }

    #[test]
    fn test_temperature_modifier() {
        assert_eq!(Season::Summer.temperature_modifier(), 10.0);
        assert_eq!(Season::Winter.temperature_modifier(), -15.0);
    }
}