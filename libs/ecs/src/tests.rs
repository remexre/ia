use crate::{
    components::{Name, Position},
    Component, ComponentStore,
};
use cgmath::Point3;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn create() {
    drop(ComponentStore::new());
}

#[test]
fn simple() {
    let mut store = ComponentStore::new();

    let foo = store.new_entity();
    let bar = store.new_entity();

    store.set_component(foo, Name("Foo".to_string()));
    store.set_component(foo, Position::new(1.0, 2.0, 3.0));
    store.set_component(bar, Name("Bar".to_string()));
    store.set_component(bar, Position::new(1.0, 2.0, 3.0));

    store.remove_component::<Position>(foo);

    assert_eq!(
        store.get_component::<Name>(foo).map(|n| -> &str { &n.0 }),
        Some("Foo")
    );
    assert_eq!(store.get_component::<Position>(foo).map(|p| p.0), None);

    assert_eq!(
        store.get_component::<Name>(bar).map(|n| -> &str { &n.0 }),
        Some("Bar")
    );
    assert_eq!(
        store.get_component::<Position>(bar).map(|p| p.0),
        Some(Point3::new(1.0, 2.0, 3.0))
    );
}

#[test]
fn dropping() {
    lazy_static! {
        static ref N: AtomicUsize = AtomicUsize::new(0);
    }

    #[derive(Debug)]
    struct C;
    impl C {
        fn new() -> C {
            let _ = N.fetch_add(1, Ordering::SeqCst);
            C
        }
    }
    impl Component for C {}
    impl Drop for C {
        fn drop(&mut self) {
            let _ = N.fetch_sub(1, Ordering::SeqCst);
        }
    }

    let mut store = ComponentStore::new();
    let foo = store.new_entity();
    let bar = store.new_entity();

    store.set_component(foo, C::new());
    assert_eq!(N.load(Ordering::SeqCst), 1);

    store.set_component(bar, C::new());
    assert_eq!(N.load(Ordering::SeqCst), 2);

    store.set_component(foo, C::new());
    assert_eq!(N.load(Ordering::SeqCst), 2);

    store.remove_component::<C>(bar);
    assert_eq!(N.load(Ordering::SeqCst), 1);

    drop(store);
    assert_eq!(N.load(Ordering::SeqCst), 0);
}
