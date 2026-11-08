use crate::world::World;
use pumpkin_macros::{Event, cancellable};
use pumpkin_world::chunk::ChunkData;
use std::sync::Arc;

/// An event that occurs after a chunk has been saved in a world.
///
/// It fires once the chunk was handed to the region storage, so cancelling it has no effect:
/// skipping the write of an unloading chunk would lose its changes.
#[cancellable]
#[derive(Event, Clone)]
pub struct ChunkSave {
    /// The world in which the chunk was saved.
    pub world: Arc<World>,

    /// The chunk data that was saved.
    pub chunk: Arc<ChunkData>,
}

impl ChunkSave {
    pub const fn new(world: Arc<World>, chunk: Arc<ChunkData>) -> Self {
        Self {
            world,
            chunk,
            cancelled: false,
        }
    }
}
