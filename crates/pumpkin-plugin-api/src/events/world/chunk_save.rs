use crate::wit::pumpkin::plugin::event::{ChunkSaveEventData, Event, EventType};

use super::super::FromIntoEvent;

/// An event that occurs after a chunk has been saved in a world.
///
/// Cancelling it has no effect: the chunk is already written.
pub struct ChunkSaveEvent;

impl FromIntoEvent for ChunkSaveEvent {
    const EVENT_TYPE: EventType = EventType::ChunkSaveEvent;
    type Data = ChunkSaveEventData;

    fn data_from_event(event: Event) -> Self::Data {
        match event {
            Event::ChunkSaveEvent(data) => data,
            _ => panic!("unexpected event"),
        }
    }

    fn data_into_event(data: Self::Data) -> Event {
        Event::ChunkSaveEvent(data)
    }
}
