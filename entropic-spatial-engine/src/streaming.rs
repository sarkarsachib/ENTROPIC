use crate::chunk_manager::{ChunkManager, Priority};
use crate::constants::*;
use crate::errors::SpatialError;
use crate::{ChunkCoord, World};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Chunk streaming protocol for async loading/unloading
pub struct ChunkStreamer {
    manager: Arc<ChunkManager>,
    load_tx: mpsc::UnboundedSender<StreamingCommand>,
}

/// Commands for chunk streaming
#[derive(Debug)]
pub enum StreamingCommand {
    UpdatePosition { x: f32, y: f32 },
    LoadChunk { coord: ChunkCoord, priority: Priority },
    UnloadChunk { coord: ChunkCoord },
    Pause,
    Resume,
    Shutdown,
}

/// Events emitted during streaming
#[derive(Debug, Clone)]
pub enum StreamingEvent {
    ChunkLoaded { coord: ChunkCoord },
    ChunkUnloaded { coord: ChunkCoord },
    StreamPaused,
    StreamResumed,
    Error { message: String },
}

impl ChunkStreamer {
    /// Create a new chunk streamer
    pub fn new(manager: Arc<ChunkManager>) -> (Self, mpsc::UnboundedReceiver<StreamingEvent>) {
        let (load_tx, load_rx) = mpsc::unbounded_channel();
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let streamer = Self {
            manager,
            load_tx,
        };

        // Spawn streaming task
        tokio::spawn(streaming_task(manager.clone(), load_rx, event_tx));

        (streamer, event_rx)
    }

    /// Send command to streamer
    pub fn send(&self, command: StreamingCommand) -> Result<(), SpatialError> {
        self.load_tx
            .send(command)
            .map_err(|_| SpatialError::LoadQueueFull)
    }

    /// Update player position (trigger chunk loading)
    pub fn update_position(&self, x: f32, y: f32) -> Result<(), SpatialError> {
        self.send(StreamingCommand::UpdatePosition { x, y })
    }

    /// Request loading of a specific chunk
    pub fn load_chunk(&self, coord: ChunkCoord, priority: Priority) -> Result<(), SpatialError> {
        self.send(StreamingCommand::LoadChunk { coord, priority })
    }

    /// Request unloading of a specific chunk
    pub fn unload_chunk(&self, coord: ChunkCoord) -> Result<(), SpatialError> {
        self.send(StreamingCommand::UnloadChunk { coord })
    }

    /// Pause streaming
    pub fn pause(&self) -> Result<(), SpatialError> {
        self.send(StreamingCommand::Pause)
    }

    /// Resume streaming
    pub fn resume(&self) -> Result<(), SpatialError> {
        self.send(StreamingCommand::Resume)
    }

    /// Shutdown streamer
    pub fn shutdown(&self) -> Result<(), SpatialError> {
        self.send(StreamingCommand::Shutdown)
    }
}

/// Streaming task that processes commands
async fn streaming_task(
    manager: Arc<ChunkManager>,
    mut command_rx: mpsc::UnboundedReceiver<StreamingCommand>,
    event_tx: mpsc::UnboundedSender<StreamingEvent>,
) {
    let mut paused = false;
    let mut tick_interval = tokio::time::interval(tokio::time::Duration::from_millis(50));

    loop {
        tokio::select! {
            // Process incoming commands
            result = command_rx.recv() => {
                match result {
                    Some(command) => {
                        match command {
                            StreamingCommand::UpdatePosition { x, y } => {
                                if !paused {
                                    if let Err(e) = manager.update_visible_chunks(x, y).await {
                                        let _ = event_tx.send(StreamingEvent::Error {
                                            message: format!("Failed to update visible chunks: {}", e),
                                        });
                                    }
                                }
                            }
                            StreamingCommand::LoadChunk { coord, priority } => {
                                if let Err(e) = manager.queue_chunk(coord, priority) {
                                    let _ = event_tx.send(StreamingEvent::Error {
                                        message: format!("Failed to queue chunk: {}", e),
                                    });
                                }
                            }
                            StreamingCommand::UnloadChunk { coord } => {
                                if let Err(e) = manager.unload_chunk(coord) {
                                    let _ = event_tx.send(StreamingEvent::Error {
                                        message: format!("Failed to unload chunk: {}", e),
                                    });
                                } else {
                                    let _ = event_tx.send(StreamingEvent::ChunkUnloaded { coord });
                                }
                            }
                            StreamingCommand::Pause => {
                                paused = true;
                                let _ = event_tx.send(StreamingEvent::StreamPaused);
                            }
                            StreamingCommand::Resume => {
                                paused = false;
                                let _ = event_tx.send(StreamingEvent::StreamResumed);
                            }
                            StreamingCommand::Shutdown => {
                                break;
                            }
                        }
                    }
                    None => {
                        // Channel closed, shutdown
                        break;
                    }
                }
            }

            // Process load queue on tick
            _ = tick_interval.tick() => {
                if !paused {
                    if let Ok(Some(coord)) = manager.process_load_queue().await {
                        let _ = event_tx.send(StreamingEvent::ChunkLoaded { coord });
                    }

                    // Process unloading
                    if let Ok(unloaded) = manager.process_unload_queue() {
                        for coord in unloaded {
                            let _ = event_tx.send(StreamingEvent::ChunkUnloaded { coord });
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_manager() -> Arc<ChunkManager> {
        let world = Arc::new(tokio::sync::RwLock::new(World::new(
            "Test World".to_string(),
            "game1".to_string(),
            10,
            10,
        )));
        Arc::new(ChunkManager::new(world, 2))
    }

    #[tokio::test]
    async fn test_streamer_creation() {
        let manager = create_test_manager();
        let (streamer, _event_rx) = ChunkStreamer::new(manager);

        assert!(streamer.update_position(100.0, 100.0).is_ok());
    }

    #[tokio::test]
    async fn test_streamer_pause_resume() {
        let manager = create_test_manager();
        let (streamer, _event_rx) = ChunkStreamer::new(manager);

        assert!(streamer.pause().is_ok());
        assert!(streamer.resume().is_ok());
    }

    #[tokio::test]
    async fn test_streamer_load_unload() {
        let manager = create_test_manager();
        let (streamer, _event_rx) = ChunkStreamer::new(manager);

        let coord = ChunkCoord::new(0, 0);
        assert!(streamer.load_chunk(coord, Priority::High).is_ok());
        assert!(streamer.unload_chunk(coord).is_ok());
    }

    #[tokio::test]
    async fn test_streamer_shutdown() {
        let manager = create_test_manager();
        let (streamer, _event_rx) = ChunkStreamer::new(manager);

        assert!(streamer.shutdown().is_ok());
    }
}
