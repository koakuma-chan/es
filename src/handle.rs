pub trait Handle {
    type Command;

    type Event;

    type Error;

    fn handle(&self, command: &Self::Command) -> Result<Vec<Self::Event>, Self::Error>;
}
