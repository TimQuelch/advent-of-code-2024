use cached::{proc_macro::cached, stores::UnboundCache, Cached};

fn parse(input: &str) -> (Vec<&str>, impl Iterator<Item = &str>) {
    let (towels_in, patterns_in) = input.split_once("\n\n").unwrap();

    let towels: Vec<_> = towels_in.split(", ").collect();

    let patterns = patterns_in.lines();

    (towels, patterns)
}

#[cached(
    ty = "UnboundCache<String, bool>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ pattern.to_string() }"#
)]
fn is_possible(towels: &Vec<&str>, pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }
    towels
        .iter()
        .any(|towel| match pattern.strip_prefix(towel) {
            None => false,
            Some(new) => is_possible(&towels, new),
        })
}

pub fn part1(input: &str) -> i64 {
    let (towels, patterns) = parse(input);

    // Clear the cache each time so we have fair timings
    IS_POSSIBLE.lock().unwrap().cache_clear();

    let result = patterns
        .filter(|&pattern| is_possible(&towels, pattern))
        .count();

    result.try_into().unwrap()
}

#[cached(
    ty = "UnboundCache<String, u64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ pattern.to_string() }"#
)]
fn count_ways(towels: &Vec<&str>, pattern: &str) -> u64 {
    if pattern.is_empty() {
        return 1;
    }
    towels
        .iter()
        .map(|towel| match pattern.strip_prefix(towel) {
            None => 0,
            Some(new) => count_ways(&towels, new),
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let (towels, patterns) = parse(input);

    // Clear the cache each time so we have fair timings
    COUNT_WAYS.lock().unwrap().cache_clear();

    let result = patterns
        .map(|pattern| count_ways(&towels, pattern))
        .sum::<u64>();

    result.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 6)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 16)
    }
}
