#[test]
fn observation() {    
    use crate::tests::for_not_sized::{A, B};
    use crate::prelude::*;
    use crate::tests::for_not_sized::Value;

    let mut a = Node::from(Box::new(A(5)) as Box<dyn Value<usize>>);
    let mut b = Node::from(Box::new(B(10)) as Box<dyn Value<usize>>);
    let _ = b.try_mark_for_observation(a.clone(), |a, b| {
        *b.get_mut() += *a.get();
        true
    });

    assert_eq!(*a.get(),5);
    a.set_and_notify(Box::new(A(11)) as Box<dyn Value<usize>>);
    assert_eq!(*a.get(),11);
    assert_eq!(*b.get(),10);
    b.watch_for_updates();
    assert_eq!(*b.get(),21);
}