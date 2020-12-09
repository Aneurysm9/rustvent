use aoc;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

pub fn benchmark_2020(c: &mut Criterion) {
    for day in 1..10 {
        run_benchmark(c, 2020, day);
    }
}

fn run_benchmark(c: &mut Criterion, year: usize, day: usize) {
    let runner = aoc::new(
        &(year.to_string()),
        &(day.to_string()),
        fs::read_to_string(format!("../input/{}/day{}.in", year, day)).expect("Error reading file"),
    )
    .unwrap();
    c.bench_function(&(format!("{} Day {} Part A", year, day)), |b| {
        b.iter(|| runner.run_a());
    });
    c.bench_function(&(format!("{} Day {} Part B", year, day)), |b| {
        b.iter(|| runner.run_b());
    });
}

criterion_group!(benches, benchmark_2020);
criterion_main!(benches);
