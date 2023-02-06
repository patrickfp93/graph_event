


#[test]
fn notification() {
    use crate::tests::for_not_sized::{Value, A,B};
    use crate::prelude::*;

    let mut a = Node::from(Box::new(A(5)) as Box<dyn Value<usize>>);
    let b = Node::from(Box::new(B(10)) as Box<dyn Value<usize>>);

    let _ = a.try_mark_for_notification(b.clone(), |a,b| {
        *b.get_mut() += a.get();
        true
    } , NotificationPolicy::All);

    a.set_and_notify(Box::new(A(11)) as Box<dyn Value<usize>>);
    assert_eq!(*a.get(),11);
    assert_eq!(*b.get(),21);
}