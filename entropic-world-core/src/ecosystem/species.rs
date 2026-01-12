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

    pub fn add_preferred_biome(&mut self, biome: Biome) {
        if !self.preferred_biomes.contains(&biome) {
            self.preferred_biomes.push(biome);
        }
    }

    pub fn add_prey(&mut self, prey_id: SpeciesId) {
        if !self.hunting_prey.contains(&prey_id) {
            self.hunting_prey.push(prey_id);
        }
    }

    pub fn add_predator(&mut self, predator_id: SpeciesId) {
        if !self.hunted_by.contains(&predator_id) {
            self.hunted_by.push(predator_id);
        }
    }

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
