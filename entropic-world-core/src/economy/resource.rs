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
