mod tests;
mod notification;
mod observation;


/// # Welcome to GraphEvent!
/// The **Graph Event** is a Rust crate that is designed to relate states to each other. Each state will be contained in a specific node along with a generic type value. In turn, the nodes connect in two different ways: by notification and observation.
///
///
///
/// # How it works 
/// ## Notifications
/// First, any node needs to mark other nodes, as if saying that it will notify them whenever it updates. If the generic type implements PartialEq, it will check if there has been a change in the value that the node carries. If not, the user system will have to notify the node itself that it has indeed been updated.
///
///     use  crate::{node::Node};
///     let  mut  a  =  Node::new(5);
///     let  b  =  Node::new(10);
///     let _ =  a.try_mark_for_notification(b.clone(), |a,b| {*b  +=  *a ; true}, crate::util::NotificationPolicy::All);
///     a.update(11);
///     assert_eq!(*a,11);
///     assert_eq!(*b,21);
/// ## Notification Policy
/// | Policy | Description | In Code|
/// |--|--|--|
/// | ALL | Notifies all nodes. | NotificationPolicy::All |
/// | Less Updated| Notifies less updated nodes. | NotificationPolicy::LessUpdated|
/// | More Updated| Notifies more updated nodes. | NotificationPolicy::MoreUpdated|
/// | Equally Updated| Notifies us equally updated. | NotificationPolicy::EquallyUpdated|
/// | Differently Updated| Notifies differently updated nodes. | NotificationPolicy::DifferentlyUpdated|
/// | Equally Or Less Updated| Notifies equally or less updated nodes. | NotificationPolicy::Equally Or Less Updated|
/// | Equally Or More Updated| Notifies equally or more updated nodes. | NotificationPolicy::EquallyOrMoreUpdated|
///
///
/// ## Observations 
/// This happens when any node has marked another node and that other node is updated. Then, the relationship/connection between the nodes holds the previous state of the updated node. 
/// > **Note:** However, the **observing** node only observes if it is demanded.
///
///     use  crate::{node::Node};
///     let  mut  a  =  Node::new(5);
///     let  mut  b  =  Node::new(10);
///     let _ =  b.try_mark_for_observation(a.clone(), |a, b| {*b  +=  *a; true });
///     assert_eq!(*a,5);
///     a.update(11);
///     assert_eq!(*a,11);
///     assert_eq!(*b,10);
///     b.watch_for_updates();
///     assert_eq!(*b,21);
///  ## Links:
///  [Doc(1.1.1)](https://docs.rs/graph_event/1.0.1)
/// [crate.io](https://crates.io/crates/graph_event)

pub mod prelude {
    #![allow(unused_imports)]
    pub use crate::{
        node::Node,
        util::{EventMethod, NodeError, NotificationPolicy},
    };
}
pub mod node;
pub mod util;