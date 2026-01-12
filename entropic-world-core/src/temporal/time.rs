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

    pub fn advance_tick(&mut self, ticks_per_second: u64) {
        self.tick += 1;
        if self.tick >= ticks_per_second {
            self.tick = 0;
            self.advance_second();
        }
    }

    pub fn advance_second(&mut self) {
        self.second += 1;
        if self.second >= 60 {
            self.second = 0;
            self.advance_minute();
        }
    }

    pub fn advance_minute(&mut self) {
        self.minute += 1;
        if self.minute >= 60 {
            self.minute = 0;
            self.advance_hour();
        }
    }

    pub fn advance_hour(&mut self) {
        self.hour += 1;
        if self.hour >= 24 {
            self.hour = 0;
            self.advance_day();
        }
    }

    pub fn advance_day(&mut self) {
        self.day += 1;
        let days_in_month = Self::days_in_month(self.month, self.year);
        if self.day > days_in_month {
            self.day = 1;
            self.advance_month();
        }
    }

    pub fn advance_month(&mut self) {
        self.month += 1;
        if self.month > 12 {
            self.month = 1;
            self.year += 1;
        }
    }

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

    fn is_leap_year(year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    pub fn total_seconds(&self) -> u64 {
        let days = self.total_days();
        days * 86400 + self.hour as u64 * 3600 + self.minute as u64 * 60 + self.second as u64
    }

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
