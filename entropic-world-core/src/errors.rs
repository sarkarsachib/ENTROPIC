use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorldError {
    #[error("Invalid chunk coordinate: ({0}, {1})")]
    InvalidChunkCoord(u32, u32),

    #[error("Chunk not loaded at ({0}, {1})")]
    ChunkNotLoaded(u32, u32),

    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("NPC not found: {0}")]
    NpcNotFound(String),

    #[error("Faction not found: {0}")]
    FactionNotFound(String),

    #[error("Settlement not found: {0}")]
    SettlementNotFound(String),

    #[error("Invalid world state: {0}")]
    InvalidWorldState(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Version mismatch: expected {expected}, found {found}")]
    VersionMismatch { expected: String, found: String },

    #[error("Invalid time: {0}")]
    InvalidTime(String),

    #[error("Spatial index error: {0}")]
    SpatialIndexError(String),
}

pub type Result<T> = std::result::Result<T, WorldError>;
