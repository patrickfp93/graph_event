use std::ops::Deref;

use armc::Armc;

use crate::{
    notification::Notification,
    observation::Observation,
    util::{EventMethod, NodeError, NotificationPolicy},
};

pub struct Node<T: ?Sized> {
    pub(crate) counter: Armc<u64>,
    pub(crate) value: Armc<Box<T>>,
    notifications: Armc<Vec<Notification<T>>>,
    observations: Armc<Vec<Observation<T>>>,
}

impl<T> Node<T> {
    /// Creates a new [`Node<T>`].
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use graph_event::prelude::*;
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

impl<T: ?Sized> Node<T> {
    fn new_by_box(value: Box<T>) -> Self {
        Self {
            counter: Armc::new(0),
            value: Armc::new(value),
            notifications: Armc::new(vec![]),
            observations: Armc::new(vec![]),
        }
    }

    /// Tries to mark the node `other` for notification by this node.
    ///
    /// # Arguments
    ///
    /// * `other` - The node that is going to be notified.
    /// * `event` - The function that will be executed when the notification happens. This function takes as input a reference to the immutable value stored in the node and a mutable reference to the same type.
    ///  The closure must return a boolean value indicating whether the notified node should be considered updated.
    /// * `policy` - The policy that will be used when sending the notification.
    ///
    /// # Returns
    ///
    /// Returns an `Ok` variant if the marking was successful, or an `Err` variant with a [`NodeError::AlreadyExists`] error if the `other` node is already marked for notification by this node.
    ///
    /// # Examples
    ///
    /// ```
    /// # use graph_event::prelude::*;
    /// # let mut node1 = Node::new(1);
    /// # let mut node2 = Node::new(2);
    /// let event = |reference: &i32, mutable: &mut i32| -> bool {
    ///     *mutable += 1;
    ///     true
    /// };
    /// let result = node1.try_mark_for_notification(node2.clone(), event, NotificationPolicy::All);
    /// assert!(result.is_ok());
    /// ```
    ///
    /// ```
    /// # use graph_event::prelude::*;
    /// # let mut node1 = Node::new(1);
    /// # let mut node2 = Node::new(2);
    /// let event = |reference: &i32, mutable: &mut i32| -> bool {
    ///     *mutable += 1;
    ///     true
    /// };
    /// node1.try_mark_for_notification(node2.clone(), event, NotificationPolicy::All);
    /// let result = node1.try_mark_for_notification(node2, event, NotificationPolicy::All);
    /// assert!(result.is_err());
    /// ```
    pub fn try_mark_for_notification(
        &mut self,
        other: Node<T>,
        event: EventMethod<T>,
        policy: NotificationPolicy,
    ) -> Result<(), NodeError> {
        let try_finded = self.notifications.iter().find(|&e| e.mutable == other);
        if let Some(_) = try_finded {
            return Err(NodeError::AlreadyExists);
        } else {
            let notif = Notification::new(self.clone(), other.clone(), event, policy);
            self.notifications.lock().push(notif);
            Ok(())
        }
    }

    /// This method tries to remove a `Node` from the list of notified nodes.
    ///
    /// # Arguments
    ///
    /// * `other` - A `Node` that may be currently in the list of notified nodes.
    ///
    /// # Returns
    ///
    /// This method returns `Ok(())` if the removal was successful, and `Err(NodeError::DoesNotExist)` if the `Node`
    /// is not in the list of notified nodes.
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

    /// Tries to mark `other` node for observation by this node.
    ///
    /// The `event` parameter is a function that will be triggered when the observed node changes its value.
    ///
    /// # Arguments
    ///
    /// * `other` - The node that will be observed.
    /// * `event` - The function that will be executed when the modification happens. This function takes as input a reference to the immutable value stored in the node and a mutable reference to the same type.
    ///  The closure must return a boolean value indicating whether the notified node should be considered updated.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the observation was successfully marked, or `Err(NodeError::AlreadyExists)` if the `other` node was already marked for observation.
    ///
    /// # Example
    ///
    /// ```
    /// # use graph_event::prelude::*;
    ///
    /// let mut n1 = Node::new(1);
    /// let mut n2 = Node::new(2);
    ///
    /// n1.try_mark_for_observation(n2.clone(), |reference, mutable| {
    ///     println!("{} -> {}", reference, mutable);
    ///     true
    /// });
    /// ```
    pub fn try_mark_for_observation(
        &mut self,
        other: Node<T>,
        event: EventMethod<T>,
    ) -> Result<(), NodeError> {
        let try_finded = self.observations.iter().find(|&e| e.mutable == other);
        if let Some(_) = try_finded {
            return Err(NodeError::AlreadyExists);
        } else {
            let obs = Observation::new(other.clone(), self.clone(), event);
            self.observations.lock().push(obs);
            Ok(())
        }
    }

    /// The `try_mark_off_observation` method is used to remove an observation node from the current node.
    /// It takes another node as a parameter and returns a Result indicating success (`Ok`) or failure (`Err`).
    /// In case of success, the node is successfully removed from the observation list.
    /// If the node does not exist in the observation list, the function returns `Err(NodeError::DoesNotExist)`.
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

    /// The `watch_for_updates` method is used to check for updates in the observations associated with the node.
    ///
    /// # Example
    ///
    /// ```
    /// # use graph_event::prelude::*;
    ///
    /// let mut node = Node::new(0);
    /// let mut observed_node = Node::new(10);    /// 
    /// assert_eq!(*observed_node, 10);
    /// let update_fn = |observed_value: &i32, value: &mut i32| {
    ///     *value = 1;
    ///     true
    /// };
    ///
    /// node.try_mark_for_observation(observed_node.clone(), update_fn);
    /// node.watch_for_updates();
    /// observed_node.update(20);
    /// assert_eq!(*node, 0);
    /// node.watch_for_updates();
    /// assert_eq!(*node, 1);
    ///
    /// ```
    pub fn watch_for_updates(&mut self) {
        self.observations
            .lock()
            .iter_mut()
            .for_each(|obs| obs.check());
    }
}

impl<T: PartialEq> Node<T> {
    /// This method updates the value of the Node. It first checks if the new value is different from the current value.
    /// If they are different, it calls the set_and_notify method with the new value, which will update the value and notify
    /// all the events associated with this node.
    pub fn update(&mut self, new_value: T) {
        if new_value != **self.value {
            self.set_and_notify(Box::new(new_value))
        }
    }
}

impl<T: ?Sized> Node<T> {
    /// Sets the and notify of this [`Node<T>`].
    /// The set_and_notify method takes in a new value as a Box, sets the value of the Node to the new value, and then calls the notify_to_neighbors method.
    pub fn set_and_notify(&mut self, new_value: Box<T>) {
        self.set(new_value);
        self.notify_to_neighbors();
    }

    /// The set method simply updates the value of the Node to the new value.
    pub fn set(&mut self, new_value: Box<T>) {
        *self.value.lock() = new_value;
    }

    /// Returns the make notify of this [`Node<T>`].
    /// Sends notifications to all connected nodes with the default notification policy.
    pub fn notify_to_neighbors(&mut self) {
        *self.counter.lock() += 1;
        self.notifications
            .lock()
            .iter_mut()
            .for_each(|notify| notify.send_with_default_policy());
    }

    /// Sends notifications to all connected nodes with a specified notification policy.
    ///
    /// # Parameters
    ///
    /// - `policy`: The notification policy to be used.
    pub fn notify_to_neighbors_with_policy(&mut self, policy: NotificationPolicy) {
        *self.counter.lock() += 1;
        self.notifications
            .lock()
            .iter_mut()
            .for_each(|notify| notify.send(policy));
    }
}

impl<T: ?Sized> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.deref()
    }
}

impl<T: ?Sized> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            counter: self.counter.clone(),
            value: self.value.clone(),
            notifications: self.notifications.clone(),
            observations: self.observations.clone(),
        }
    }
}

impl<T: ?Sized> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: ?Sized> From<Box<T>> for Node<T> {
    fn from(value: Box<T>) -> Self {
        Self::new_by_box(value)
    }
}
