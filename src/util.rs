
pub type EventMethod<T> = fn(reference: &T, mutable: &mut T) -> bool;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NotificationPolicy {
    All,
    LessUpdated,
    MoreUpdated,
    EquallyUpdated,
    DifferentlyUpdated,
    EquallyOrLessUpdated,
    EquallyOrMoreUpdated
}

impl NotificationPolicy{
    pub fn should_notify(&self,node_state_reference : u64,node_state_mutable : u64) -> bool{
        match self {
            NotificationPolicy::All => true,
            NotificationPolicy::LessUpdated => node_state_mutable < node_state_reference,
            NotificationPolicy::MoreUpdated => node_state_mutable > node_state_reference,
            NotificationPolicy::EquallyUpdated => node_state_mutable == node_state_reference,
            NotificationPolicy::DifferentlyUpdated => node_state_mutable != node_state_reference,
            NotificationPolicy::EquallyOrLessUpdated => node_state_mutable <= node_state_reference,
            NotificationPolicy::EquallyOrMoreUpdated => node_state_mutable >= node_state_reference,
        }
    }
}

pub enum NodeError {
    AlreadyExists,
    DoesNotExist,
}

impl From<NodeError> for String {
    fn from(error: NodeError) -> Self {
        match error {
            NodeError::AlreadyExists => "Already exists!".into(),
            NodeError::DoesNotExist => "Does not exist!".into(),
        }
    }
}


