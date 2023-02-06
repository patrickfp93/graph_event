use crate::{node::Node, util::{EventMethod, NotificationPolicy}};

pub struct Notification<T: ?Sized> {
    pub(crate) reference: Node<T>,
    pub(crate) mutable: Node<T>,
    event: EventMethod<T>,
    default_notification_policy: NotificationPolicy,
}

impl<T: ?Sized> Notification<T> {
    pub fn new(
        reference: Node<T>,
        mutable: Node<T>,
        event: EventMethod<T>,
        default_notification_policy: NotificationPolicy,
    ) -> Self {
        Self {
            reference,
            mutable,
            event,
            default_notification_policy,
        }
    }

    pub fn send(&mut self, policy : NotificationPolicy){
        if policy.should_notify(*self.reference.counter.as_ref(), *self.reference.counter){
            let event = &mut self.event;
            if event(
                self.reference.value.as_ref(),
                self.mutable.value.lock().as_mut(),
            ){
                self.mutable.notify_to_neighbors();
            }
        }
    }

    pub fn send_with_default_policy(&mut self) {
        self.send(self.default_notification_policy);
    }
}