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
    /// Creates a new `Market` with the given identifier and settlement identifier, initializing empty price, supply, and demand maps.
    ///
    /// The returned market has no registered resources; call `add_resource` to register resources and set initial prices, supply, and demand.
    ///
    /// # Examples
    ///
    /// ```
    /// let market = Market::new("market-1".into(), "settlement-1".to_string());
    /// assert_eq!(market.settlement_id, "settlement-1");
    /// assert!(market.prices.is_empty());
    /// assert!(market.supply.is_empty());
    /// assert!(market.demand.is_empty());
    /// ```
    pub fn new(id: MarketId, settlement_id: String) -> Self {
        Self {
            id,
            settlement_id,
            prices: HashMap::new(),
            supply: HashMap::new(),
            demand: HashMap::new(),
        }
    }

    /// Registers a resource in the market and initializes its price, supply, and demand.
    ///
    /// The resource's base value is used as both its `base_price` and initial `current_price`.
    /// Volatility is set to `DEFAULT_PRICE_VOLATILITY` and `last_updated` is set to `WorldTime::default()`.
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource type to add to the market.
    /// * `supply` - Initial available quantity for the resource.
    /// * `demand` - Initial demanded quantity for the resource.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut market = Market::new("m1".into(), "settlement".into());
    /// market.add_resource(ResourceType::Food, 100, 50);
    /// assert_eq!(market.supply.get(&ResourceType::Food), Some(&100));
    /// assert_eq!(market.demand.get(&ResourceType::Food), Some(&50));
    /// ```
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

    /// Update the stored current price for a resource using the ratio of demand to supply.
    ///
    /// If the resource exists and both recorded supply and demand are greater than zero,
    /// the function sets `current_price = base_price * (demand / supply)` (floating-point ratio
    /// truncated to `u32`) and updates `last_updated` to `time`. If the resource is absent or
    /// either supply or demand is zero, no changes are made.
    ///
    /// # Parameters
    ///
    /// - `resource`: The resource whose price should be updated.
    /// - `time`: The timestamp to record as `last_updated` when the price is changed.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut market = Market::new("m1".to_string(), "settlement".to_string());
    /// market.add_resource(ResourceType::Food, 100, 200);
    /// market.update_price(ResourceType::Food, WorldTime::default());
    /// let price = market.get_price(&ResourceType::Food).unwrap();
    /// assert!(price > ResourceType::Food.base_value());
    /// ```
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

    /// Retrieve the current market price for a given resource.
    ///
    /// # Returns
    ///
    /// `Some(current_price)` if the resource has a recorded price, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// // Create a market, register a resource, then read its price.
    /// let mut market = Market::new("market-1".into(), "settlement-1".into());
    /// market.add_resource(ResourceType::Food, 100, 50);
    /// let price = market.get_price(&ResourceType::Food);
    /// assert!(price.is_some());
    /// ```
    pub fn get_price(&self, resource: &ResourceType) -> Option<u32> {
        self.prices.get(resource).map(|p| p.current_price)
    }

    /// Increases the stored supply for a given resource by the specified amount.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut market = Market::new("m1".to_string(), "settlement".to_string());
    /// market.add_resource(ResourceType::Food, 0, 0);
    /// market.add_supply(ResourceType::Food, 10);
    /// assert!(market.consume_supply(ResourceType::Food, 5));
    /// assert_eq!(market.supply.get(&ResourceType::Food).copied().unwrap_or(0), 5);
    /// ```
    pub fn add_supply(&mut self, resource: ResourceType, amount: u32) {
        *self.supply.entry(resource).or_insert(0) += amount;
    }

    /// Increases the recorded demand for `resource` by `amount`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut market = Market::new("m1".to_string(), "settlement".to_string());
    /// market.add_resource(ResourceType::Food, 0, 0);
    /// market.add_demand(ResourceType::Food, 5);
    /// assert_eq!(market.demand.get(&ResourceType::Food).copied().unwrap_or(0), 5);
    /// ```
    pub fn add_demand(&mut self, resource: ResourceType, amount: u32) {
        *self.demand.entry(resource).or_insert(0) += amount;
    }

    /// Attempts to decrease the available supply of a resource by a given amount.
    ///
    /// Returns `true` if the supply for `resource` was at least `amount` and was decreased by `amount`, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut market = Market::new("mkt-1".into(), "settlement-1".into());
    /// market.add_resource(ResourceType::Food, 10, 0);
    /// assert!(market.consume_supply(ResourceType::Food, 5));
    /// assert_eq!(market.supply.get(&ResourceType::Food), Some(&5));
    /// ```
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