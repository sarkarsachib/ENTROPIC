use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub item_type: ItemType,
    pub quantity: u32,
    pub weight: f32,
    pub value: u32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemType {
    Weapon,
    Armor,
    Food,
    Potion,
    Currency,
    Custom(u32),
}

impl Item {
    pub fn new(id: String, item_type: ItemType, quantity: u32, weight: f32, value: u32) -> Self {
        Self {
            id,
            item_type,
            quantity,
            weight,
            value,
        }
    }

    pub fn total_weight(&self) -> f32 {
        self.weight * self.quantity as f32
    }

    pub fn total_value(&self) -> u32 {
        self.value * self.quantity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item::new("sword_1".to_string(), ItemType::Weapon, 1, 5.0, 100);
        assert_eq!(item.item_type, ItemType::Weapon);
        assert_eq!(item.quantity, 1);
    }

    #[test]
    fn test_item_totals() {
        let item = Item::new("potion_1".to_string(), ItemType::Potion, 10, 0.5, 5);
        assert_eq!(item.total_weight(), 5.0);
        assert_eq!(item.total_value(), 50);
    }
}
