use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Food,
    Wood,
    Metal,
    Stone,
    Cloth,
    Herbs,
    Gold,
    Custom(u32),
}

impl ResourceType {
    /// Get the human-readable name for the resource type.
    ///
    /// # Returns
    ///
    /// The static string name of the resource type (e.g., `"Food"`, `"Metal"`, or `"Custom"`).
    ///
    /// # Examples
    ///
    /// ```
    /// let r = ResourceType::Food;
    /// assert_eq!(r.name(), "Food");
    /// let r = ResourceType::Custom(42);
    /// assert_eq!(r.name(), "Custom");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            ResourceType::Food => "Food",
            ResourceType::Wood => "Wood",
            ResourceType::Metal => "Metal",
            ResourceType::Stone => "Stone",
            ResourceType::Cloth => "Cloth",
            ResourceType::Herbs => "Herbs",
            ResourceType::Gold => "Gold",
            ResourceType::Custom(_) => "Custom",
        }
    }

    /// Provides the base numeric value associated with the resource type.
    ///
    /// The returned value represents a default worth used for economic calculations.
    ///
    /// # Returns
    ///
    /// `u32` base value for the resource: Food=2, Wood=1, Metal=10, Stone=3, Cloth=5, Herbs=8, Gold=100, Custom(_) = 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use entropic_world_core::economy::resource::ResourceType;
    ///
    /// assert_eq!(ResourceType::Food.base_value(), 2);
    /// assert_eq!(ResourceType::Gold.base_value(), 100);
    /// ```
    pub fn base_value(&self) -> u32 {
        match self {
            ResourceType::Food => 2,
            ResourceType::Wood => 1,
            ResourceType::Metal => 10,
            ResourceType::Stone => 3,
            ResourceType::Cloth => 5,
            ResourceType::Herbs => 8,
            ResourceType::Gold => 100,
            ResourceType::Custom(_) => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_name() {
        assert_eq!(ResourceType::Food.name(), "Food");
        assert_eq!(ResourceType::Metal.name(), "Metal");
    }

    #[test]
    fn test_resource_base_value() {
        assert_eq!(ResourceType::Food.base_value(), 2);
        assert_eq!(ResourceType::Gold.base_value(), 100);
    }
}