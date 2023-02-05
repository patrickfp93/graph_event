use crate::node::Node;


pub type EventMethod<T> = fn(reference : &T,mutable : &mut T);

pub struct Event<T: ? Sized> {
    current_counter: u64,
    pub(crate) reference: Node<T>,
    pub(crate) mutable: Node<T>,
    event: EventMethod<T>,
    condition: Condition,
}

impl<T: ?Sized> Event<T> {
    pub fn new(
        reference: Node<T>,
        mutable: Node<T>,
        event: EventMethod<T>,
        condition: Condition,
    ) -> Self {
        Self {
            current_counter: 0,
            reference,
            mutable,
            event,
            condition,
        }
    }

    pub fn check(&mut self) {
        let event = &mut self.event;
        match self.condition {
            Condition::Always => event(self.reference.value.as_ref(), self.mutable.value.lock().as_mut()),
            Condition::ToUpdate => if *self.reference.counter.as_ref() < self.current_counter {
                event(self.reference.value.as_ref(), self.mutable.value.lock().as_mut()) 
            },
            Condition::WhenDifferent => if *self.reference.counter.as_ref() != self.current_counter {
                event(self.reference.value.as_ref(), self.mutable.value.lock().as_mut()) 
            },
            Condition::WhenEqual => if *self.reference.counter.as_ref() == self.current_counter {
                event(self.reference.value.as_ref(), self.mutable.value.lock().as_mut()) 
            },
        }
    }

}

pub enum Condition {
    Always,
    ToUpdate,
    WhenDifferent,
    WhenEqual
}

pub enum NodeError {
    AlreadyExists,
    DoesNotExist,
}

impl From<NodeError> for String{
    fn from(error: NodeError) -> Self {
        match error {
            NodeError::AlreadyExists => "Already exists!".into(),
            NodeError::DoesNotExist => "Does not exist!".into(),
        }
    }
}
