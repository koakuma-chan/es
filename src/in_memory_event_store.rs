use crate::{Apply, EventStore};

use std::{convert::Infallible, marker::PhantomData};

use nohash_hasher::IntMap;

pub struct InMemoryEventStore<A, E>
where
    //
    A: Apply<Event = E>,
{
    streams: IntMap<u64, Vec<E>>,

    _phantom_data: PhantomData<A>,
}

impl<A, E> Default for InMemoryEventStore<A, E>
where
    //
    A: Apply<Event = E>,
{
    fn default() -> Self {
        Self {
            streams: Default::default(),

            _phantom_data: PhantomData::default(),
        }
    }
}

impl<A, E> EventStore<A, E> for InMemoryEventStore<A, E>
where
    //
    A: Apply<Event = E>,
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
