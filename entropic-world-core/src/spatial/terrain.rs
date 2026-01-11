use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Biome {
    Forest,
    Desert,
    Mountains,
    Plains,
    Swamp,
    Tundra,
    Ocean,
    Grassland,
    Custom(u32),
}

impl Default for Biome {
    fn default() -> Self {
        Biome::Plains
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StructureType {
    House,
    Castle,
    Tower,
    Farm,
    Market,
    Temple,
    Barracks,
    Inn,
    Workshop,
    Bridge,
    Custom(u32),
}

pub type StructureId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Structure {
    pub id: StructureId,
    pub structure_type: StructureType,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub owner: Option<String>,
    pub faction: Option<String>,
    pub built_at: crate::temporal::time::WorldTime,
    pub condition: f32,
}

impl Structure {
    pub fn new(
        id: StructureId,
        structure_type: StructureType,
        x: f32,
        y: f32,
        z: f32,
    ) -> Self {
        Self {
            id,
            structure_type,
            x,
            y,
            z,
            owner: None,
            faction: None,
            built_at: crate::temporal::time::WorldTime::default(),
            condition: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biome_default() {
        let biome = Biome::default();
        assert_eq!(biome, Biome::Plains);
    }

    #[test]
    fn test_structure_creation() {
        let structure = Structure::new(
            "struct_1".to_string(),
            StructureType::House,
            100.0,
            200.0,
            0.0,
        );
        assert_eq!(structure.structure_type, StructureType::House);
        assert_eq!(structure.condition, 1.0);
    }
}
