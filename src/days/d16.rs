use ndarray::Array2;
use std::collections::HashMap;

use super::utils;

type Grid = Array2<V>;
type Pos = (u8, u8);
type Dir = (i16, i16);
type Cost = u64;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum V {
    Open,
    Wall,
}

fn build_grid(input: &str) -> Grid {
    let ncols = input.lines().next().unwrap().len();
    let nrows = ((input.len() + 1) / ncols) - 1;

    let grid = Grid::from_shape_vec(
        (nrows, ncols),
        input
            .chars()
            .filter_map(|c| match c {
                '.' | 'S' | 'E' => Some(V::Open),
                '#' => Some(V::Wall),
                _ => None,
            })
            .collect(),
    )
    .unwrap();

    grid
}

fn find_pos(input: &str, to_find: char) -> Pos {
    input
        .lines()
        .enumerate()
        .find_map(|(i, l)| {
            l.chars()
                .enumerate()
                .find_map(|(j, c)| match c {
                    c if c == to_find => Some(j),
                    _ => None,
                })
                .and_then(|j| j.try_into().ok())
                .and_then(|j| i.try_into().ok().map(|i| (i, j)))
        })
        .unwrap()
}

fn next_dirs(dir: Dir) -> impl Iterator<Item = Dir> {
    match dir {
        (0, _) => [(-1, 0), (1, 0)].into_iter(),
        (_, 0) => [(0, -1), (0, 1)].into_iter(),
        _ => panic!("invalid direction: {:?}", dir),
    }
}

fn increment_pos(p: Pos, d: Dir) -> Option<Pos> {
    let x = (p.0 as i16 + d.0).try_into().ok();
    let y = (p.1 as i16 + d.1).try_into().ok();
    x.zip(y)
}

fn neighbours(p: Pos, d: Dir, grid: &Grid) -> impl Iterator<Item = ((Pos, Dir), Cost)> + use<'_> {
    let forward = std::iter::once(
        increment_pos(p, d)
            .filter(|p| {
                grid.get((p.0 as usize, p.1 as usize))
                    .map_or(false, |&v| v == V::Open)
            })
            .map(|p| ((p, d), 1 as Cost)),
    )
    .filter_map(|x| x);

    let turning = next_dirs(d).map(move |d| ((p, d), 1000 as Cost));
    forward.chain(turning)
}

pub fn part1(input: &str) -> i64 {
    let grid = build_grid(input);
    let start = find_pos(input, 'S');
    let end = find_pos(input, 'E');

    let mut working_space = utils::DijkstraWorkingSpace::new();

    let costs = utils::dijkstra_cost_map(
        (start, (0, 1)),
        |(p, d)| neighbours(p, d, &grid),
        &mut working_space,
        45_000
    );

    let result = get_min_cost(&costs, end);

    result.unwrap().try_into().unwrap()
}

fn get_min_cost(map: &HashMap<(Pos, Dir), Cost>, p: Pos) -> Option<Cost> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .map(|d| map.get(&(p, d)))
        .into_iter()
        .filter_map(|x| x)
        .cloned()
        .min()
}

fn reverse_dir(d: Dir) -> Dir {
    (d.0 * -1, d.1 * -1)
}

fn is_on_shortest_path(
    forward_costs: &HashMap<(Pos, Dir), Cost>,
    reverse_costs: &HashMap<(Pos, Dir), Cost>,
    p: Pos,
    goal: Cost,
) -> bool {
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    dirs.iter()
        .zip(dirs.iter().map(|&d| reverse_dir(d)))
        .any(|(f_dir, r_dir)| {
            let f = forward_costs.get(&(p, *f_dir));
            let r = reverse_costs.get(&(p, r_dir));
            f.zip(r).map(|(&f, &r)| f + r == goal).unwrap_or(false)
        })
}

pub fn part2(input: &str) -> i64 {
    let grid = build_grid(input);
    let start = find_pos(input, 'S');
    let end = find_pos(input, 'E');

    let mut working_space = utils::DijkstraWorkingSpace::new();

    let forward_costs = utils::dijkstra_cost_map(
        (start, (0, 1)),
        |(p, d)| neighbours(p, d, &grid),
        &mut working_space,
        45_000,
    );

    let lowest_end_cost = get_min_cost(&forward_costs, end).unwrap();

    let ((_, d), _) = forward_costs
        .iter()
        .find(|((p, _), c)| *p == end && **c == lowest_end_cost)
        .unwrap();
    let reversed_dir = (d.0 * -1, d.1 * -1);
    // println!("\n\nreversing!");

    let reverse_costs = utils::dijkstra_cost_map(
        (end, reversed_dir),
        |(p, d)| neighbours(p, d, &grid),
        &mut working_space,
        forward_costs.len()
    );

    println!("forward costs {}", forward_costs.len());
    println!("reverse costs {}", reverse_costs.len());

    // println!("end cost {:?}", forward_costs.get(&(end, (0, 1))));
    // println!("end cost {:?}", forward_costs.get(&(end, (0, -1))));
    // println!("end cost {:?}", forward_costs.get(&(end, (1, 0))));
    // println!("end cost {:?}", forward_costs.get(&(end, (-1, 0))));
    // println!("start cost {:?}", forward_costs.get(&(start, (0, 1))));
    // println!("start cost {:?}", forward_costs.get(&(start, (0, -1))));
    // println!("start cost {:?}", forward_costs.get(&(start, (1, 0))));
    // println!("start cost {:?}", forward_costs.get(&(start, (-1, 0))));
    // println!("end cost {:?}", reverse_costs.get(&(end, (0, 1))));
    // println!("end cost {:?}", reverse_costs.get(&(end, (0, -1))));
    // println!("end cost {:?}", reverse_costs.get(&(end, (1, 0))));
    // println!("end cost {:?}", reverse_costs.get(&(end, (-1, 0))));
    // println!("start cost {:?}", reverse_costs.get(&(start, (0, 1))));
    // println!("start cost {:?}", reverse_costs.get(&(start, (0, -1))));
    // println!("start cost {:?}", reverse_costs.get(&(start, (1, 0))));
    // println!("start cost {:?}", reverse_costs.get(&(start, (-1, 0))));

    let count = grid
        .indexed_iter()
        .filter(|(_, v)| **v == V::Open)
        .map(|(p, _)| (p.0 as u8, p.1 as u8))
        .filter(|&p| is_on_shortest_path(&forward_costs, &reverse_costs, p, lowest_end_cost))
        .count()
        + 1;

    count.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const EXAMPLE2: &str = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn example_part1_1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 7036)
    }

    #[test]
    fn example_part1_2() {
        let result = part1(EXAMPLE2.trim());
        assert_eq!(result, 11048)
    }

    #[test]
    fn example_part2_1() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 45)
    }

    #[test]
    fn example_part2_2() {
        let result = part2(EXAMPLE2.trim());
        assert_eq!(result, 64)
    }
}
