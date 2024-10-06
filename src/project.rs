use crate::{Apply, EventStore};

pub trait Project<A> {
    type Error;

    fn project(
        //
        &self,
        //
        stream_id: u64,
    ) -> Result<A, Self::Error>;
}

impl<T, A, E> Project<A> for T
where
    //
    T: EventStore<A, E>,
    //
    A: Apply<Event = E> + Default,
{
    type Error = <T as EventStore<A, E>>::Error;

    fn project(
        //
        &self,
        //
        stream_id: u64,
    ) -> Result<A, Self::Error> {
        let mut aggregate = A::default();

        let stream = self.stream(stream_id)?;

        for event in stream {
            let event = event?;

            aggregate.apply(&event);
        }

        Ok(aggregate)
    }
}
