use entropic_world_core::temporal::time::WorldTime;
use entropic_world_core::temporal::weather::{Weather, WeatherCondition};
use entropic_world_core::temporal::seasons::Season;

#[test]
fn test_time_advancement() {
    let mut time = WorldTime::new(1, 1, 1, 0, 0, 0);
    
    for _ in 0..60 {
        time.advance_second();
    }
    
    assert_eq!(time.minute, 1);
    assert_eq!(time.second, 0);
}

#[test]
fn test_day_transition() {
    let mut time = WorldTime::new(1, 1, 1, 23, 59, 59);
    time.advance_second();
    
    assert_eq!(time.day, 2);
    assert_eq!(time.hour, 0);
    assert_eq!(time.minute, 0);
    assert_eq!(time.second, 0);
}

#[test]
fn test_month_transition() {
    let mut time = WorldTime::new(1, 1, 31, 23, 59, 59);
    time.advance_second();
    
    assert_eq!(time.month, 2);
    assert_eq!(time.day, 1);
}

#[test]
fn test_year_transition() {
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
    assert_eq!(time.month, 2);
}

#[test]
fn test_weather_conditions() {
    let weather = Weather::default();
    assert_eq!(weather.condition, WeatherCondition::Clear);
    assert!(weather.is_clear());
    assert!(!weather.is_raining());
}

#[test]
fn test_season_from_month() {
    assert_eq!(Season::from_month(3), Season::Spring);
    assert_eq!(Season::from_month(7), Season::Summer);
    assert_eq!(Season::from_month(10), Season::Fall);
    assert_eq!(Season::from_month(12), Season::Winter);
}

#[test]
fn test_tick_advancement() {
    let mut time = WorldTime::new(1, 1, 1, 0, 0, 0);
    
    for _ in 0..20 {
        time.advance_tick(20);
    }
    
    assert_eq!(time.second, 1);
    assert_eq!(time.tick, 0);
}
