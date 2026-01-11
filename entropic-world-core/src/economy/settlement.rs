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

    pub fn add_building(&mut self, building_id: StructureId) {
        self.buildings.push(building_id);
    }

    pub fn add_population(&mut self, amount: u32) {
        self.population += amount;
    }

    pub fn remove_population(&mut self, amount: u32) {
        self.population = self.population.saturating_sub(amount);
    }

    pub fn add_wealth(&mut self, amount: u64) {
        self.wealth += amount;
    }

    pub fn spend_wealth(&mut self, amount: u64) -> bool {
        if self.wealth >= amount {
            self.wealth -= amount;
            true
        } else {
            false
        }
    }

    pub fn add_resource(&mut self, resource: ResourceType, amount: u32) {
        *self.resources.entry(resource).or_insert(0) += amount;
    }

    pub fn consume_resource(&mut self, resource: ResourceType, amount: u32) -> bool {
        if let Some(available) = self.resources.get_mut(&resource) {
            if *available >= amount {
                *available -= amount;
                return true;
            }
        }
        false
    }

    pub fn get_resource(&self, resource: &ResourceType) -> u32 {
        *self.resources.get(resource).unwrap_or(&0)
    }

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
