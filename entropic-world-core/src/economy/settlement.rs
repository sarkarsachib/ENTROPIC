use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::economy::resource::ResourceType;
use crate::spatial::terrain::StructureId;

pub type SettlementId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settlement {
    pub id: SettlementId,
    pub name: String,
    pub faction: String,
    pub x: f32,
    pub y: f32,
    pub population: u32,
    pub wealth: u64,
    pub buildings: Vec<StructureId>,
    pub markets: Vec<String>,
    pub allegiances: HashMap<String, f32>,
    pub resources: HashMap<ResourceType, u32>,
    pub happiness: f32,
}

impl Settlement {
    /// Creates a Settlement with the given id, name, faction, and position.
    ///
    /// The settlement is initialized with population and wealth set to 0, empty
    /// collections for buildings, markets, allegiances, and resources, and
    /// happiness set to 0.5.
    ///
    /// # Returns
    ///
    /// A `Settlement` initialized with the provided fields and the defaults described above.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = Settlement::new(
    ///     "settlement-1".to_string(),
    ///     "Riverside".to_string(),
    ///     "Neutral".to_string(),
    ///     10.0,
    ///     20.0,
    /// );
    /// assert_eq!(s.name, "Riverside");
    /// assert_eq!(s.population, 0);
    /// assert_eq!(s.wealth, 0);
    /// assert_eq!(s.happiness, 0.5);
    /// ```
    pub fn new(id: SettlementId, name: String, faction: String, x: f32, y: f32) -> Self {
        Self {
            id,
            name,
            faction,
            x,
            y,
            population: 0,
            wealth: 0,
            buildings: Vec::new(),
            markets: Vec::new(),
            allegiances: HashMap::new(),
            resources: HashMap::new(),
            happiness: 0.5,
        }
    }

    /// Adds a structure identifier to this settlement's list of buildings.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = crate::economy::settlement::Settlement::new(
    ///     "settlement-1".into(),
    ///     "New Town".into(),
    ///     "Neutral".into(),
    ///     0.0,
    ///     0.0,
    /// );
    /// s.add_building("structure-1".into());
    /// ```
    pub fn add_building(&mut self, building_id: StructureId) {
        self.buildings.push(building_id);
    }

    /// Increases the settlement's population by the given amount.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Settlement::new("s1".to_string(), "Hamlet".to_string(), "Neutral".to_string(), 0.0, 0.0);
    /// s.add_population(10);
    /// assert_eq!(s.population, 10);
    /// ```
    pub fn add_population(&mut self, amount: u32) {
        self.population += amount;
    }

    /// Decreases the settlement's population by the given amount, clamping the value at zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Settlement::new("id".into(), "Town".into(), "Faction".into(), 0.0, 0.0);
    /// s.add_population(10);
    /// s.remove_population(4);
    /// assert_eq!(s.population, 6);
    /// s.remove_population(10);
    /// assert_eq!(s.population, 0);
    /// ```
    pub fn remove_population(&mut self, amount: u32) {
        self.population = self.population.saturating_sub(amount);
    }

    /// Increases the settlement's wealth by the specified amount.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Settlement::new("s1".to_string(), "Town".to_string(), "Neutral".to_string(), 0.0, 0.0);
    /// s.add_wealth(100);
    /// assert_eq!(s.wealth, 100);
    /// ```
    pub fn add_wealth(&mut self, amount: u64) {
        self.wealth += amount;
    }

    /// Attempts to deduct the given amount from the settlement's wealth.
    ///
    /// If the settlement has at least `amount` wealth, the amount is subtracted from `self.wealth`.
    ///
    /// # Returns
    ///
    /// `true` if the amount was deducted, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Settlement::new("id".to_string(), "Town".to_string(), "Faction".to_string(), 0.0, 0.0);
    /// s.add_wealth(100);
    /// let ok = s.spend_wealth(40);
    /// assert!(ok);
    /// assert_eq!(s.wealth, 60);
    /// let fail = s.spend_wealth(100);
    /// assert!(!fail);
    /// assert_eq!(s.wealth, 60);
    /// ```
    pub fn spend_wealth(&mut self, amount: u64) -> bool {
        if self.wealth >= amount {
            self.wealth -= amount;
            true
        } else {
            false
        }
    }

    /// Increments the stored quantity of a resource for this settlement.
    ///
    /// Adds `amount` to the entry for `resource`, inserting zero if the resource was not present.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Settlement::new("id".to_string(), "Town".to_string(), "Faction".to_string(), 0.0, 0.0);
    /// s.add_resource(ResourceType::Wood, 10);
    /// assert_eq!(s.get_resource(&ResourceType::Wood), 10);
    /// s.add_resource(ResourceType::Wood, 5);
    /// assert_eq!(s.get_resource(&ResourceType::Wood), 15);
    /// ```
    pub fn add_resource(&mut self, resource: ResourceType, amount: u32) {
        *self.resources.entry(resource).or_insert(0) += amount;
    }

    /// Attempts to consume `amount` of `resource` from the settlement's stored resources.
    ///
    /// Consumes and decrements the stored amount only if at least `amount` is available; otherwise leaves storage unchanged.
    ///
    /// # Parameters
    ///
    /// - `resource`: the resource type to consume.
    /// - `amount`: the quantity to remove from storage.
    ///
    /// # Returns
    ///
    /// `true` if `amount` was successfully consumed, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// // Construct a settlement and add resources, then consume them.
    /// let mut s = Settlement::new("s1".into(), "Town".into(), "Faction".into(), 0.0, 0.0);
    /// s.add_resource(ResourceType::Food, 10);
    /// assert!(s.consume_resource(ResourceType::Food, 5));
    /// assert_eq!(s.get_resource(&ResourceType::Food), 5);
    /// assert!(!s.consume_resource(ResourceType::Food, 6));
    /// ```
    pub fn consume_resource(&mut self, resource: ResourceType, amount: u32) -> bool {
        if let Some(available) = self.resources.get_mut(&resource) {
            if *available >= amount {
                *available -= amount;
                return true;
            }
        }
        false
    }

    /// Fetches the stored amount for a given resource.
    ///
    /// # Parameters
    ///
    /// - `resource`: The resource type to query.
    ///
    /// # Returns
    ///
    /// `u32` amount of the specified resource, or `0` if none is stored.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Settlement::new("s1".into(), "Town".into(), "Faction".into(), 0.0, 0.0);
    /// s.add_resource(ResourceType::Food, 10);
    /// assert_eq!(s.get_resource(&ResourceType::Food), 10);
    /// assert_eq!(s.get_resource(&ResourceType::Wood), 0);
    /// ```
    pub fn get_resource(&self, resource: &ResourceType) -> u32 {
        *self.resources.get(resource).unwrap_or(&0)
    }

    /// Adjusts the settlement's happiness by the given delta, clamping the result to the range [0.0, 1.0].
    ///
    /// The `delta` is added to the current happiness; positive values increase happiness and negative values decrease it.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut s = Settlement::new("id".into(), "Town".into(), "Faction".into(), 0.0, 0.0);
    /// s.adjust_happiness(0.3);
    /// assert!((s.happiness - 0.8).abs() < f32::EPSILON); // initial 0.5 + 0.3 = 0.8
    /// s.adjust_happiness(-1.0);
    /// assert_eq!(s.happiness, 0.0); // clamped to lower bound
    /// ```
    pub fn adjust_happiness(&mut self, delta: f32) {
        self.happiness = (self.happiness + delta).clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settlement_creation() {
        let settlement = Settlement::new(
            "settlement_1".to_string(),
            "Rivertown".to_string(),
            "faction_1".to_string(),
            100.0,
            200.0,
        );
        assert_eq!(settlement.name, "Rivertown");
        assert_eq!(settlement.population, 0);
    }

    #[test]
    fn test_settlement_population() {
        let mut settlement = Settlement::new(
            "settlement_1".to_string(),
            "Rivertown".to_string(),
            "faction_1".to_string(),
            100.0,
            200.0,
        );

        settlement.add_population(100);
        assert_eq!(settlement.population, 100);

        settlement.remove_population(30);
        assert_eq!(settlement.population, 70);
    }

    #[test]
    fn test_settlement_wealth() {
        let mut settlement = Settlement::new(
            "settlement_1".to_string(),
            "Rivertown".to_string(),
            "faction_1".to_string(),
            100.0,
            200.0,
        );

        settlement.add_wealth(1000);
        assert!(settlement.spend_wealth(500));
        assert_eq!(settlement.wealth, 500);

        assert!(!settlement.spend_wealth(1000));
    }

    #[test]
    fn test_settlement_resources() {
        let mut settlement = Settlement::new(
            "settlement_1".to_string(),
            "Rivertown".to_string(),
            "faction_1".to_string(),
            100.0,
            200.0,
        );

        settlement.add_resource(ResourceType::Food, 100);
        assert_eq!(settlement.get_resource(&ResourceType::Food), 100);

        assert!(settlement.consume_resource(ResourceType::Food, 30));
        assert_eq!(settlement.get_resource(&ResourceType::Food), 70);

        assert!(!settlement.consume_resource(ResourceType::Food, 100));
    }
}