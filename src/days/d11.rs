use std::{cell::RefCell, collections::HashMap};

fn trim_zeros(s: &str) -> &str {
    match s.trim_start_matches('0') {
        "" => "0",
        trimmed => trimmed,
    }
}

thread_local! {
    static COUNT_STONES_LOOKUP: RefCell<HashMap<(u32, u8), u32>> = RefCell::new(HashMap::new());
}

fn lookup_cache(s: u32, d: u8) -> Option<u32> {
    COUNT_STONES_LOOKUP.with_borrow(|lookup| lookup.get(&(s, d)).map(|c| *c))
}

fn set_cache(s: u32, d: u8, result: u32) {
    COUNT_STONES_LOOKUP.with_borrow_mut(|lookup| lookup.insert((s, d), result));
}

fn clear_cache() {
    COUNT_STONES_LOOKUP.with_borrow_mut(|lookup| lookup.clear());
}

fn count_stones(s: &str, depth: u8) -> u32 {
    let parsed = s.parse().unwrap();
    if let Some(cached) = lookup_cache(parsed, depth) {
        return cached;
    }

    if depth == 0 {
        return 1;
    }
    let result = match s {
        "0" => count_stones("1", depth - 1),
        x if x.len() % 2 == 0 => {
            let (a, b) = s.split_at(s.len() / 2);
            count_stones(a, depth - 1) + count_stones(trim_zeros(b), depth - 1)
        }
        _ => count_stones(&(parsed * 2024).to_string(), depth - 1),
    };

    set_cache(parsed, depth, result);

    result
}

fn solve(input: &str, depth: u8) -> i64 {
    clear_cache();

    let res = input
        .split_whitespace()
        .map(|s| count_stones(s, depth))
        .sum::<u32>()
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
