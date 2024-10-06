pub trait Apply {
    type Event;

    fn apply(&mut self, event: &Self::Event);
}
