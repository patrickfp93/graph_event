#[test]
fn observation() {
    use crate::{node::Node, util::Condition};

    let mut a = Node::new(5);
    let mut b = Node::new(10);
    let _ = b.try_mark_for_observation(a.clone(), |a, b| *b += *a , Condition::Always);

    assert_eq!(*a,5);
    a.update(11);
    assert_eq!(*a,11);
    assert_eq!(*b,10);
    b.watch_for_updates();
    assert_eq!(*b,21);
}