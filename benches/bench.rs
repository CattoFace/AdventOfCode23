use std::time::Duration;

use aoc23::{
    day10a, day10b, day11a, day11b, day12a, day12b, day13a, day13b, day14a, day14b, day15a, day15b,
    day16a, day16b, day17a, day17b, day18a, day18b, day19a, day19b, day1a, day1b, day20a, day20b,
    day21a, day21b, day2a, day2b, day3a, day3b, day4a, day4b, day5a, day5b, day6a, day6b, day7a,
    day7b, day8a, day8b, day9a, day9b,
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
    // c.bench_function("day7a", |b| b.iter(day7a::solve_day));
    // c.bench_function("day7b", |b| b.iter(day7b::solve_day));
    // c.bench_function("day8a", |b| b.iter(day8a::solve_day));
    // c.bench_function("day8b", |b| b.iter(day8b::solve_day));
    // c.bench_function("day9a", |b| b.iter(day9a::solve_day));
    // c.bench_function("day9b", |b| b.iter(day9b::solve_day));
    // c.bench_function("day10a", |b| b.iter(day10a::solve_day));
    // c.bench_function("day10b", |b| b.iter(day10b::solve_day));
    // c.bench_function("day11a", |b| b.iter(day11a::solve_day));
    // c.bench_function("day11b", |b| b.iter(day11b::solve_day));
    // c.bench_function("day12a", |b| b.iter(day12a::solve_day));
    // c.bench_function("day13b", |b| b.iter(day12b::solve_day));
    // c.bench_function("day13a", |b| b.iter(day13a::solve_day));
    // c.bench_function("day13b", |b| b.iter(day13b::solve_day));
    // c.bench_function("day14a", |b| b.iter(day14a::solve_day));
    // c.bench_function("day14b", |b| b.iter(day14b::solve_day));
    // c.bench_function("day15a", |b| b.iter(day15a::solve_day));
    // c.bench_function("day15b", |b| b.iter(day15b::solve_day));
    // c.bench_function("day16a", |b| b.iter(day16a::solve_day));
    // c.bench_function("day16b", |b| b.iter(day16b::solve_day));
    // c.bench_function("day17a", |b| b.iter(day17a::solve_day));
    // c.bench_function("day17b", |b| b.iter(day17b::solve_day));
    // c.bench_function("day18a", |b| b.iter(day18a::solve_day));
    // c.bench_function("day18b", |b| b.iter(day18b::solve_day));
    // c.bench_function("day19a", |b| b.iter(day19a::solve_day));
    // c.bench_function("day19b", |b| b.iter(day19b::solve_day));
    // c.bench_function("day20a", |b| b.iter(day20a::solve_day));
    // c.bench_function("day20b", |b| b.iter(day20b::solve_day));
    c.bench_function("day21a", |b| b.iter(day21a::solve_day));
    // c.bench_function("day21b", |b| b.iter(day21b::solve_day));
}

criterion_group! {
    name = benches;
    config=Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
}
criterion_main!(benches);
