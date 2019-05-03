use criterion::{criterion_group, criterion_main, Criterion};
use iqm::IQM;
use std::fs::read;

fn kick(c: &mut Criterion) {
    c.bench_function("loading actors/player/kick.iqm", |b| {
        let kick_iqm = read("../../assets/actors/player/kick.iqm").unwrap();
        b.iter(|| IQM::parse_from(&kick_iqm).unwrap())
    });

    c.bench_function("loading actors/player/kick.iqm (incl. IO)", |b| {
        b.iter(|| {
            let kick_iqm = read("../../assets/actors/player/kick.iqm").unwrap();
            IQM::parse_from(&kick_iqm).unwrap()
        })
    });
}

criterion_group!(benches, kick);
criterion_main!(benches);
