use itertools::Itertools;
use std::{
    iter::{Chain, Iterator},
    slice::Iter,
};

fn is_safe<I>(iter: I) -> bool
where
    I: Iterator<Item = i8> + Clone,
{
    let mut diffiter = iter.tuple_windows().map(|(a, b)| a - b);

    match diffiter.next().unwrap() {
        x if (x.abs() >= 3) => return false,
        x if (x > 0) => diffiter.all(|x| x > 0 && x <= 3),
        x if (x < 0) => diffiter.all(|x| x < 0 && x >= -3),
        _ => return false,
    }
}

pub fn part1(input: &str) -> i64 {
    return input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<i8>().unwrap()))
        .filter_map(|r| match is_safe(r.clone()) {
            true => Some(()),
            false => None,
        })
        .count()
        .try_into()
        .unwrap();
}

struct TryRemovingEach<'a, T> {
    slice: &'a Vec<T>,
    current: usize,
}

impl<'a, T> TryRemovingEach<'a, T> {
    fn new(slice: &'a Vec<T>) -> Self {
        TryRemovingEach { slice, current: 0 }
    }
}

impl<'a, T> Iterator for TryRemovingEach<'a, T> {
    type Item = Chain<Iter<'a, T>, Iter<'a, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.slice.len() {
            return None;
        }
        let before = self.slice[..self.current].iter();
        let after = self.slice[self.current + 1..].iter();
        let combined = before.chain(after);
        self.current += 1;
        return Some(combined);
    }
}

pub fn part2(input: &str) -> i64 {
    let mut working_space: Vec<i8> = vec![];
    return input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<i8>().unwrap()))
        .filter_map(|r| {
            working_space.clear();
            working_space.extend(r);
            match TryRemovingEach::new(&working_space).any(|newr| is_safe(newr.cloned())) {
                true => Some(()),
                false => None,
            }
        })
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
