use criterion::{criterion_group, criterion_main, Criterion};
use ecstasy::{
    components::{DebugFlag, Name, Position},
    system, system_mut, Engine, Entity,
};

#[system(simple)]
fn noop(_e: Entity, _dt: f32) {}

#[system_mut(simple)]
fn noop_mut(_e: Entity, _dt: f32) {}

#[system(simple)]
fn noop_args(_e: Entity, _dt: f32, _: &DebugFlag, _: &Position) {
    unreachable!()
}

#[system_mut(simple)]
fn noop_mut_args(_e: Entity, _dt: f32, _: &mut DebugFlag, _: &mut Position) {
    unreachable!()
}

fn noop_system(c: &mut Criterion) {
    c.bench_function("running a no-op System on 10k entities", |b| {
        let mut engine = Engine::new().build_par_pass().add(noop).finish();
        for _ in 0..10000 {
            let e = engine.store.new_entity();
            engine.store.set_component(e, Name(format!("{:?}", e)));
        }
        b.iter(|| engine.run_once())
    });

    c.bench_function("running a no-op SystemMut on 10k entities", |b| {
        let mut engine = Engine::new().add_mut_pass(noop_mut);
        for _ in 0..10000 {
            engine.store.new_entity();
        }
        b.iter(|| engine.run_once())
    });

    c.bench_function("running a no-op System with args on 10k entities", |b| {
        let mut engine = Engine::new().build_par_pass().add(noop_args).finish();
        for _ in 0..10000 {
            let e = engine.store.new_entity();
            engine.store.set_component(e, Name(format!("{:?}", e)));
        }
        b.iter(|| engine.run_once())
    });

    c.bench_function("running a no-op SystemMut with args on 10k entities", |b| {
        let mut engine = Engine::new().add_mut_pass(noop_mut_args);
        for _ in 0..10000 {
            engine.store.new_entity();
        }
        b.iter(|| engine.run_once())
    });
}

criterion_group!(benches, noop_system);
criterion_main!(benches);
