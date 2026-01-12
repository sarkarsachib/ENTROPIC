use serde::{Deserialize, Serialize};
use crate::spatial::terrain::Biome;

pub type SpeciesId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Species {
    pub id: SpeciesId,
    pub name: String,
    pub diet: Diet,
    pub base_population: u32,
    pub reproduction_rate: f32,
    pub lifespan_years: u16,
    pub preferred_biomes: Vec<Biome>,
    pub hunting_prey: Vec<SpeciesId>,
    pub hunted_by: Vec<SpeciesId>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Diet {
    Herbivore,
    Carnivore,
    Omnivore,
}

impl Species {
    /// Creates a `Species` with the given `id`, `name`, and `diet`, using sensible defaults for other fields.
    ///
    /// The returned `Species` has `base_population` set to 100, `reproduction_rate` set to 0.1,
    /// `lifespan_years` set to 10, and empty vectors for `preferred_biomes`, `hunting_prey`, and `hunted_by`.
    ///
    /// # Examples
    ///
    /// ```
    /// let sp = Species::new("deer".to_string(), "White-tailed Deer".to_string(), Diet::Herbivore);
    /// assert_eq!(sp.id, "deer");
    /// assert_eq!(sp.name, "White-tailed Deer");
    /// assert_eq!(sp.diet, Diet::Herbivore);
    /// assert_eq!(sp.base_population, 100);
    /// assert_eq!(sp.reproduction_rate, 0.1);
    /// assert_eq!(sp.lifespan_years, 10);
    /// assert!(sp.preferred_biomes.is_empty());
    /// assert!(sp.hunting_prey.is_empty());
    /// assert!(sp.hunted_by.is_empty());
    /// ```
    pub fn new(id: SpeciesId, name: String, diet: Diet) -> Self {
        Self {
            id,
            name,
            diet,
            base_population: 100,
            reproduction_rate: 0.1,
            lifespan_years: 10,
            preferred_biomes: Vec::new(),
            hunting_prey: Vec::new(),
            hunted_by: Vec::new(),
        }
    }

    /// Adds `biome` to the species' preferred biomes if it is not already present.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Species::new("bear".into(), "Brown Bear".into(), Diet::Omnivore);
    /// s.add_preferred_biome(Biome::Forest);
    /// assert!(s.prefers_biome(&Biome::Forest));
    /// ```
    pub fn add_preferred_biome(&mut self, biome: Biome) {
        if !self.preferred_biomes.contains(&biome) {
            self.preferred_biomes.push(biome);
        }
    }

    /// Adds a prey species id to this species' `hunting_prey` list.
    ///
    /// If `prey_id` is already present, the list is left unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Species::new("wolf".into(), "Gray Wolf".into(), Diet::Carnivore);
    /// let deer = "deer".to_string();
    /// s.add_prey(deer.clone());
    /// assert!(s.hunting_prey.contains(&deer));
    /// ```
    pub fn add_prey(&mut self, prey_id: SpeciesId) {
        if !self.hunting_prey.contains(&prey_id) {
            self.hunting_prey.push(prey_id);
        }
    }

    /// Adds a predator's species id to this species' `hunted_by` list if it is not already present.
    ///
    /// Ensures the predator id appears at most once in the `hunted_by` vector.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut rabbit = Species::new("rabbit".to_string(), "Rabbit".to_string(), Diet::Herbivore);
    /// rabbit.add_predator("fox".to_string());
    /// assert!(rabbit.hunted_by.contains(&"fox".to_string()));
    /// rabbit.add_predator("fox".to_string());
    /// assert_eq!(rabbit.hunted_by.iter().filter(|id| *id == "fox").count(), 1);
    /// ```
    pub fn add_predator(&mut self, predator_id: SpeciesId) {
        if !self.hunted_by.contains(&predator_id) {
            self.hunted_by.push(predator_id);
        }
    }

    /// Checks whether the species prefers a given biome.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Species::new("deer".into(), "Deer".into(), Diet::Herbivore);
    /// s.add_preferred_biome(Biome::Forest);
    /// assert!(s.prefers_biome(&Biome::Forest));
    /// assert!(!s.prefers_biome(&Biome::Desert));
    /// ```
    ///
    /// # Returns
    ///
    /// `true` if the species prefers the given `biome`, `false` otherwise.
    pub fn prefers_biome(&self, biome: &Biome) -> bool {
        self.preferred_biomes.contains(biome)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_species_creation() {
        let species = Species::new(
            "deer".to_string(),
            "White-tailed Deer".to_string(),
            Diet::Herbivore,
        );
        assert_eq!(species.name, "White-tailed Deer");
        assert_eq!(species.diet, Diet::Herbivore);
    }

    #[test]
    fn test_species_biome_preference() {
        let mut species = Species::new(
            "bear".to_string(),
            "Brown Bear".to_string(),
            Diet::Omnivore,
        );
        species.add_preferred_biome(Biome::Forest);

        assert!(species.prefers_biome(&Biome::Forest));
        assert!(!species.prefers_biome(&Biome::Desert));
    }

    #[test]
    fn test_species_food_chain() {
        let mut predator = Species::new(
            "wolf".to_string(),
            "Gray Wolf".to_string(),
            Diet::Carnivore,
        );
        predator.add_prey("deer".to_string());

        assert_eq!(predator.hunting_prey.len(), 1);
        assert!(predator.hunting_prey.contains(&"deer".to_string()));
    }
}