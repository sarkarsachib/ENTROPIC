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
    pub fn new(resource: ResourceType) -> Self {
        Self {
            producers: Vec::new(),
            consumers: Vec::new(),
            resource,
            production_rate: 0,
            consumption_rate: 0,
        }
    }

    pub fn add_producer(&mut self, settlement_id: SettlementId, rate: u32) {
        self.producers.push(settlement_id);
        self.production_rate += rate;
    }

    pub fn add_consumer(&mut self, settlement_id: SettlementId, rate: u32) {
        self.consumers.push(settlement_id);
        self.consumption_rate += rate;
    }

    pub fn is_balanced(&self) -> bool {
        self.production_rate >= self.consumption_rate
    }

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
