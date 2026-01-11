use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::ecosystem::species::SpeciesId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FoodChain {
    relationships: HashMap<SpeciesId, Vec<SpeciesId>>,
}

impl FoodChain {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }

    pub fn add_predator_prey(&mut self, predator: SpeciesId, prey: SpeciesId) {
        self.relationships
            .entry(predator)
            .or_insert_with(Vec::new)
            .push(prey);
    }

    pub fn get_prey(&self, predator: &SpeciesId) -> Option<&Vec<SpeciesId>> {
        self.relationships.get(predator)
    }

    pub fn get_predators(&self, prey: &SpeciesId) -> Vec<SpeciesId> {
        self.relationships
            .iter()
            .filter_map(|(predator, prey_list)| {
                if prey_list.contains(prey) {
                    Some(predator.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn has_predator(&self, species: &SpeciesId) -> bool {
        self.relationships
            .values()
            .any(|prey_list| prey_list.contains(species))
    }

    pub fn is_predator(&self, species: &SpeciesId) -> bool {
        self.relationships.contains_key(species)
    }
}

impl Default for FoodChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_food_chain_creation() {
        let chain = FoodChain::new();
        assert!(chain.relationships.is_empty());
    }

    #[test]
    fn test_add_predator_prey() {
        let mut chain = FoodChain::new();
        chain.add_predator_prey("wolf".to_string(), "deer".to_string());
        chain.add_predator_prey("wolf".to_string(), "rabbit".to_string());

        let prey = chain.get_prey(&"wolf".to_string());
        assert!(prey.is_some());
        assert_eq!(prey.unwrap().len(), 2);
    }

    #[test]
    fn test_get_predators() {
        let mut chain = FoodChain::new();
        chain.add_predator_prey("wolf".to_string(), "deer".to_string());
        chain.add_predator_prey("bear".to_string(), "deer".to_string());

        let predators = chain.get_predators(&"deer".to_string());
        assert_eq!(predators.len(), 2);
        assert!(predators.contains(&"wolf".to_string()));
        assert!(predators.contains(&"bear".to_string()));
    }

    #[test]
    fn test_predator_checks() {
        let mut chain = FoodChain::new();
        chain.add_predator_prey("wolf".to_string(), "deer".to_string());

        assert!(chain.is_predator(&"wolf".to_string()));
        assert!(chain.has_predator(&"deer".to_string()));
        assert!(!chain.is_predator(&"deer".to_string()));
    }
}
