use aoc23::{
    day1a, day1b, day2a, day2b, day3a, day3b, day4a, day4b, day5a, day5b, day6a, day6b, day7a,
    day7b,
};
use criterion::{criterion_group, criterion_main, Criterion};
fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("day1a", |b| b.iter(day1a::solve_day));
    // c.bench_function("day1b", |b| b.iter(day1b::solve_day));
    // c.bench_function("day2a", |b| b.iter(day2a::solve_day));
    // c.bench_function("day2b", |b| b.iter(day2b::solve_day));
    // c.bench_function("day3a", |b| b.iter(day3a::solve_day));
    // c.bench_function("day3b", |b| b.iter(day3b::solve_day));
    // c.bench_function("day4a", |b| b.iter(day4a::solve_day));
    // c.bench_function("day4b", |b| b.iter(day4b::solve_day));
    // c.bench_function("day5a", |b| b.iter(day5a::solve_day));
    // c.bench_function("day5b", |b| b.iter(day5b::solve_day));
    // c.bench_function("day6a", |b| b.iter(day6a::solve_day));
    // c.bench_function("day6b", |b| b.iter(day6b::solve_day));
    c.bench_function("day7a", |b| b.iter(day7a::solve_day));
    c.bench_function("day7b", |b| b.iter(day7b::solve_day));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
