use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl Season {
    pub fn from_month(month: u8) -> Self {
        match month {
            3 | 4 | 5 => Season::Spring,
            6 | 7 | 8 => Season::Summer,
            9 | 10 | 11 => Season::Fall,
            _ => Season::Winter,
        }
    }

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
