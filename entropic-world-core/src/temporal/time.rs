use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorldTime {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub tick: u64,
}

impl WorldTime {
    /// Constructs a WorldTime for the given date and time.
    ///
    /// The returned instance has its `tick` field initialized to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let t = WorldTime::new(2025, 1, 12, 6, 30, 45);
    /// assert_eq!(t.year, 2025);
    /// assert_eq!(t.month, 1);
    /// assert_eq!(t.day, 12);
    /// assert_eq!(t.hour, 6);
    /// assert_eq!(t.minute, 30);
    /// assert_eq!(t.second, 45);
    /// assert_eq!(t.tick, 0);
    /// ```
    pub fn new(year: u32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            tick: 0,
        }
    }

    /// Advances the internal tick counter by one and rolls over into the next second when the tick count reaches `ticks_per_second`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut t = WorldTime::new(1, 1, 1, 0, 0, 0);
    /// // single tick increments the tick counter
    /// t.advance_tick(20);
    /// assert_eq!(t.tick, 1);
    ///
    /// // advance until rollover: 19 more ticks -> second increments and tick resets to 0
    /// for _ in 0..19 {
    ///     t.advance_tick(20);
    /// }
    /// assert_eq!(t.tick, 0);
    /// assert_eq!(t.second, 1);
    /// ```
    pub fn advance_tick(&mut self, ticks_per_second: u64) {
        self.tick += 1;
        if self.tick >= ticks_per_second {
            self.tick = 0;
            self.advance_second();
        }
    }

    /// Advances the stored time by one second, rolling over to the next minute (and higher units) when necessary.
    ///
    /// Increments the `second` field; if it reaches 60 it is reset to 0 and the minute is advanced.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut t = WorldTime::new(1, 1, 1, 23, 59, 59);
    /// t.advance_second();
    /// assert_eq!(t.hour, 0);
    /// assert_eq!(t.day, 2);
    /// ```
    pub fn advance_second(&mut self) {
        self.second += 1;
        if self.second >= 60 {
            self.second = 0;
            self.advance_minute();
        }
    }

    /// Advances the time by one minute, rolling over to the next hour when the minute reaches 60.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut t = WorldTime::new(1, 1, 1, 0, 59, 0);
    /// t.advance_minute();
    /// assert_eq!(t.minute, 0);
    /// assert_eq!(t.hour, 1);
    /// ```
    pub fn advance_minute(&mut self) {
        self.minute += 1;
        if self.minute >= 60 {
            self.minute = 0;
            self.advance_hour();
        }
    }

    /// Advances the hour by one and rolls over to the next day when the hour reaches 24.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut t = WorldTime::new(1, 1, 1, 23, 0, 0);
    /// t.advance_hour();
    /// assert_eq!(t.hour, 0);
    /// assert_eq!(t.day, 2);
    /// ```
    pub fn advance_hour(&mut self) {
        self.hour += 1;
        if self.hour >= 24 {
            self.hour = 0;
            self.advance_day();
        }
    }

    /// Advances the date by one day, rolling to the next month and incrementing the year when necessary.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut t = WorldTime::new(1, 12, 31, 0, 0, 0);
    /// t.advance_day();
    /// assert_eq!(t.year, 2);
    /// assert_eq!(t.month, 1);
    /// assert_eq!(t.day, 1);
    /// ```
    pub fn advance_day(&mut self) {
        self.day += 1;
        let days_in_month = Self::days_in_month(self.month, self.year);
        if self.day > days_in_month {
            self.day = 1;
            self.advance_month();
        }
    }

    /// Advances the current month by one, rolling over to January and incrementing the year when the month exceeds 12.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut t = WorldTime::new(1, 12, 31, 0, 0, 0);
    /// t.advance_month();
    /// assert_eq!(t.month, 1);
    /// assert_eq!(t.year, 2);
    /// ```
    pub fn advance_month(&mut self) {
        self.month += 1;
        if self.month > 12 {
            self.month = 1;
            self.year += 1;
        }
    }

    /// Compute the number of days in a given month for a specified year.
    ///
    /// February accounts for leap years per Gregorian rules.
    ///
    /// # Examples
    ///
    /// ```
    /// let days = WorldTime::days_in_month(2, 2020);
    /// assert_eq!(days, 29);
    /// ```
    fn days_in_month(month: u8, year: u32) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    /// Determines whether a year is a leap year in the Gregorian calendar.
    ///
    /// # Returns
    ///
    /// `true` if the year is a leap year according to Gregorian rules, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(is_leap_year(2000)); // divisible by 400
    /// assert!(!is_leap_year(1900)); // divisible by 100 but not 400
    /// assert!(is_leap_year(2024)); // divisible by 4 but not 100
    /// assert!(!is_leap_year(2023)); // not divisible by 4
    /// ```
    fn is_leap_year(year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Computes the total number of seconds elapsed since year 1-01-01 00:00:00 for this `WorldTime`.
    ///
    /// # Returns
    ///
    /// The total elapsed seconds as a `u64`.
    ///
    /// # Examples
    ///
    /// ```
    /// let t = WorldTime::new(1, 1, 1, 0, 0, 1);
    /// assert_eq!(t.total_seconds(), 1);
    /// ```
    pub fn total_seconds(&self) -> u64 {
        let days = self.total_days();
        days * 86400 + self.hour as u64 * 3600 + self.minute as u64 * 60 + self.second as u64
    }

    /// Computes the number of days elapsed from year 1, month 1, day 1 to this date.
    ///
    /// The calculation counts full days; year 1-01-01 corresponds to 0 days elapsed. Leap years
    /// are accounted for when summing whole years and the days in prior months of the current year.
    ///
    /// # Examples
    ///
    /// ```
    /// let t = WorldTime::new(1, 1, 1, 0, 0, 0);
    /// assert_eq!(t.total_days(), 0);
    ///
    /// let t2 = WorldTime::new(1, 1, 2, 0, 0, 0);
    /// assert_eq!(t2.total_days(), 1);
    /// ```
    pub fn total_days(&self) -> u64 {
        let mut days = 0u64;
        for y in 1..self.year {
            days += if Self::is_leap_year(y) { 366 } else { 365 };
        }
        for m in 1..self.month {
            days += Self::days_in_month(m, self.year) as u64;
        }
        days += self.day as u64 - 1;
        days
    }
}

impl Default for WorldTime {
    /// Creates the canonical default WorldTime: year 1, month 1, day 1 at 06:00:00.
    ///
    /// # Returns
    ///
    /// A `WorldTime` set to year 1, month 1, day 1, 06:00:00 with `tick` initialized to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// let t = WorldTime::default();
    /// assert_eq!(t, WorldTime::new(1, 1, 1, 6, 0, 0));
    /// ```
    fn default() -> Self {
        Self::new(1, 1, 1, 6, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_advance() {
        let mut time = WorldTime::new(1, 1, 1, 23, 59, 59);
        time.advance_second();
        assert_eq!(time.hour, 0);
        assert_eq!(time.day, 2);
    }

    #[test]
    fn test_month_advance() {
        let mut time = WorldTime::new(1, 12, 31, 23, 59, 59);
        time.advance_second();
        assert_eq!(time.year, 2);
        assert_eq!(time.month, 1);
        assert_eq!(time.day, 1);
    }

    #[test]
    fn test_leap_year() {
        let mut time = WorldTime::new(4, 2, 28, 23, 59, 59);
        time.advance_second();
        assert_eq!(time.day, 29);
    }

    #[test]
    fn test_tick_advance() {
        let mut time = WorldTime::new(1, 1, 1, 0, 0, 0);
        for _ in 0..20 {
            time.advance_tick(20);
        }
        assert_eq!(time.second, 1);
    }
}