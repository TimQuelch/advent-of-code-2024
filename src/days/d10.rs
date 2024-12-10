use itertools::Itertools;
use ndarray::Array2;

type Pos = (usize, usize);

fn build_grid(input: &str) -> Array2<u8> {
    let ncols = input.lines().next().unwrap().len();
    let nrows = ((input.len() + 1) / ncols) - 1;

    Array2::from_shape_vec(
        (nrows, ncols),
        input
            .bytes()
            .filter_map(|x| match x {
                x if x >= b'0' && x <= b'9' => Some(x - b'0'),
                _ => None,
            })
            .collect(),
    )
    .unwrap()
}

fn neighbours(grid: &Array2<u8>, p: Pos) -> Vec<Pos> {
    let x = grid.get(p).unwrap();
    [
        p.0.checked_sub(1).map(|i| (i, p.1)),
        p.0.checked_add(1).map(|i| (i, p.1)),
        p.1.checked_sub(1).map(|j| (p.0, j)),
        p.1.checked_add(1).map(|j| (p.0, j)),
    ]
    .into_iter()
    .filter_map(move |p| p.filter(|p| grid.get(*p).map_or(false, |y| *y == (x + 1))))
    .collect_vec()
}

fn score_trail(grid: &Array2<u8>, start: Pos, stack: &mut Vec<Pos>, ends: &mut Vec<Pos>) -> u32 {
    ends.clear();
    stack.clear();
    stack.push(start);

    while let Some(p) = stack.pop() {
        if *grid.get(p).unwrap() == 9 && !ends.contains(&p) {
            ends.push(p)
        }
        stack.extend(neighbours(grid, p));
    }

    ends.len().try_into().unwrap()
}

fn rate_trail(grid: &Array2<u8>, start: Pos, stack: &mut Vec<Pos>, _ends: &mut Vec<Pos>) -> u32 {
    let mut rating = 0;

    stack.clear();
    stack.push(start);

    while let Some(p) = stack.pop() {
        if *grid.get(p).unwrap() == 9 {
            rating += 1
        }
        stack.extend(neighbours(grid, p));
    }

    rating
}

pub fn solve(
    input: &str,
    score_fn: impl Fn(&Array2<u8>, Pos, &mut Vec<Pos>, &mut Vec<Pos>) -> u32,
) -> i64 {
    let grid = build_grid(input);

    let mut stack = vec![];
    let mut ends = vec![];

    grid.indexed_iter()
        .filter_map(|(i, x)| if *x == 0 { Some(i) } else { None })
        .map(|p| score_fn(&grid, p, &mut stack, &mut ends))
        .sum::<u32>()
        .try_into()
        .unwrap()
}

pub fn part1(input: &str) -> i64 {
    solve(input, score_trail)
}

pub fn part2(input: &str) -> i64 {
    solve(input, rate_trail)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    const EXAMPLE2: &str = "
1110111
1111111
1112111
6543456
7111117
8111118
9111119
";

    #[test]
    fn example2_part1() {
        let result = part1(EXAMPLE2.trim());
        assert_eq!(result, 2)
    }

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 36)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 81)
    }
}
