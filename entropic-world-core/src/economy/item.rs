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
    /// Constructs a new `Item` with the provided identifier, type, quantity, weight, and value.
    ///
    /// The returned `Item` has its public fields set to the corresponding arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// let item = Item::new("sword_1".to_string(), ItemType::Weapon, 1, 5.0, 100);
    /// assert_eq!(item.id, "sword_1");
    /// assert_eq!(item.item_type, ItemType::Weapon);
    /// assert_eq!(item.quantity, 1);
    /// assert_eq!(item.weight, 5.0);
    /// assert_eq!(item.value, 100);
    /// ```
    pub fn new(id: String, item_type: ItemType, quantity: u32, weight: f32, value: u32) -> Self {
        Self {
            id,
            item_type,
            quantity,
            weight,
            value,
        }
    }

    /// Computes the total weight for this item based on its quantity.
    ///
    /// # Returns
    ///
    /// The item's total weight (weight multiplied by quantity) as an `f32`.
    ///
    /// # Examples
    ///
    /// ```
    /// let item = Item { id: "potion_1".into(), item_type: ItemType::Potion, quantity: 3, weight: 0.5, value: 5 };
    /// assert_eq!(item.total_weight(), 1.5);
    /// ```
    pub fn total_weight(&self) -> f32 {
        self.weight * self.quantity as f32
    }

    /// Compute the total value of the item across its quantity.
    ///
    /// Returns the item's unit value multiplied by its quantity.
    ///
    /// # Examples
    ///
    /// ```
    /// use entropic_world_core::economy::{Item, ItemType};
    ///
    /// let item = Item::new("potion_1".into(), ItemType::Potion, 10, 0.5, 5);
    /// assert_eq!(item.total_value(), 50);
    /// ```
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