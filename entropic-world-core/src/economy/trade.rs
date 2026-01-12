use serde::{Deserialize, Serialize};
use crate::economy::resource::ResourceType;
use crate::economy::settlement::SettlementId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeRoute {
    pub id: String,
    pub from: SettlementId,
    pub to: SettlementId,
    pub resource: ResourceType,
    pub frequency: u32,
    pub caravan_size: u32,
    pub active: bool,
}

impl TradeRoute {
    pub fn new(
        id: String,
        from: SettlementId,
        to: SettlementId,
        resource: ResourceType,
        frequency: u32,
        caravan_size: u32,
    ) -> Self {
        Self {
            id,
            from,
            to,
            resource,
            frequency,
            caravan_size,
            active: true,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeTransaction {
    pub id: String,
    pub seller: String,
    pub buyer: String,
    pub resource: ResourceType,
    pub quantity: u32,
    pub price_per_unit: u32,
    pub total_price: u32,
}

impl TradeTransaction {
    pub fn new(
        id: String,
        seller: String,
        buyer: String,
        resource: ResourceType,
        quantity: u32,
        price_per_unit: u32,
    ) -> Self {
        Self {
            id,
            seller,
            buyer,
            resource,
            quantity,
            price_per_unit,
            total_price: quantity * price_per_unit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_route_creation() {
        let route = TradeRoute::new(
            "route_1".to_string(),
            "settlement_1".to_string(),
            "settlement_2".to_string(),
            ResourceType::Food,
            4,
            50,
        );
        assert!(route.is_active());
        assert_eq!(route.frequency, 4);
    }

    #[test]
    fn test_trade_route_activation() {
        let mut route = TradeRoute::new(
            "route_1".to_string(),
            "settlement_1".to_string(),
            "settlement_2".to_string(),
            ResourceType::Wood,
            2,
            100,
        );

        route.deactivate();
        assert!(!route.is_active());

        route.activate();
        assert!(route.is_active());
    }

    #[test]
    fn test_trade_transaction() {
        let transaction = TradeTransaction::new(
            "tx_1".to_string(),
            "seller_1".to_string(),
            "buyer_1".to_string(),
            ResourceType::Metal,
            10,
            50,
        );
        assert_eq!(transaction.total_price, 500);
    }
}
