use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::ecosystem::species::SpeciesId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FoodChain {
    relationships: HashMap<SpeciesId, Vec<SpeciesId>>,
}

impl FoodChain {
    /// Creates a new, empty FoodChain.
    ///
    /// # Examples
    ///
    /// ```
    /// use entropic_world_core::ecosystem::food_chain::FoodChain;
    ///
    /// let chain = FoodChain::new();
    /// ```
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }

    /// Records a predator-prey relationship by adding `prey` to `predator`'s prey list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fc = FoodChain::new();
    /// let predator: SpeciesId = 1;
    /// let prey: SpeciesId = 2;
    /// fc.add_predator_prey(predator, prey);
    /// assert_eq!(fc.get_prey(&predator).unwrap().len(), 1);
    /// ```
    pub fn add_predator_prey(&mut self, predator: SpeciesId, prey: SpeciesId) {
        self.relationships
            .entry(predator)
            .or_insert_with(Vec::new)
            .push(prey);
    }

    /// Retrieve the list of prey for a given predator.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fc = FoodChain::new();
    /// let predator = SpeciesId::default();
    /// let prey = SpeciesId::default();
    /// fc.add_predator_prey(predator.clone(), prey.clone());
    /// assert_eq!(fc.get_prey(&predator).unwrap().as_slice(), &[prey]);
    /// ```
    pub fn get_prey(&self, predator: &SpeciesId) -> Option<&Vec<SpeciesId>> {
        self.relationships.get(predator)
    }

    /// Returns the list of species that prey on the given `prey`.
    ///
    /// The returned `Vec<SpeciesId>` contains each predator that has `prey` in its prey list. Order is unspecified.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut fc = FoodChain::new();
    /// let predator_a = SpeciesId::from(1);
    /// let predator_b = SpeciesId::from(2);
    /// let prey = SpeciesId::from(3);
    ///
    /// fc.add_predator_prey(predator_a.clone(), prey.clone());
    /// fc.add_predator_prey(predator_b.clone(), prey.clone());
    ///
    /// let predators = fc.get_predators(&prey);
    /// assert!(predators.contains(&predator_a));
    /// assert!(predators.contains(&predator_b));
    /// ```
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

    /// Determines whether any species in the food chain preys on the given species.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fc = FoodChain::new();
    /// let predator: SpeciesId = 1.into();
    /// let prey: SpeciesId = 2.into();
    /// fc.add_predator_prey(predator, prey.clone());
    /// assert!(fc.has_predator(&prey));
    /// ```
    ///
    /// # Returns
    ///
    /// `true` if at least one predator lists `species` as prey, `false` otherwise.
    pub fn has_predator(&self, species: &SpeciesId) -> bool {
        self.relationships
            .values()
            .any(|prey_list| prey_list.contains(species))
    }

    /// Checks whether a species is listed as a predator in the food chain.
    ///
    /// # Returns
    ///
    /// `true` if the species has any prey entries in the relationships map, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fc = FoodChain::new();
    /// let predator = SpeciesId::from(1);
    /// let prey = SpeciesId::from(2);
    /// fc.add_predator_prey(predator.clone(), prey);
    /// assert!(fc.is_predator(&predator));
    /// assert!(!fc.is_predator(&SpeciesId::from(3)));
    /// ```
    pub fn is_predator(&self, species: &SpeciesId) -> bool {
        self.relationships.contains_key(species)
    }
}

impl Default for FoodChain {
    /// Creates an empty FoodChain.
    ///
    /// # Returns
    ///
    /// A `FoodChain` with no predator-prey relationships.
    ///
    /// # Examples
    ///
    /// ```
    /// let fc = entropic_world_core::ecosystem::food_chain::FoodChain::default();
    /// assert!(fc.get_prey(&0).is_none()); // assuming 0 is not a SpeciesId present
    /// ```
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