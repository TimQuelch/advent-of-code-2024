use cached::{proc_macro::cached, Cached};

#[cached]
fn count_stones(s: u64, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    if s == 0 {
        return count_stones(1, depth - 1);
    }

    // Calculate ceil rounded log 10 to get the number of digits. Add 0.1 to ensure that exact
    // powers of 10 round up to the next one.
    let log_s = (s as f64 + 0.1).log10().ceil() as u32;
    if log_s != 0 && log_s % 2 == 0 {
        let split_factor = 10_u64.pow(log_s / 2);
        return count_stones(s / split_factor, depth - 1)
            + count_stones(s % split_factor, depth - 1);
    }

    return count_stones(s * 2024, depth - 1);
}

fn solve(input: &str, depth: u8) -> i64 {
    // Clear the cache each time so we have fair timings
    COUNT_STONES.lock().unwrap().cache_clear();

    let res = input
        .split_whitespace()
        .map(|s| count_stones(s.parse().unwrap(), depth))
        .sum::<u64>()
        .try_into()
        .unwrap();

    res
}

pub fn part1(input: &str) -> i64 {
    solve(input, 25)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
125 17
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 55312)
    }
}
