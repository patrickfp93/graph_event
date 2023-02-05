
#[test]
fn notification() {
    use crate::{node::Node, util::Condition};

    let mut a = Node::new(5);
    let b = Node::new(10);

    let _ = a.try_mark_for_notification(b.clone(), |a,b| *b += *a , Condition::Always);

    a.update(11);
    assert_eq!(*a,11);
    assert_eq!(*b,21);
}