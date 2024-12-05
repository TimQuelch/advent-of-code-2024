use std::fs;

use advent_of_code_2024::days::DAYS;
use criterion::{criterion_group, criterion_main, Criterion};

fn day_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("days");
    DAYS.iter().for_each(|day| {
        group.bench_function(format!("{}_part1", day.name).as_str(), |b| {
            b.iter(|| day.part1());
        });
        group.bench_function(format!("{}_part2", day.name).as_str(), |b| {
            b.iter(|| day.part2());
        });
    });
    group.finish();
}

fn d05_benches(c: &mut Criterion) {
    let mut d05 = c.benchmark_group("d05");

    use advent_of_code_2024::days::d05;
    let input = fs::read_to_string("./data/d05.txt").unwrap();
    d05.bench_function("part1", |b| b.iter(|| d05::part1(input.as_str())));
    d05.bench_function("part2", |b| b.iter(|| d05::part2(input.as_str())));
    d05.bench_function("old_part1", |b| b.iter(|| d05::old::part1(input.as_str())));
    d05.bench_function("old_part2", |b| b.iter(|| d05::old::part2(input.as_str())));

    d05.finish()
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
criterion_group!(d05, d05_benches);
criterion_group! {
    name = all;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(15));
    targets = all_benches
}
criterion_main!(days, d05, all);
