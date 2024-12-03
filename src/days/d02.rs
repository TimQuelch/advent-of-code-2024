use itertools::Itertools;
use std::iter::{Chain, Iterator, Skip, Take};

fn is_safe<I>(iter: I) -> bool
where
    I: Iterator<Item = i32> + Clone,
{
    let diff: Vec<_> = iter.tuple_windows().map(|(a, b)| a - b).collect();

    (diff.iter().all(|x| *x > 0) || diff.iter().all(|x| *x < 0))
        && diff.iter().all(|x| x.abs() <= 3)
}

pub fn part1(input: &str) -> i32 {
    return input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()))
        .filter(|r| is_safe(r.clone()))
        .count()
        .try_into()
        .unwrap();
}

struct TryRemovingEach<T>
where
    T: Iterator + Clone,
{
    iter: T,
    current: usize,
}

impl<T> TryRemovingEach<T>
where
    T: Iterator + Clone,
{
    fn new(iter: T) -> Self {
        TryRemovingEach { iter, current: 0 }
    }
}

impl<T> Iterator for TryRemovingEach<T>
where
    T: Iterator + Clone,
{
    type Item = Chain<Take<T>, Skip<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.clone().nth(self.current).is_none() {
            return None;
        }
        let before = self.iter.clone().take(self.current);
        let after = self.iter.clone().skip(self.current + 1);
        let combined = before.chain(after);
        self.current += 1;
        return Some(combined);
    }
}

pub fn part2(input: &str) -> i32 {
    return input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()))
        .filter(|r| TryRemovingEach::new(r.clone()).any(|newr| is_safe(newr)))
        .count()
        .try_into()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 2)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 4)
    }
}
