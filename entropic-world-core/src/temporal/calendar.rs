use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Calendar {
    pub year_length_days: u16,
    pub month_names: Vec<String>,
    pub day_names: Vec<String>,
    pub season_names: Vec<String>,
    pub month_lengths: Vec<u8>,
}

impl Default for Calendar {
    fn default() -> Self {
        Self {
            year_length_days: 365,
            month_names: vec![
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
            day_names: vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            season_names: vec!["Spring", "Summer", "Fall", "Winter"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            month_lengths: vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        }
    }
}

impl Calendar {
    pub fn get_month_name(&self, month: u8) -> Option<&str> {
        if month >= 1 && (month as usize) <= self.month_names.len() {
            Some(&self.month_names[month as usize - 1])
        } else {
            None
        }
    }

    pub fn get_season(&self, month: u8) -> Option<&str> {
        let season_index = ((month as usize - 1) / 3) % self.season_names.len();
        self.season_names.get(season_index).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_default() {
        let calendar = Calendar::default();
        assert_eq!(calendar.month_names.len(), 12);
        assert_eq!(calendar.day_names.len(), 7);
    }

    #[test]
    fn test_get_month_name() {
        let calendar = Calendar::default();
        assert_eq!(calendar.get_month_name(1), Some("January"));
        assert_eq!(calendar.get_month_name(12), Some("December"));
        assert_eq!(calendar.get_month_name(13), None);
    }

    #[test]
    fn test_get_season() {
        let calendar = Calendar::default();
        assert_eq!(calendar.get_season(1), Some("Spring"));
        assert_eq!(calendar.get_season(6), Some("Summer"));
    }
}
