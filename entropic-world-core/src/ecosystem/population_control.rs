use serde::{Deserialize, Serialize};
use crate::ecosystem::species::SpeciesId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PopulationControl {
    pub species_id: SpeciesId,
    pub current_population: u32,
    pub birth_rate: f32,
    pub death_rate: f32,
    pub carrying_capacity: u32,
}

impl PopulationControl {
    pub fn new(species_id: SpeciesId, initial_population: u32, carrying_capacity: u32) -> Self {
        Self {
            species_id,
            current_population: initial_population,
            birth_rate: 0.1,
            death_rate: 0.05,
            carrying_capacity,
        }
    }

    pub fn simulate_growth(&mut self) {
        let births = (self.current_population as f32 * self.birth_rate) as u32;
        let deaths = (self.current_population as f32 * self.death_rate) as u32;

        self.current_population = self.current_population.saturating_add(births);
        self.current_population = self.current_population.saturating_sub(deaths);

        if self.current_population > self.carrying_capacity {
            let excess_deaths = self.current_population - self.carrying_capacity;
            self.current_population = self.carrying_capacity;
            self.death_rate += excess_deaths as f32 * 0.01;
        }

        self.death_rate = self.death_rate.max(0.0).min(1.0);
    }

    pub fn add_individuals(&mut self, count: u32) {
        self.current_population = (self.current_population + count).min(self.carrying_capacity);
    }

    pub fn remove_individuals(&mut self, count: u32) {
        self.current_population = self.current_population.saturating_sub(count);
    }

    pub fn is_overpopulated(&self) -> bool {
        self.current_population > self.carrying_capacity
    }

    pub fn is_endangered(&self) -> bool {
        self.current_population < self.carrying_capacity / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_control_creation() {
        let control = PopulationControl::new("deer".to_string(), 100, 1000);
        assert_eq!(control.current_population, 100);
        assert_eq!(control.carrying_capacity, 1000);
    }

    #[test]
    fn test_population_growth() {
        let mut control = PopulationControl::new("rabbit".to_string(), 100, 500);
        control.birth_rate = 0.2;
        control.death_rate = 0.1;

        control.simulate_growth();
        assert!(control.current_population > 100);
    }

    #[test]
    fn test_carrying_capacity() {
        let mut control = PopulationControl::new("deer".to_string(), 150, 100);
        assert!(control.is_overpopulated());

        control.simulate_growth();
        assert_eq!(control.current_population, 100);
    }

    #[test]
    fn test_endangered_status() {
        let control = PopulationControl::new("tiger".to_string(), 5, 100);
        assert!(control.is_endangered());
    }
}
