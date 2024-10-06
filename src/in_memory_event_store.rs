use crate::EventStore;

use std::convert::Infallible;

use nohash_hasher::IntMap;

pub struct InMemoryEventStore<E> {
    streams: IntMap<u64, Vec<E>>,
}

impl<E> Default for InMemoryEventStore<E> {
    fn default() -> Self {
        Self {
            streams: Default::default(),
        }
    }
}

impl<E> EventStore<E> for InMemoryEventStore<E>
where
    //
    E: Clone,
{
    type Error = Infallible;

    fn append(
        //
        &mut self,
        //
        entity_id: u64,
        //
        events: Vec<E>,
    ) -> Result<(), Self::Error> {
        self.streams
            //
            .entry(entity_id)
            //
            .or_default()
            //
            .extend(events);

        Ok(())
    }

    fn stream(
        //
        &self,
        //
        entity_id: u64,
    ) -> Result<impl Iterator<Item = Result<E, Self::Error>>, Self::Error> {
        let events = self
            //
            .streams
            //
            .get(&entity_id)
            //
            .cloned()
            //
            .unwrap_or_default();

        Ok(
            //
            events
                //
                .into_iter()
                //
                .map(Ok),
        )
    }
}
