use entropic_world_core::constants::DEFAULT_CHUNK_SIZE;

/// Chunk size in meters (256x256 meters per chunk)
pub const CHUNK_SIZE: f32 = DEFAULT_CHUNK_SIZE;

/// Heightmap resolution per chunk (256x256 height samples)
pub const HEIGHTMAP_RESOLUTION: usize = 256;

/// Entity radius for collision detection (in meters)
pub const ENTITY_RADIUS: f32 = 1.0;

/// Maximum number of chunks to keep loaded in memory
pub const MAX_LOADED_CHUNKS: usize = 10_000;

/// Default view distance (in chunks)
pub const DEFAULT_VIEW_DISTANCE: u32 = 5;

/// Pathfinding grid cell size (in meters)
pub const PATHFINDING_GRID_SIZE: f32 = 16.0;

/// Maximum pathfinding iterations
pub const MAX_PATHFINDING_ITERATIONS: u32 = 10_000;

/// Noise octaves for terrain generation
pub const TERRAIN_NOISE_OCTAVES: u32 = 6;

/// Noise frequency for terrain generation
pub const TERRAIN_NOISE_FREQUENCY: f64 = 0.001;

/// Noise frequency for biome generation
pub const BIOME_NOISE_FREQUENCY: f64 = 0.01;

/// Target chunk generation time (in milliseconds)
pub const TARGET_CHUNK_GENERATION_MS: u64 = 50;

/// Target spatial query time (in milliseconds)
pub const TARGET_SPATIAL_QUERY_MS: u64 = 10;

/// Target pathfinding time (in milliseconds)
pub const TARGET_PATHFINDING_MS: u64 = 5;
