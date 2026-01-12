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
    /// Constructs a PopulationControl for a species with the specified initial population and carrying capacity.
    ///
    /// The instance is initialized with a default birth rate of 0.1 and a default death rate of 0.05.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::ecosystem::population_control::PopulationControl;
    /// use crate::ecosystem::species::SpeciesId;
    ///
    /// let species = SpeciesId::from("deer");
    /// let pc = PopulationControl::new(species, 100, 1000);
    /// assert_eq!(pc.current_population, 100);
    /// assert_eq!(pc.carrying_capacity, 1000);
    /// ```
    pub fn new(species_id: SpeciesId, initial_population: u32, carrying_capacity: u32) -> Self {
        Self {
            species_id,
            current_population: initial_population,
            birth_rate: 0.1,
            death_rate: 0.05,
            carrying_capacity,
        }
    }

    /// Advances the population by one timestep using the current birth and death rates, enforcing carrying capacity and adjusting mortality when capacity is exceeded.
    ///
    /// The method updates `current_population` by applying births and deaths, caps the population at `carrying_capacity` (increasing `death_rate` proportionally to any excess), and clamps `death_rate` to the range 0.0..=1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::ecosystem::population_control::PopulationControl;
    ///
    /// let mut pc = PopulationControl::new("rabbit".into(), 100, 500);
    /// pc.birth_rate = 0.2;
    /// pc.death_rate = 0.1;
    /// pc.simulate_growth();
    /// assert!(pc.current_population > 100);
    /// ```
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

    /// Increases the population by a given number of individuals without exceeding carrying capacity.
    ///
    /// The population is incremented by `count`; if the resulting population would exceed
    /// `carrying_capacity`, it is clamped to `carrying_capacity`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut pc = PopulationControl::new("rabbit".into(), 50, 100);
    /// pc.add_individuals(30);
    /// assert_eq!(pc.current_population, 80);
    /// pc.add_individuals(50);
    /// assert_eq!(pc.current_population, 100); // clamped to carrying_capacity
    /// ```
    pub fn add_individuals(&mut self, count: u32) {
        self.current_population = (self.current_population + count).min(self.carrying_capacity);
    }

    /// Decreases the current population by the given number of individuals.
    ///
    /// The population is reduced by `count` but will not go below zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut pc = PopulationControl {
    ///     species_id: "rabbit".into(),
    ///     current_population: 10,
    ///     birth_rate: 0.1,
    ///     death_rate: 0.05,
    ///     carrying_capacity: 100,
    /// };
    /// pc.remove_individuals(4);
    /// assert_eq!(pc.current_population, 6);
    /// pc.remove_individuals(10);
    /// assert_eq!(pc.current_population, 0);
    /// ```
    pub fn remove_individuals(&mut self, count: u32) {
        self.current_population = self.current_population.saturating_sub(count);
    }

    /// Report whether the population exceeds the configured carrying capacity.
    ///
    /// # Returns
    ///
    /// `true` if `current_population` is greater than `carrying_capacity`, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let pc = PopulationControl::new("deer".into(), 150, 100);
    /// assert!(pc.is_overpopulated());
    /// ```
    pub fn is_overpopulated(&self) -> bool {
        self.current_population > self.carrying_capacity
    }

    /// Determines whether the species is endangered based on population relative to carrying capacity.
    ///
    /// # Returns
    ///
    /// `true` if the current population is less than one tenth of the carrying capacity, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let pc = PopulationControl::new("tiger".into(), 5, 100);
    /// assert!(pc.is_endangered());
    /// ```
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