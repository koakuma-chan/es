use crate::{Apply, EventStore, Handle, Project};

pub trait Execute<A>
where
    //
    A: Handle,
{
    type Error;

    fn execute(
        //
        &mut self,
        //
        stream_id: u64,
        //
        command: &<A as Handle>::Command,
    ) -> Result<(), Self::Error>;
}

impl<T, A, E> Execute<A> for T
where
    //
    T: EventStore<E> + Project<A>,
    //
    A: Apply<Event = E> + Handle<Event = E>,
{
    type Error = Error<
        //
        <A as Handle>::Error,
        //
        <T as Project<A>>::Error,
        //
        <T as EventStore<E>>::Error,
    >;

    fn execute(
        //
        &mut self,
        //
        stream_id: u64,
        //
        command: &<A as Handle>::Command,
    ) -> Result<(), Self::Error> {
        let aggregate: A = self
            //
            .project(stream_id)
            //
            .map_err(Error::Project)?;

        let events = aggregate
            //
            .handle(command)
            //
            .map_err(Error::Handle)?;

        self
            //
            .append(stream_id, events)
            //
            .map_err(Error::EventStore)?;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error<H, P, S> {
    #[error("handle error: {0}")]
    Handle(H),

    #[error("project error: {0}")]
    Project(P),

    #[error("event store error: {0}")]
    EventStore(S),
}
