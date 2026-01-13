//! # Entropic Spatial Engine
//!
//! High-performance spatial engine that manages world chunks, handles streaming/loading/unloading,
//! provides efficient spatial queries, and supports deterministic terrain generation.
//!
//! ## Features
//!
//! - **Chunk Manager**: Asynchronous chunk loading/unloading with priority queues
//! - **Terrain Generator**: Deterministic procedural generation using Perlin noise
//! - **Spatial Queries**: Fast radius, nearest entity, and raycast queries
//! - **Pathfinding**: A* algorithm with terrain awareness
//! - **Collision Detection**: Circle and terrain-based collision
//! - **LOD System**: Progressive detail levels for distant terrain
//!
//! ## Example
//!
//! ```rust,no_run
//! use entropic_spatial_engine::{ChunkManager, TerrainGenerator};
//! use entropic_world_core::World;
//! use std::sync::Arc;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!     let world = Arc::new(World::new("Test World".to_string(), "game1".to_string(), 100, 100));
//!     let manager = ChunkManager::new(world.clone(), 5);
//!
//!     // Update visible chunks based on player position
//!     manager.update_visible_chunks(1000.0, 1000.0).await.unwrap();
//!
//!     // Process load queue
//!     while let Some(coord) = manager.process_load_queue().await.unwrap() {
//!         println!("Loaded chunk: {:?}", coord);
//!     }
//! }
//! ```

pub mod chunk_manager;
pub mod collision;
pub mod constants;
pub mod errors;
pub mod lod;
pub mod noise;
pub mod pathfinding;
pub mod serialization;
pub mod spatial_queries;
pub mod streaming;
pub mod terrain_generator;

// Optional 3D features
#[cfg(feature = "voxel")]
pub mod voxel;

#[cfg(feature = "octree")]
pub mod octree;

pub use chunk_manager::{ChunkManager, Priority};
pub use collision::CollisionDetector;
pub use errors::SpatialError;
pub use lod::{LODLevel, LODManager};
pub use pathfinding::Pathfinder;
pub use spatial_queries::SpatialQueries;
pub use terrain_generator::TerrainGenerator;

pub use entropic_world_core::{
    Chunk, ChunkCoord, Entity, Biome, World,
};

// Re-export constants
pub use constants::{
    CHUNK_SIZE, HEIGHTMAP_RESOLUTION, ENTITY_RADIUS,
    MAX_LOADED_CHUNKS, DEFAULT_VIEW_DISTANCE,
};

// Re-export noise functions
pub use noise::{NoiseFn, Perlin, Simplex};
