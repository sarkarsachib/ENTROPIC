use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::economy::resource::ResourceType;
use crate::temporal::time::WorldTime;
use crate::constants::DEFAULT_PRICE_VOLATILITY;

pub type MarketId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Market {
    pub id: MarketId,
    pub settlement_id: String,
    pub prices: HashMap<ResourceType, MarketPrice>,
    pub supply: HashMap<ResourceType, u32>,
    pub demand: HashMap<ResourceType, u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarketPrice {
    pub base_price: u32,
    pub current_price: u32,
    pub volatility: f32,
    pub last_updated: WorldTime,
}

impl Market {
    pub fn new(id: MarketId, settlement_id: String) -> Self {
        Self {
            id,
            settlement_id,
            prices: HashMap::new(),
            supply: HashMap::new(),
            demand: HashMap::new(),
        }
    }

    pub fn add_resource(&mut self, resource: ResourceType, supply: u32, demand: u32) {
        let base_price = resource.base_value();
        self.prices.insert(
            resource,
            MarketPrice {
                base_price,
                current_price: base_price,
                volatility: DEFAULT_PRICE_VOLATILITY,
                last_updated: WorldTime::default(),
            },
        );
        self.supply.insert(resource, supply);
        self.demand.insert(resource, demand);
    }

    pub fn update_price(&mut self, resource: ResourceType, time: WorldTime) {
        if let Some(price) = self.prices.get_mut(&resource) {
            let supply = *self.supply.get(&resource).unwrap_or(&0);
            let demand = *self.demand.get(&resource).unwrap_or(&0);

            if supply > 0 && demand > 0 {
                let ratio = demand as f32 / supply as f32;
                let new_price = (price.base_price as f32 * ratio) as u32;
                price.current_price = new_price;
                price.last_updated = time;
            }
        }
    }

    pub fn get_price(&self, resource: &ResourceType) -> Option<u32> {
        self.prices.get(resource).map(|p| p.current_price)
    }

    pub fn add_supply(&mut self, resource: ResourceType, amount: u32) {
        *self.supply.entry(resource).or_insert(0) += amount;
    }

    pub fn add_demand(&mut self, resource: ResourceType, amount: u32) {
        *self.demand.entry(resource).or_insert(0) += amount;
    }

    pub fn consume_supply(&mut self, resource: ResourceType, amount: u32) -> bool {
        if let Some(supply) = self.supply.get_mut(&resource) {
            if *supply >= amount {
                *supply -= amount;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_creation() {
        let market = Market::new("market_1".to_string(), "settlement_1".to_string());
        assert_eq!(market.settlement_id, "settlement_1");
    }

    #[test]
    fn test_market_add_resource() {
        let mut market = Market::new("market_1".to_string(), "settlement_1".to_string());
        market.add_resource(ResourceType::Food, 100, 50);

        assert_eq!(market.supply.get(&ResourceType::Food), Some(&100));
        assert_eq!(market.demand.get(&ResourceType::Food), Some(&50));
    }

    #[test]
    fn test_market_price_update() {
        let mut market = Market::new("market_1".to_string(), "settlement_1".to_string());
        market.add_resource(ResourceType::Food, 100, 200);
        market.update_price(ResourceType::Food, WorldTime::default());

        let price = market.get_price(&ResourceType::Food);
        assert!(price.is_some());
        assert!(price.unwrap() > ResourceType::Food.base_value());
    }

    #[test]
    fn test_market_consume_supply() {
        let mut market = Market::new("market_1".to_string(), "settlement_1".to_string());
        market.add_resource(ResourceType::Wood, 50, 10);

        assert!(market.consume_supply(ResourceType::Wood, 20));
        assert_eq!(market.supply.get(&ResourceType::Wood), Some(&30));

        assert!(!market.consume_supply(ResourceType::Wood, 50));
    }
}
