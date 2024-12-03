use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code_2024::days::DAYS;

fn benchmark(c: &mut Criterion) {
    DAYS.iter().enumerate().for_each(|(i, day)| {
        c.bench_function(format!("day_{}_part_1", i).as_str(), |b| {
            b.iter(|| day.part1());
        });
        c.bench_function(format!("day_{}_part_2", i).as_str(), |b| {
            b.iter(|| day.part2());
        });
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
