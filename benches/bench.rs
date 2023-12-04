use aoc23::{day1a, day1b, day2a, day2b, day3a, day3b};
use criterion::{criterion_group, criterion_main, Criterion};
fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("day1a", |b| b.iter(day1a::solve_day));
    // c.bench_function("day1b", |b| b.iter(day1b::solve_day));
    // c.bench_function("day2a", |b| b.iter(day2a::solve_day));
    // c.bench_function("day2b", |b| b.iter(day2b::solve_day));
    c.bench_function("day3a", |b| b.iter(day3a::solve_day));
    c.bench_function("day3b", |b| b.iter(day3b::solve_day));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
