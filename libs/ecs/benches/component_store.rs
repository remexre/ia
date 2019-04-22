use criterion::{criterion_group, criterion_main, Criterion};
use ecs::{Component, ComponentStore, Entity};
use rand::random;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ComponentZST;
impl Component for ComponentZST {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ComponentWord(usize);
impl Component for ComponentWord {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ComponentStr(&'static str);
impl Component for ComponentStr {}

fn set_up_component_store<T: Component + Copy>(
    component: T,
    present: bool,
) -> (ComponentStore, Entity) {
    // This stymies both optimizations where component store memory is lazily allocated and where
    // it behaves differently with very few entities.

    let mut cs = ComponentStore::new();
    for _ in 0..5000 {
        let e = cs.new_entity();
        if random() {
            cs.set_component(e, component);
        }
    }
    let e = cs.new_entity();
    if present {
        cs.set_component(e, component);
    }
    for _ in 0..5000 {
        let e = cs.new_entity();
        if random() {
            cs.set_component(e, component);
        }
    }

    (cs, e)
}

fn bench_getter<T: Component + Copy + PartialEq>(
    c: &mut Criterion,
    component: T,
    name: &str,
    present: bool,
) {
    c.bench_function(name, move |b| {
        let (cs, e) = set_up_component_store(component, present);
        if present {
            b.iter(|| assert_eq!(cs.get_component(e), Some(&component)))
        } else {
            b.iter(|| assert_eq!(cs.get_component::<T>(e), None))
        }
    });
}

fn bench_getters<T: Component + Copy + PartialEq>(c: &mut Criterion, component: T, name: &str) {
    bench_getter(c, component, &format!("get {} (present)", name), true);
    bench_getter(c, component, &format!("get {} (not present)", name), false);
}

fn component_store(c: &mut Criterion) {
    bench_getters(c, ComponentZST, "ZST");
    bench_getters(c, ComponentWord(12345), "word-sized");
    bench_getters(
        c,
        ComponentStr("12345"),
        "word-sized with null pointer optimization",
    );

    c.bench_function("get not-present, set, get present; word-sized", |b| {
        let mut cs = ComponentStore::new();
        b.iter(|| {
            let e = cs.new_entity();
            assert_eq!(cs.get_component::<ComponentWord>(e), None);
            cs.set_component::<ComponentWord>(e, ComponentWord(12345));
            assert_eq!(
                cs.get_component::<ComponentWord>(e),
                Some(&ComponentWord(12345))
            );
        })
    });
}

criterion_group!(benches, component_store);
criterion_main!(benches);
