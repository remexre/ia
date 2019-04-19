use criterion::{criterion_group, criterion_main, Criterion};
use ecs::{Component, ComponentStore};

#[derive(Debug)]
struct ComponentZST;
impl Component for ComponentZST {}

#[derive(Debug)]
struct ComponentWord(usize);
impl Component for ComponentWord {}

fn simple(c: &mut Criterion) {
    c.bench_function("get ZST (present)", |b| {
        let mut cs = ComponentStore::new();
        let e = cs.new_entity();
        cs.set_component(e, ComponentZST);
        b.iter(|| {
            cs.get_component::<ComponentZST>(e);
        })
    });

    c.bench_function("get ZST (not present)", |b| {
        let mut cs = ComponentStore::new();
        let e = cs.new_entity();
        b.iter(|| {
            cs.get_component::<ComponentZST>(e);
        })
    });

    c.bench_function("get word-sized (present)", |b| {
        let mut cs = ComponentStore::new();
        let e = cs.new_entity();
        e.set_component(e, ComponentWord(12345));
        b.iter(|| {
            cs.get_component::<ComponentWord>(e);
        })
    });

    c.bench_function("get word-sized (not present)", |b| {
        let mut cs = ComponentStore::new();
        let e = cs.new_entity();
        b.iter(|| {
            cs.get_component::<ComponentWord>(e);
        })
    });

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

criterion_group!(benches, simple);
criterion_main!(benches);
