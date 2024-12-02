use std::fs;

use advent_of_code_2024::*;

fn main() {
    let data01 = fs::read_to_string("./data/d01.txt").unwrap();
    let data02 = fs::read_to_string("./data/d02.txt").unwrap();

    let p01_1 = d01::part1(&data01);
    let p01_2 = d01::part2(&data01);
    let p02_1 = d02::part1(&data02);
    let p02_2 = d02::part2(&data02);
    println!("{}, {}", p01_1, p01_2);
    println!("{}, {}", p02_1, p02_2);
}
