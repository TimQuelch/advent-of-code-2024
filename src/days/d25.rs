use itertools::Itertools;

pub fn part1(input: &str) -> i64 {
    let mut locks: Vec<[u8; 5]> = Vec::new();
    let mut keys: Vec<[u8; 5]> = Vec::new();

    for block in input.split("\n\n") {
        let out = if block.chars().next().unwrap() == '#' {
            &mut locks
        } else {
            &mut keys
        };

        out.push([0; 5]);
        let heights = out.last_mut().unwrap();

        for line in block.split('\n') {
            for (i, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
                heights[i] += 1
            }
        }
    }

    let result = locks
        .into_iter()
        .cartesian_product(keys.into_iter())
        .filter(|(lock, key)| lock.iter().zip(key.iter()).all(|(x, y)| (*x + *y) < 8))
        .count();

    result.try_into().unwrap()
}

pub fn part2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 3)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 0)
    }
}
