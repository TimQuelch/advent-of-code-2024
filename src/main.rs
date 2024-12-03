use advent_of_code_2024::days::{Day, DAYS};

fn run_day(day: &Day) {
    println!("{}, {}", day.part1(), day.part2());
}

fn main() {
    DAYS.iter().for_each(|day| run_day(day));
}
