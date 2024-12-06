use std::{
    collections::{BinaryHeap, HashMap},
    iter::zip,
};

struct PopIterator<T> {
    heap: BinaryHeap<T>,
}

impl<T> PopIterator<T> {
    fn new(heap: BinaryHeap<T>) -> Self {
        PopIterator { heap }
    }
}

impl<T> Iterator for PopIterator<T>
where
    T: Ord,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

pub fn part1(input: &str) -> i32 {
    let mut list1 = BinaryHeap::new();
    let mut list2 = BinaryHeap::new();

    input.lines().for_each(|l| {
        let mut iter = l.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        list1.push(iter.next().unwrap());
        list2.push(iter.next().unwrap());
    });

    return zip(PopIterator::new(list1), PopIterator::new(list2))
        .map(|(a, b)| (a - b).abs())
        .sum();
}

pub fn part2(input: &str) -> i32 {
    let mut map: HashMap<i32, (i32, i32)> = HashMap::new();

    input.lines().for_each(|l| {
        let mut iter = l.split_whitespace().map(|s| s.parse().unwrap());

        map.entry(iter.next().unwrap())
            .and_modify(|(l, _)| *l += 1)
            .or_insert((1, 0));

        map.entry(iter.next().unwrap())
            .and_modify(|(_, r)| *r += 1)
            .or_insert((0, 1));
    });

    return map.drain().map(|(x, (y, z))| x * y * z).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 11)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 31)
    }
}
