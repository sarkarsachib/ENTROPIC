use thiserror::Error;

/// Errors that can occur in the spatial engine
#[derive(Error, Debug)]
pub enum SpatialError {
    #[error("Chunk at {coord:?} not found")]
    ChunkNotFound { coord: ChunkCoord },

    #[error("Chunk generation failed for {coord:?}: {message}")]
    ChunkGenerationFailed { coord: ChunkCoord, message: String },

    #[error("Chunk at {coord:?} is already loaded")]
    ChunkAlreadyLoaded { coord: ChunkCoord },

    #[error("Load queue is full")]
    LoadQueueFull,

    #[error("Unload queue is full")]
    UnloadQueueFull,

    #[error("Spatial query failed: {message}")]
    QueryFailed { message: String },

    #[error("Pathfinding failed: {message}")]
    PathfindingFailed { message: String },

    #[error("Collision detection failed: {message}")]
    CollisionError { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    #[error("Deserialization error: {message}")]
    DeserializationError { message: String },

    #[error("IO error: {message}")]
    IoError { message: String },

    #[error("Invalid coordinates: {message}")]
    InvalidCoordinates { message: String },

    #[error("Terrain generation error: {message}")]
    TerrainError { message: String },

    #[error("Out of memory")]
    OutOfMemory,
}

pub use entropic_world_core::ChunkCoord;
