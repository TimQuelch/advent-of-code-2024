use std::{
    collections::{BinaryHeap, HashMap},
    iter::zip,
};

pub fn part1(input: &str) -> i32 {
    let mut lists: [BinaryHeap<i32>; 2] = [(); 2].map(|_| BinaryHeap::new());

    input.lines().for_each(|l| {
        l.split_whitespace()
            .map(|s| s.parse())
            .enumerate()
            .for_each(|(i, val)| lists[i].push(val.unwrap()))
    });

    assert!(lists[0].len() == lists[1].len());

    let sorted = lists.map(|l| l.into_sorted_vec());

    return zip(sorted[0].iter(), sorted[1].iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
}

pub fn part2(input: &str) -> i32 {
    let mut map: HashMap<i32, (i32, i32)> = HashMap::new();

    input.lines().for_each(|l| {
        let vals: Vec<i32> = l.split_whitespace().map(|s| s.parse().unwrap()).collect();

        assert!(vals.len() == 2);

        map.entry(vals[0])
            .and_modify(|(l, _)| *l += 1)
            .or_insert((1, 0));
        map.entry(vals[1])
            .and_modify(|(_, r)| *r += 1)
            .or_insert((0, 1));
    });

    return map
        .drain()
        .map(|(x, (y, z))| x * y * z)
        .fold(0, |acc, x| acc + x);
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
