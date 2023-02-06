use crate::{node::Node, util::EventMethod};

pub struct Observation<T: ?Sized> {
    pub(crate) reference: Node<T>,
    pub(crate) mutable: Node<T>,
    event: EventMethod<T>,
    counter: u64,
}

impl<T: ?Sized> Observation<T> {
    pub fn new(reference: Node<T>, mutable: Node<T>, event: EventMethod<T>) -> Self {
        let counter = *reference.counter;
        Self {
            reference,
            mutable,
            event,
            counter,
        }
    }

    pub fn check(&mut self) {
        if self.counter != *self.reference.counter{
            self.counter = *self.reference.counter;
            let event = &mut self.event;
            if event(self.reference.value.as_ref(),self.mutable.value.lock().as_mut()){
                self.mutable.notify_to_neighbors()
            }
        }
    }
}