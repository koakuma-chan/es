pub trait EventStore<E> {
    type Error;

    fn append(
        &mut self,
        //
        stream_id: u64,
        //
        events: Vec<E>,
    ) -> Result<(), Self::Error>;

    fn stream(
        //
        &self,
        //
        stream_id: u64,
    ) -> Result<impl Iterator<Item = Result<E, Self::Error>>, Self::Error>;
}
