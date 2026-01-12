use serde::{Deserialize, Serialize};
use crate::economy::resource::ResourceType;
use crate::economy::settlement::SettlementId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupplyChain {
    pub producers: Vec<SettlementId>,
    pub consumers: Vec<SettlementId>,
    pub resource: ResourceType,
    pub production_rate: u32,
    pub consumption_rate: u32,
}

impl SupplyChain {
    /// Creates a new `SupplyChain` for the specified resource with no producers or consumers and zeroed rates.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::economy::supply_chain::SupplyChain;
    /// use crate::economy::resource::ResourceType;
    ///
    /// let sc = SupplyChain::new(ResourceType::Food);
    /// assert_eq!(sc.resource, ResourceType::Food);
    /// assert_eq!(sc.production_rate, 0);
    /// assert_eq!(sc.consumption_rate, 0);
    /// ```
    pub fn new(resource: ResourceType) -> Self {
        Self {
            producers: Vec::new(),
            consumers: Vec::new(),
            resource,
            production_rate: 0,
            consumption_rate: 0,
        }
    }

    /// Registers a settlement as a producer and increases the supply chain's total production rate.
    ///
    /// `settlement_id` is the identifier of the settlement to add as a producer.
    /// `rate` is the production rate contributed by that settlement.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::economy::supply_chain::SupplyChain;
    /// use crate::economy::resource::ResourceType;
    ///
    /// let mut sc = SupplyChain::new(ResourceType::Food);
    /// sc.add_producer("settlement_a".into(), 50);
    /// assert_eq!(sc.production_rate, 50);
    /// assert!(sc.producers.contains(&"settlement_a".into()));
    /// ```
    pub fn add_producer(&mut self, settlement_id: SettlementId, rate: u32) {
        self.producers.push(settlement_id);
        self.production_rate += rate;
    }

    /// Registers a consumer settlement and increments the supply chain's total consumption rate.
    ///
    /// Adds `settlement_id` to the `consumers` list and increases `consumption_rate` by `rate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::economy::{supply_chain::SupplyChain, resource::ResourceType, settlement::SettlementId};
    ///
    /// let mut sc = SupplyChain::new(ResourceType::Food);
    /// let id: SettlementId = "town_a".into();
    /// sc.add_consumer(id.clone(), 42);
    /// assert!(sc.consumers.contains(&id));
    /// assert_eq!(sc.consumption_rate, 42);
    /// ```
    pub fn add_consumer(&mut self, settlement_id: SettlementId, rate: u32) {
        self.consumers.push(settlement_id);
        self.consumption_rate += rate;
    }

    /// Checks whether production meets or exceeds consumption for this supply chain.
    ///
    /// # Returns
    ///
    /// `true` if `production_rate` is greater than or equal to `consumption_rate`, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sc = SupplyChain::new(ResourceType::Wood);
    /// sc.add_producer("A".into(), 100);
    /// sc.add_consumer("B".into(), 80);
    /// assert!(sc.is_balanced());
    /// ```
    pub fn is_balanced(&self) -> bool {
        self.production_rate >= self.consumption_rate
    }

    /// Computes the signed surplus of the supply chain.
    ///
    /// The surplus is `production_rate` minus `consumption_rate`, returned as an `i64`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sc = SupplyChain::new(ResourceType::Wood);
    /// sc.add_producer("A".into(), 100);
    /// sc.add_consumer("B".into(), 80);
    /// assert_eq!(sc.surplus(), 20);
    /// ```
    pub fn surplus(&self) -> i64 {
        self.production_rate as i64 - self.consumption_rate as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply_chain_creation() {
        let chain = SupplyChain::new(ResourceType::Food);
        assert_eq!(chain.resource, ResourceType::Food);
        assert_eq!(chain.production_rate, 0);
    }

    #[test]
    fn test_supply_chain_balance() {
        let mut chain = SupplyChain::new(ResourceType::Wood);
        chain.add_producer("settlement_1".to_string(), 100);
        chain.add_consumer("settlement_2".to_string(), 80);

        assert!(chain.is_balanced());
        assert_eq!(chain.surplus(), 20);
    }

    #[test]
    fn test_supply_chain_deficit() {
        let mut chain = SupplyChain::new(ResourceType::Metal);
        chain.add_producer("settlement_1".to_string(), 50);
        chain.add_consumer("settlement_2".to_string(), 100);

        assert!(!chain.is_balanced());
        assert_eq!(chain.surplus(), -50);
    }
}