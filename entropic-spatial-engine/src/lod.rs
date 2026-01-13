/// Level-of-detail (LOD) system for progressive terrain detail
#[derive(Clone)]
pub struct LODLevel {
    /// Distance threshold for this LOD level
    pub distance: f32,

    /// Detail scale (1.0 = full detail, 0.5 = half detail)
    pub detail_scale: f32,

    /// Heightmap resolution for this level
    pub heightmap_resolution: usize,

    /// Vegetation resolution for this level
    pub vegetation_resolution: usize,
}

impl LODLevel {
    /// Create a new LOD level
    pub fn new(distance: f32, detail_scale: f32) -> Self {
        Self {
            distance,
            detail_scale,
            heightmap_resolution: (crate::HEIGHTMAP_RESOLUTION as f32 * detail_scale) as usize,
            vegetation_resolution: (crate::HEIGHTMAP_RESOLUTION as f32 * detail_scale) as usize,
        }
    }
}

/// LOD manager for progressive terrain detail
pub struct LODManager {
    levels: Vec<LODLevel>,
}

impl LODManager {
    /// Create a new LOD manager with default levels
    pub fn new() -> Self {
        Self {
            levels: vec![
                LODLevel::new(0.0, 1.0),
                LODLevel::new(512.0, 0.5),
                LODLevel::new(1024.0, 0.25),
                LODLevel::new(2048.0, 0.125),
            ],
        }
    }

    /// Create a new LOD manager with custom levels
    pub fn with_levels(levels: Vec<LODLevel>) -> Self {
        let mut sorted_levels = levels;
        sorted_levels.sort_by_key(|l| l.distance as i32);

        Self {
            levels: sorted_levels,
        }
    }

    /// Get appropriate LOD level for distance
    pub fn get_lod_level(&self, distance: f32) -> &LODLevel {
        self.levels
            .iter()
            .rev()
            .find(|l| l.distance <= distance)
            .unwrap_or(&self.levels[self.levels.len() - 1])
    }

    /// Get all LOD levels
    pub fn levels(&self) -> &[LODLevel] {
        &self.levels
    }

    /// Add a new LOD level
    pub fn add_level(&mut self, level: LODLevel) {
        self.levels.push(level);
        self.levels.sort_by_key(|l| l.distance as i32);
    }

    /// Remove LOD level at index
    pub fn remove_level(&mut self, index: usize) {
        if index < self.levels.len() {
            self.levels.remove(index);
        }
    }

    /// Calculate detail scale for distance
    pub fn get_detail_scale(&self, distance: f32) -> f32 {
        self.get_lod_level(distance).detail_scale
    }

    /// Check if LOD should change for new distance
    pub fn should_change_lod(&self, old_distance: f32, new_distance: f32) -> bool {
        let old_lod = self.get_detail_scale(old_distance);
        let new_lod = self.get_detail_scale(new_distance);
        old_lod != new_lod
    }
}

impl Default for LODManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lod_manager_creation() {
        let manager = LODManager::new();
        assert_eq!(manager.levels().len(), 4);
    }

    #[test]
    fn test_get_lod_level() {
        let manager = LODManager::new();

        let lod = manager.get_lod_level(0.0);
        assert_eq!(lod.distance, 0.0);
        assert_eq!(lod.detail_scale, 1.0);

        let lod = manager.get_lod_level(600.0);
        assert_eq!(lod.distance, 512.0);
        assert_eq!(lod.detail_scale, 0.5);

        let lod = manager.get_lod_level(3000.0);
        assert_eq!(lod.distance, 2048.0);
        assert_eq!(lod.detail_scale, 0.125);
    }

    #[test]
    fn test_get_detail_scale() {
        let manager = LODManager::new();

        assert_eq!(manager.get_detail_scale(0.0), 1.0);
        assert_eq!(manager.get_detail_scale(600.0), 0.5);
        assert_eq!(manager.get_detail_scale(3000.0), 0.125);
    }

    #[test]
    fn test_should_change_lod() {
        let manager = LODManager::new();

        assert!(!manager.should_change_lod(100.0, 200.0));
        assert!(manager.should_change_lod(100.0, 600.0));
    }

    #[test]
    fn test_add_level() {
        let mut manager = LODManager::new();
        manager.add_level(LODLevel::new(256.0, 0.75));

        assert_eq!(manager.levels().len(), 5);
    }

    #[test]
    fn test_remove_level() {
        let mut manager = LODManager::new();
        manager.remove_level(0);

        assert_eq!(manager.levels().len(), 3);
    }

    #[test]
    fn test_with_levels() {
        let levels = vec![
            LODLevel::new(0.0, 1.0),
            LODLevel::new(1000.0, 0.5),
        ];

        let manager = LODManager::with_levels(levels.clone());
        assert_eq!(manager.levels().len(), 2);
    }
}
