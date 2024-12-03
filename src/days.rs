use std::fs;

use once_cell::sync::Lazy;

type PartFn = fn(&str) -> i32;

pub struct Day {
    input: String,
    part1_impl: PartFn,
    part2_impl: PartFn,
}

impl Day {
    fn new(filename: &str, part1: PartFn, part2: PartFn) -> Self {
        Day {
            input: fs::read_to_string(filename).unwrap(),
            part1_impl: part1,
            part2_impl: part2,
        }
    }

    pub fn part1(&self) -> i32 {
        (self.part1_impl)(&self.input)
    }

    pub fn part2(&self) -> i32 {
        (self.part2_impl)(&self.input)
    }
}

macro_rules! declare_modules {
    ($($day:ident),*) => {
        $(
            mod $day;
        )*
    };
}

macro_rules! make_days {
    ($($day:ident),*) => {
        {
            vec![
                $(
                    Day::new(concat!("data/", stringify!($day), ".txt"), $day::part1, $day::part2),
                )*
            ]
        }
    };
}

declare_modules!(d01, d02, d03);
pub static DAYS: Lazy<Vec<Day>> = Lazy::new(|| make_days!(d01, d02, d03));
