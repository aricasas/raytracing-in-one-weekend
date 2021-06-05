use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracing::oscillate;

fn bench_oscillate(c: &mut Criterion) {
    c.bench_function("oscillate 5.0", |b| {
        b.iter(|| oscillate(black_box(5.0), std::f64::consts::PI / 10.0))
    });
    c.bench_function("oscillate -5.0", |b| {
        b.iter(|| oscillate(black_box(-5.0), std::f64::consts::PI / 10.0))
    });
    c.bench_function("oscillate 0.0", |b| {
        b.iter(|| oscillate(black_box(0.0), std::f64::consts::PI / 10.0))
    });
    c.bench_function("oscillate -0.0", |b| {
        b.iter(|| oscillate(black_box(-0.0), std::f64::consts::PI / 10.0))
    });
    c.bench_function("oscillate 5000.0", |b| {
        b.iter(|| oscillate(black_box(5000.0), std::f64::consts::PI / 10.0))
    });
    c.bench_function("oscillate -5000.0", |b| {
        b.iter(|| oscillate(black_box(-5000.0), std::f64::consts::PI / 10.0))
    });
}
fn bench_sin(c: &mut Criterion) {
    c.bench_function("sin 5.0", |b| b.iter(|| f64::sin(black_box(5.0 * 10.0))));
    c.bench_function("sin -5.0", |b| b.iter(|| f64::sin(black_box(-5.0 * 10.0))));
    c.bench_function("sin 0.0", |b| b.iter(|| f64::sin(black_box(0.0 * 10.0))));
    c.bench_function("sin -0.0", |b| b.iter(|| f64::sin(black_box(-0.0 * 10.0))));
    c.bench_function("sin 5000.0", |b| {
        b.iter(|| f64::sin(black_box(5000.0 * 10.0)))
    });
    c.bench_function("sin -5000.0", |b| {
        b.iter(|| f64::sin(black_box(-5000.0 * 10.0)))
    });
}

criterion_group!(benches, bench_oscillate, bench_sin);
criterion_main!(benches);
