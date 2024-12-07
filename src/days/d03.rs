use once_cell::sync::Lazy;
use regex::Regex;

pub fn part1(input: &str) -> i64 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
    return RE
        .captures_iter(input)
        .map(|m| {
            m.iter()
                .skip(1)
                .map(|d| d.unwrap().as_str().parse::<i32>().unwrap())
                .product::<i32>()
        })
        .sum::<i32>()
        .try_into()
        .unwrap();
}

struct EnabledMulIterator<'a> {
    input: &'a str,
    offset: usize,
}

impl<'a> EnabledMulIterator<'a> {
    fn new(input: &'a str) -> Self {
        EnabledMulIterator { input, offset: 0 }
    }
}

impl Iterator for EnabledMulIterator<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        static MUL_OR_DONT_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"don't\(\)|mul\((\d+),(\d+)\)").unwrap());
        static DO_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"do\(\)").unwrap());

        loop {
            match MUL_OR_DONT_RE.captures_at(self.input, self.offset) {
                None => return None,
                Some(x) if x.get(0).unwrap().as_str().starts_with("m") => {
                    self.offset = x.get(0).unwrap().end();
                    return Some((
                        x.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                        x.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    ));
                }
                Some(x) => match DO_RE.find_at(self.input, x.get(0).unwrap().end()) {
                    None => return None,
                    Some(next_do) => {
                        self.offset = next_do.end();
                    }
                },
            }
        }
    }
}

pub fn part2(input: &str) -> i64 {
    EnabledMulIterator::new(input)
        .map(|(a, b)| a * b)
        .sum::<i32>()
        .try_into()
        .unwrap()
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
