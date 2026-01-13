//! Octree spatial index for 3D spatial queries
//!
//! This module is only available when the "octree" feature is enabled.

use crate::errors::SpatialError;

/// Octree node
#[derive(Debug)]
pub enum OctreeNode {
    Leaf { bounds: Bounds, data: Vec<(f32, f32, f32, String)> },
    Internal {
        bounds: Bounds,
        children: [Box<OctreeNode>; 8],
    },
}

/// Bounding box
#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

impl Bounds {
    /// Create new bounds
    pub fn new(min_x: f32, min_y: f32, min_z: f32, max_x: f32, max_y: f32, max_z: f32) -> Self {
        Self {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        }
    }

    /// Check if point is inside bounds
    pub fn contains(&self, x: f32, y: f32, z: f32) -> bool {
        x >= self.min_x && x <= self.max_x &&
        y >= self.min_y && y <= self.max_y &&
        z >= self.min_z && z <= self.max_z
    }

    /// Check if bounds intersects sphere
    pub fn intersects_sphere(&self, cx: f32, cy: f32, cz: f32, radius: f32) -> bool {
        // Find closest point on box to sphere center
        let closest_x = cx.max(self.min_x).min(self.max_x);
        let closest_y = cy.max(self.min_y).min(self.max_y);
        let closest_z = cz.max(self.min_z).min(self.max_z);

        let dx = closest_x - cx;
        let dy = closest_y - cy;
        let dz = closest_z - cz;

        (dx * dx + dy * dy + dz * dz) <= radius * radius
    }
}

/// Octree spatial index
#[derive(Debug)]
pub struct Octree {
    root: Option<OctreeNode>,
    max_objects: usize,
}

impl Octree {
    /// Create new octree
    pub fn new(max_objects: usize) -> Self {
        Self {
            root: None,
            max_objects,
        }
    }

    /// Insert object into octree
    pub fn insert(&mut self, x: f32, y: f32, z: f32, id: String) -> Result<(), SpatialError> {
        if self.root.is_none() {
            self.root = Some(OctreeNode::Leaf {
                bounds: Bounds::new(-1000.0, -1000.0, -1000.0, 1000.0, 1000.0, 1000.0),
                data: vec![(x, y, z, id)],
            });
            return Ok(());
        }

        if let Some(root) = &mut self.root {
            Self::insert_recursive(root, x, y, z, id, self.max_objects, 0);
        }

        Ok(())
    }

    fn insert_recursive(
        node: &mut OctreeNode,
        x: f32,
        y: f32,
        z: f32,
        id: String,
        max_objects: usize,
        depth: usize,
    ) {
        match node {
            OctreeNode::Leaf { bounds, data } => {
                if data.len() < max_objects || depth > 10 {
                    data.push((x, y, z, id));
                } else {
                    // Subdivide
                    let children = Self::subdivide(*bounds);
                    let old_data = std::mem::replace(data, vec![]);

                    for (ox, oy, oz, oid) in old_data {
                        let child_index = Self::get_child_index(*bounds, ox, oy, oz);
                        Self::insert_recursive(&mut children[child_index], ox, oy, oz, oid, max_objects, depth + 1);
                    }

                    let child_index = Self::get_child_index(*bounds, x, y, z);
                    Self::insert_recursive(&mut children[child_index], x, y, z, id, max_objects, depth + 1);

                    *node = OctreeNode::Internal {
                        bounds: *bounds,
                        children,
                    };
                }
            }
            OctreeNode::Internal { bounds, children } => {
                let child_index = Self::get_child_index(*bounds, x, y, z);
                Self::insert_recursive(&mut children[child_index], x, y, z, id, max_objects, depth + 1);
            }
        }
    }

    fn subdivide(bounds: Bounds) -> [Box<OctreeNode>; 8] {
        let cx = (bounds.min_x + bounds.max_x) / 2.0;
        let cy = (bounds.min_y + bounds.max_y) / 2.0;
        let cz = (bounds.min_z + bounds.max_z) / 2.0;

        [
            Box::new(OctreeNode::Leaf {
                bounds: Bounds::new(bounds.min_x, bounds.min_y, bounds.min_z, cx, cy, cz),
                data: vec![],
            }),
            Box::new(OctreeNode::Leaf {
                bounds: Bounds::new(cx, bounds.min_y, bounds.min_z, bounds.max_x, cy, cz),
                data: vec![],
            }),
            Box::new(OctreeNode::Leaf {
                bounds: Bounds::new(bounds.min_x, cy, bounds.min_z, cx, bounds.max_y, cz),
                data: vec![],
            }),
            Box::new(OctreeNode::Leaf {
                bounds: Bounds::new(cx, cy, bounds.min_z, bounds.max_x, bounds.max_y, cz),
                data: vec![],
            }),
            Box::new(OctreeNode::Leaf {
                bounds: Bounds::new(bounds.min_x, bounds.min_y, cz, cx, cy, bounds.max_z),
                data: vec![],
            }),
            Box::new(OctreeNode::Leaf {
                bounds: Bounds::new(cx, bounds.min_y, cz, bounds.max_x, cy, bounds.max_z),
                data: vec![],
            }),
            Box::new(OctreeNode::Leaf {
                bounds: Bounds::new(bounds.min_x, cy, cz, cx, bounds.max_y, bounds.max_z),
                data: vec![],
            }),
            Box::new(OctreeNode::Leaf {
                bounds: Bounds::new(cx, cy, cz, bounds.max_x, bounds.max_y, bounds.max_z),
                data: vec![],
            }),
        ]
    }

    fn get_child_index(bounds: Bounds, x: f32, y: f32, z: f32) -> usize {
        let cx = (bounds.min_x + bounds.max_x) / 2.0;
        let cy = (bounds.min_y + bounds.max_y) / 2.0;
        let cz = (bounds.min_z + bounds.max_z) / 2.0;

        let mut index = 0;
        if x >= cx { index |= 1; }
        if y >= cy { index |= 2; }
        if z >= cz { index |= 4; }
        index
    }

    /// Query objects within radius
    pub fn query_radius(&self, cx: f32, cy: f32, cz: f32, radius: f32) -> Vec<String> {
        match &self.root {
            Some(root) => Self::query_recursive(root, cx, cy, cz, radius),
            None => vec![],
        }
    }

    fn query_recursive(
        node: &OctreeNode,
        cx: f32,
        cy: f32,
        cz: f32,
        radius: f32,
    ) -> Vec<String> {
        match node {
            OctreeNode::Leaf { bounds, data } => {
                if bounds.intersects_sphere(cx, cy, cz, radius) {
                    data.iter()
                        .filter(|t| {
                            let (x, y, z, _) = t;
                            let dx = x - cx;
                            let dy = y - cy;
                            let dz = z - cz;
                            (dx * dx + dy * dy + dz * dz).sqrt() <= radius
                        })
                        .map(|t| t.3.clone())
                        .collect()
                } else {
                    vec![]
                }
            }
            OctreeNode::Internal { bounds, children } => {
                if bounds.intersects_sphere(cx, cy, cz, radius) {
                    let mut results = vec![];
                    for child in children {
                        results.extend(Self::query_recursive(child, cx, cy, cz, radius));
                    }
                    results
                } else {
                    vec![]
                }
            }
        }
    }

    /// Remove all objects
    pub fn clear(&mut self) {
        self.root = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octree_creation() {
        let octree = Octree::new(10);
        assert!(octree.root.is_none());
    }

    #[test]
    fn test_octree_insert() {
        let mut octree = Octree::new(2);
        octree.insert(1.0, 1.0, 1.0, "object1".to_string()).unwrap();

        assert!(octree.root.is_some());
    }

    #[test]
    fn test_octree_query_radius() {
        let mut octree = Octree::new(10);
        octree.insert(1.0, 1.0, 1.0, "object1".to_string()).unwrap();
        octree.insert(2.0, 2.0, 2.0, "object2".to_string()).unwrap();
        octree.insert(10.0, 10.0, 10.0, "object3".to_string()).unwrap();

        let results = octree.query_radius(1.0, 1.0, 1.0, 5.0);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_bounds_contains() {
        let bounds = Bounds::new(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        assert!(bounds.contains(5.0, 5.0, 5.0));
        assert!(!bounds.contains(15.0, 5.0, 5.0));
    }

    #[test]
    fn test_octree_clear() {
        let mut octree = Octree::new(10);
        octree.insert(1.0, 1.0, 1.0, "object1".to_string()).unwrap();
        octree.clear();

        assert!(octree.root.is_none());
    }
}
