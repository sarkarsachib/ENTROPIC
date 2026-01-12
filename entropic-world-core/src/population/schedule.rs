use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub routines: HashMap<DayType, Vec<ScheduleEntry>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleEntry {
    pub start_hour: u8,
    pub end_hour: u8,
    pub activity: Activity,
    pub location: Option<(f32, f32)>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Activity {
    Work,
    Sleep,
    Eat,
    Socialize,
    Travel,
    Combat,
    Custom,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DayType {
    Weekday,
    Weekend,
    Festival,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            routines: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, day_type: DayType, entry: ScheduleEntry) {
        self.routines
            .entry(day_type)
            .or_insert_with(Vec::new)
            .push(entry);
    }

    pub fn get_activity_at(&self, day_type: DayType, hour: u8) -> Option<&Activity> {
        self.routines.get(&day_type).and_then(|entries| {
            entries
                .iter()
                .find(|e| e.start_hour <= hour && hour < e.end_hour)
                .map(|e| &e.activity)
        })
    }
}

impl Default for Schedule {
    fn default() -> Self {
        let mut schedule = Self::new();

        schedule.add_entry(
            DayType::Weekday,
            ScheduleEntry {
                start_hour: 0,
                end_hour: 7,
                activity: Activity::Sleep,
                location: None,
            },
        );

        schedule.add_entry(
            DayType::Weekday,
            ScheduleEntry {
                start_hour: 7,
                end_hour: 8,
                activity: Activity::Eat,
                location: None,
            },
        );

        schedule.add_entry(
            DayType::Weekday,
            ScheduleEntry {
                start_hour: 8,
                end_hour: 17,
                activity: Activity::Work,
                location: None,
            },
        );

        schedule.add_entry(
            DayType::Weekday,
            ScheduleEntry {
                start_hour: 17,
                end_hour: 20,
                activity: Activity::Socialize,
                location: None,
            },
        );

        schedule.add_entry(
            DayType::Weekday,
            ScheduleEntry {
                start_hour: 20,
                end_hour: 24,
                activity: Activity::Sleep,
                location: None,
            },
        );

        schedule
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_creation() {
        let schedule = Schedule::new();
        assert!(schedule.routines.is_empty());
    }

    #[test]
    fn test_schedule_add_entry() {
        let mut schedule = Schedule::new();
        schedule.add_entry(
            DayType::Weekday,
            ScheduleEntry {
                start_hour: 9,
                end_hour: 17,
                activity: Activity::Work,
                location: None,
            },
        );

        let activity = schedule.get_activity_at(DayType::Weekday, 12);
        assert_eq!(activity, Some(&Activity::Work));
    }

    #[test]
    fn test_schedule_default() {
        let schedule = Schedule::default();
        assert_eq!(
            schedule.get_activity_at(DayType::Weekday, 3),
            Some(&Activity::Sleep)
        );
        assert_eq!(
            schedule.get_activity_at(DayType::Weekday, 12),
            Some(&Activity::Work)
        );
    }
}
