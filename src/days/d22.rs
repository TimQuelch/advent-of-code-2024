use std::collections::{HashMap, HashSet};
use std::iter;

const K: u64 = 16777216;

fn next_secret(mut n: u64) -> u64 {
    n = ((n * 64) ^ n) % K;
    n = ((n / 32) ^ n) % K;
    n = ((n * 2048) ^ n) % K;
    n
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .map(|n| (0..2000).fold(n, |acc, _| next_secret(acc)))
        .sum::<u64>()
        .try_into()
        .unwrap()
}

fn seq_to_key(a: i8, b: i8, c: i8, d: i8) -> u32 {
    let au = (a + 9) as u32;
    let bu = ((b + 9) as u32) << 5;
    let cu = ((c + 9) as u32) << 10;
    let du = ((d + 9) as u32) << 15;
    au | bu | cu | du
}

pub fn part2(input: &str) -> i64 {
    let mut patterns = HashMap::with_capacity(50_000);
    let mut nums = Vec::with_capacity(2000);
    let mut locally_found = HashSet::with_capacity(4000);

    input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .for_each(|n| {
            nums.clear();
            locally_found.clear();
            nums.extend(
                iter::successors(Some((n, 0)), |(n, _)| {
                    let next = next_secret(*n);
                    let diff = (next % 10) as i8 - (*n % 10) as i8;
                    Some((next, diff))
                })
                .take(2000),
            );

            for w in nums.windows(4) {
                let pattern = seq_to_key(w[0].1, w[1].1, w[2].1, w[3].1);

                if locally_found.contains(&pattern) {
                    continue;
                }

                let val = (w[3].0 % 10) as u16;

                patterns
                    .entry(pattern)
                    .and_modify(|e| *e += val)
                    .or_insert(val);

                locally_found.insert(pattern);
            }
        });

    patterns.into_values().max().unwrap().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
1
10
100
2024
";

    const EXAMPLE2: &str = "
1
2
3
2024
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 37327623)
    }

    #[test]
    fn example_part1_1() {
        let result = part1("1");
        assert_eq!(result, 8685429)
    }

    #[test]
    fn example_part1_2() {
        let result = part1("10");
        assert_eq!(result, 4700978)
    }

    #[test]
    fn example_part1_3() {
        let result = part1("100");
        assert_eq!(result, 15273692)
    }

    #[test]
    fn example_part1_4() {
        let result = part1("2024");
        assert_eq!(result, 8667524)
    }

    #[test]
    fn next_secret_1() {
        let result = next_secret(123);
        assert_eq!(result, 15887950)
    }

    #[test]
    fn next_secret_2() {
        let result = next_secret(15887950);
        assert_eq!(result, 16495136)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE2.trim());
        assert_eq!(result, 23)
    }
}
