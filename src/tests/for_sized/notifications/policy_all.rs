#[test]
fn notification() {
    use crate::{node::Node};

    let mut a = Node::new(5);
    let b = Node::new(10);

    let _ = a.try_mark_for_notification(b.clone(), |a,b| {*b += *a ; true}, crate::util::NotificationPolicy::All);

    a.update(11);
    assert_eq!(*a,11);
    assert_eq!(*b,21);
}