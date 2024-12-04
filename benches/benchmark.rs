use advent_of_code_2024::days::DAYS;
use criterion::{criterion_group, criterion_main, Criterion};

fn day_benches(c: &mut Criterion) {
    DAYS.iter().for_each(|day| {
        c.bench_function(format!("{}_part1", day.name).as_str(), |b| {
            b.iter(|| day.part1());
        });
        c.bench_function(format!("{}_part2", day.name).as_str(), |b| {
            b.iter(|| day.part2());
        });
    });
}

fn all_benches(c: &mut Criterion) {
    c.bench_function("all", |b| {
        b.iter(|| {
            DAYS.iter().for_each(|day| {
                day.part1();
                day.part2();
            });
        });
    });
}

criterion_group!(days, day_benches);
criterion_group!(all, all_benches);
criterion_main!(days, all);
