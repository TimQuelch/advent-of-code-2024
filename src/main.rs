use std::time::{Duration, Instant};

use advent_of_code_2024::days::{Day, DAYS};

fn format_duration(d: Duration) -> String {
    let nanos = d.as_nanos();

    if nanos < 1000 {
        format!("{}ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.3}µs", nanos as f64 / 1000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.3}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.3}s", nanos as f64 / 1_000_000_000.0)
    }
}

pub fn time_execution<F, T>(f: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

fn run_days(days: Vec<&Day>) {
    let (results, total) = time_execution(|| {
        days.into_iter()
            .map(|day| {
                let (part1, duration1) = time_execution(|| day.part1());
                let (part2, duration2) = time_execution(|| day.part2());
                return (day.name.as_str(), part1, part2, duration1, duration2);
            })
            .collect::<Vec<_>>()
    });
    results
        .into_iter()
        .for_each(|(name, part1, part2, duration1, duration2)| {
            println!(
                "{}: {}, {} ({}, {})",
                name,
                part1,
                part2,
                format_duration(duration1),
                format_duration(duration2)
            );
        });
    println!("Total time: {}", format_duration(total));
}

fn main() {
    let args: Vec<_> = std::env::args()
        .skip(1)
        .map(|s| {
            s.parse::<i32>()
                .map_or_else(|_| s, |n| format!("d{:02}", n))
        })
        .collect();

    let days: Vec<_> = match args.len() {
        0 => DAYS.iter().collect(),
        _ => DAYS.iter().filter(|day| args.contains(&day.name)).collect(),
    };

    run_days(days);
}
