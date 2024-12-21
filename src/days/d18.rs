use std::collections::HashSet;

use super::utils;

fn neighbours(
    p: (usize, usize),
    size: (usize, usize),
    walls: &HashSet<(usize, usize)>,
) -> impl Iterator<Item = (usize, usize)> + use<'_> {
    [
        p.0.checked_sub(1).map(|i| (i, p.1)),
        Some(p.0 + 1).filter(|&i| i < size.0).map(|i| (i, p.1)),
        p.1.checked_sub(1).map(|j| (p.0, j)),
        Some(p.1 + 1).filter(|&j| j < size.1).map(|j| (p.0, j)),
    ]
    .into_iter()
    .filter_map(|p| match p {
        None => None,
        Some(p) if walls.contains(&p) => None,
        Some(p) => Some(p),
    })
}

fn build_walls(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|x| {
            let (a, b) = x.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn solve_p1(input: &str, size: (usize, usize), n: usize) -> i64 {
    let all_walls = build_walls(input);
    let walls = all_walls[..n].iter().cloned().collect::<HashSet<_>>();
    let s = (0, 0);
    let e = (size.0 - 1, size.1 - 1);

    let mut working_space = utils::AstarWorkingSpace::new();

    let result = utils::astar(
        s,
        e,
        |p| neighbours(p, size, &walls).map(|n| (n, 1)),
        |p| {
            ((e.0 as i64 - p.0 as i64).unsigned_abs() + (e.1 as i64 - p.1 as i64).unsigned_abs())
                as usize
        },
        &mut working_space,
    )
    .unwrap();

    result.try_into().unwrap()
}

fn solve_p2(input: &str, size: (usize, usize)) -> (usize, usize) {
    let all_walls = build_walls(input);
    let s = (0, 0);
    let e = (size.0 - 1, size.1 - 1);

    let mut working_space = utils::AstarWorkingSpace::new();

    let mut walls = HashSet::new();

    let byte_that_breaks = (1..all_walls.len())
        .collect::<Vec<_>>()
        .partition_point(|&i| {
            if walls.len() > i {
                walls.clear();
                walls.extend(all_walls[..i].iter());
            } else {
                walls.extend(all_walls[walls.len()..i].iter());
            }

            utils::astar(
                s,
                e,
                |p| neighbours(p, size, &walls).map(|n| (n, 1)),
                |p| {
                    ((e.0 as i64 - p.0 as i64).unsigned_abs()
                        + (e.1 as i64 - p.1 as i64).unsigned_abs()) as usize
                },
                &mut working_space,
            )
            .is_some()
        });

    all_walls[byte_that_breaks]
}

pub fn part1(input: &str) -> i64 {
    solve_p1(input, (71, 71), 1024)
}

pub fn part2(input: &str) -> i64 {
    let result = solve_p2(input, (71, 71));
    (result.0 * result.1).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn example_part1() {
        let result = solve_p1(EXAMPLE.trim(), (7, 7), 12);
        assert_eq!(result, 22)
    }

    #[test]
    fn example_part2() {
        let result = solve_p2(EXAMPLE.trim(), (7, 7));
        assert_eq!(result, (6, 1))
    }
}
