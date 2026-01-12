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
    /// Creates an empty Schedule with no routines.
    ///
    /// The returned Schedule has an empty `routines` map ready for entries to be added.
    ///
    /// # Examples
    ///
    /// ```
    /// let sched = entropic_world_core::population::schedule::Schedule::new();
    /// assert!(sched.routines.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            routines: HashMap::new(),
        }
    }

    /// Adds a schedule entry for a specific day type.
    ///
    /// If no entries exist for `day_type` yet, a new entry list is created and
    /// `entry` is appended to that list.
    ///
    /// # Parameters
    ///
    /// - `day_type`: The day category to which the entry should be added (e.g., Weekday, Weekend).
    /// - `entry`: The schedule entry to append.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sched = Schedule::new();
    /// sched.add_entry(
    ///     DayType::Weekday,
    ///     ScheduleEntry {
    ///         start_hour: 9,
    ///         end_hour: 17,
    ///         activity: Activity::Work,
    ///         location: None,
    ///     },
    /// );
    /// assert_eq!(sched.get_activity_at(DayType::Weekday, 12), Some(&Activity::Work));
    /// ```
    pub fn add_entry(&mut self, day_type: DayType, entry: ScheduleEntry) {
        self.routines
            .entry(day_type)
            .or_insert_with(Vec::new)
            .push(entry);
    }

    /// Finds the activity scheduled for a given day type at a specific hour.
    ///
    /// Returns `Some(&Activity)` if a `ScheduleEntry` for `day_type` contains an interval
    /// where `start_hour <= hour < end_hour`, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let sched = Schedule::default();
    /// assert_eq!(sched.get_activity_at(DayType::Weekday, 12), Some(&Activity::Work));
    /// ```
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
    /// Constructs a Schedule populated with a typical weekday routine: sleep, eat, work, socialize, and sleep.
    ///
    /// # Returns
    ///
    /// The constructed `Schedule` containing predefined `Weekday` entries covering 0–24 hours.
    ///
    /// # Examples
    ///
    /// ```
    /// let sched = Schedule::default();
    /// assert_eq!(sched.get_activity_at(DayType::Weekday, 3), Some(&Activity::Sleep));
    /// assert_eq!(sched.get_activity_at(DayType::Weekday, 12), Some(&Activity::Work));
    /// ```
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