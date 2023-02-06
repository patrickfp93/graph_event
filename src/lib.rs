mod tests;

pub mod node;
mod notification;
mod observation;
pub mod util;

pub mod prelude {
    #![allow(unused_imports)]
    pub use crate::{
        node::Node,
        util::{EventMethod, NodeError, NotificationPolicy},
    };
}
