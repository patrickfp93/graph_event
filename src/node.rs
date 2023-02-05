use std::ops::Deref;

use armc::Armc;

use crate::util::{Event, EventMethod, Condition, NodeError};

pub struct Node<T: ?Sized> {
    pub(crate) counter: Armc<u64>,
    pub(crate) value: Armc<Box<T>>,
    notifications: Armc<Vec<Event<T>>>,
    observations: Armc<Vec<Event<T>>>,
}

impl<T> Node<T>{
    /// Creates a new [`Node<T>`].
    /// 
    ///
    /// # Examples
    ///
    /// ```
    /// use node_event::node::Node;
    ///
    /// assert_eq!(*Node::new(10),10);
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            counter: Armc::new(0),
            value: Armc::new(Box::new(value)),
            notifications: Armc::new(vec![]),
            observations: Armc::new(vec![]),
        }
    }
}

impl<T : ?Sized> Node<T> {   

    fn new_by_box(value: Box<T>) -> Self {
        Self {
            counter: Armc::new(0),
            value: Armc::new(value),
            notifications: Armc::new(vec![]),
            observations: Armc::new(vec![]),
        }
    }

    /// This method tries to mark a notification for a given node. It creates a new event
    /// with the current node, the given node, the event method, and the condition, and adds it to the list of 
    /// notifications of the current node. The method returns 'Ok(())' if the new event was successfully added.
    /// # Errors
    /// if the given node is already present in the list of notifications of the current node, it returns an error of type 'NodeError::AlreadyExists'.
    pub fn try_mark_for_notification(
        &mut self,
        other: Node<T>,
        event: EventMethod<T>,
        condition: Condition,
    ) -> Result<(), NodeError> {
        let try_finded = self.notifications.iter().find(|&e| e.mutable == other);
        if let Some(_) = try_finded {
            return Err(NodeError::AlreadyExists);
        } else {
            let notif = Event::new(self.clone(), other.clone(), event, condition);
            self.notifications.lock().push(notif);
            Ok(())
        }
    }

    ///This method try_mark_off_notification is used to remove a notification associated with a node. 
    /// It takes a Node as an argument and returns a Result type with either an Ok variant containing unit () or an Err variant containing a NodeError.
    ///The method first tries to find the given node other in the notifications list by using the
    /// iter().enumerate().find method and comparing the mutable field of each element with the other node.
    /// If a matching node is found, the method removes it from the list using the remove method and returns Ok(()).
    /// # Errors
    /// If no matching node is found, it returns Err(NodeError::DoesNotExist).
    pub fn try_mark_off_notification(&mut self, other: Node<T>) -> Result<(), NodeError> {
        let try_finded = self
            .notifications
            .iter()
            .enumerate()
            .find(|(_, e)| e.mutable == other);
        if let Some((index, _)) = try_finded {
            self.notifications.lock().remove(index);
            return Ok(());
        } else {
            return Err(NodeError::DoesNotExist);
        }
    }

    /// This method tries to mark a observation for a given node. It creates a new event
    /// with the current node, the given node, the event method, and the condition, and adds it to the list of 
    /// observation of the current node. The method returns [`Ok(())`] if the new event was successfully added.
    /// # Errors
    /// if the given node is already present in the list of observation of the current node, it returns an error of type [`NodeError::AlreadyExists`].
    pub fn try_mark_for_observation(
        &mut self,
        other: Node<T>,
        event: EventMethod<T>,
        condition: Condition,
    ) -> Result<(), NodeError> {
        let try_finded = self.observations.iter().find(|&e| e.mutable == other);
        if let Some(_) = try_finded {
            return Err(NodeError::AlreadyExists);
        } else {
            let obs = Event::new(other.clone(), self.clone(), event, condition);
            self.observations.lock().push(obs);
            Ok(())
        }
    }

    ///This method try_mark_off_observation is used to remove a observation associated with a node. 
    /// It takes a Node as an argument and returns a Result type with either an Ok variant containing unit () or an Err variant containing a NodeError.
    ///The method first tries to find the given node other in the observation list by using the
    /// [`iter().enumerate().find`] method and comparing the mutable field of each element with the other node.
    /// If a matching node is found, the method removes it from the list using the remove method and returns Ok(()).
    /// # Errors
    /// If no matching node is found, it returns Err(NodeError::DoesNotExist).
    pub fn try_mark_off_observation(&mut self, other: Node<T>) -> Result<(), NodeError> {
        let try_finded = self
            .observations
            .iter()
            .enumerate()
            .find(|(_, e)| e.mutable == other);
        if let Some((index, _)) = try_finded {
            self.observations.lock().remove(index);
            return Ok(());
        } else {
            return Err(NodeError::DoesNotExist);
        }
    }    

    //This method "watch_for_updates" iterates over the elements of "observations" which is a lock, and calls the "check" method on each element of "observations".
    pub fn watch_for_updates(&mut self){
        self.observations.lock().iter_mut().for_each(|obs|{
            obs.check()
        });
    }

}

impl<T: PartialEq> Node<T>{
    
    /// This method updates the value of the Node. It first checks if the new value is different from the current value. 
    /// If they are different, it calls the set_and_notify method with the new value, which will update the value and notify 
    /// all the events associated with this node.
    pub fn update(&mut self,new_value : T){
        if new_value != **self.value{
            self.set_and_notify(Box::new(new_value))
        }
    }
}

impl<T:?Sized> Node<T> {

    /// Sets the and notify of this [`Node<T>`].
    /// The set_and_notify method takes in a new value as a Box, sets the value of the Node to the new value, and then calls the make_notify method.
    pub fn set_and_notify(&mut self,new_value : Box<T>){
            self.set(new_value);
            self.make_notify();
    }

    /// The set method simply updates the value of the Node to the new value.
    pub fn set(&mut self,new_value : Box<T>){
        *self.value.lock() = new_value;
    }
    
    /// Returns the make notify of this [`Node<T>`].
    /// The make_notify method iterates over the notifications associated with the Node and calls the check method on each notification to potentially trigger a notification event.
    pub fn make_notify(&mut self){
        self.notifications.lock().iter_mut().for_each(|notify|{
            notify.check()
        });
    }

}

impl<T: ?Sized> Deref for Node<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.deref()
    }
}


impl<T : ?Sized> Clone for Node<T>{
    fn clone(&self) -> Self {
        Self { counter: self.counter.clone(), value: self.value.clone(), notifications: self.notifications.clone(), observations: self.observations.clone() }
    }
}


impl<T : ?Sized> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: ?Sized> From<Box<T>> for Node<T>{
    fn from(value : Box<T>) -> Self {
        Self::new_by_box(value)
    }
}

