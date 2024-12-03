use once_cell::sync::Lazy;
use regex::Regex;

pub fn part1(input: &str) -> i32 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
    return RE
        .captures_iter(input)
        .map(|m| {
            m.iter()
                .skip(1)
                .map(|d| d.unwrap().as_str().parse::<i32>().unwrap())
                .product::<i32>()
        })
        .sum();
}

pub fn part2(input: &str) -> i32 {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"don't\(\)|do\(\)|mul\((\d+),(\d+)\)").unwrap());

    RE.captures_iter(input)
        .scan(true, |enabled, m| match m.get(0).unwrap().as_str() {
            // don't
            s if s.starts_with("don") => {
                *enabled = false;
                Some(None)
            }
            // do
            s if s.starts_with("d") => {
                *enabled = true;
                Some(None)
            }
            // mul
            _ if *enabled => Some(Some(m)),
            _ => Some(None),
        })
        .filter_map(|x| x)
        .map(|m| {
            m.iter()
                .skip(1)
                .map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
                .product::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        println!("{}", result);
        println!("{}", result);
        println!("{}", result);
        println!("{}", result);
        assert_eq!(result, 161)
    }

    const EXAMPLE2: &str = "
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE2.trim());
        assert_eq!(result, 48)
    }
}
