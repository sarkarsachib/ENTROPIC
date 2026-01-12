/// Default chunk size (in meters)
pub const DEFAULT_CHUNK_SIZE: f32 = 256.0;

/// Heightmap resolution per chunk
pub const HEIGHTMAP_RESOLUTION: usize = 256;

/// Default spatial index grid size (in meters)
pub const DEFAULT_GRID_SIZE: f32 = 16.0;

/// Default ticks per second
pub const DEFAULT_TICKS_PER_SECOND: u64 = 20;

/// Seconds per tick
pub const SECONDS_PER_TICK: f32 = 1.0 / DEFAULT_TICKS_PER_SECOND as f32;

/// Default world time scale (1.0 = real-time)
pub const DEFAULT_TIME_SCALE: f32 = 1.0;

/// Maximum entities per chunk before warning
pub const MAX_ENTITIES_PER_CHUNK: usize = 1000;

/// Default NPC memory capacity (recent events)
pub const DEFAULT_MEMORY_CAPACITY: usize = 100;

/// Default market price volatility
pub const DEFAULT_PRICE_VOLATILITY: f32 = 0.15;
