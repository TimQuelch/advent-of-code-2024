use std::collections::HashSet;

use ndarray::Array2;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum V {
    Open,
    Obstacle,
}

type Pos = (usize, usize);
type Grid = Array2<V>;

impl std::fmt::Debug for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            V::Open => write!(f, "."),
            V::Obstacle => write!(f, "#"),
        }
    }
}

fn build_grid(input: &str) -> (Grid, Pos) {
    let ncols = input.lines().next().unwrap().len();
    let nrows = ((input.len() + 1) / ncols) - 1;

    let grid = Grid::from_shape_vec(
        (nrows, ncols),
        input
            .chars()
            .filter_map(|c| match c {
                '.' | '^' => Some(V::Open),
                '#' => Some(V::Obstacle),
                _ => None,
            })
            .collect(),
    )
    .unwrap();

    let pos: Pos = input
        .lines()
        .enumerate()
        .find_map(|(i, l)| {
            l.chars()
                .enumerate()
                .find_map(|(j, c)| match c {
                    '^' => Some(j),
                    _ => None,
                })
                .and_then(|j| j.try_into().ok())
                .and_then(|j| i.try_into().ok().map(|i| (i, j)))
        })
        .unwrap();

    (grid, pos)
}

fn next_pos(pos: Pos, dir: Dir) -> Option<Pos> {
    match dir {
        Dir::U => pos.0.checked_sub(1).map(|r| (r, pos.1)),
        Dir::D => pos.0.checked_add(1).map(|r| (r, pos.1)),
        Dir::L => pos.1.checked_sub(1).map(|c| (pos.0, c)),
        Dir::R => pos.1.checked_add(1).map(|c| (pos.0, c)),
    }
}

fn next_dir(dir: Dir) -> Dir {
    match dir {
        Dir::U => Dir::R,
        Dir::D => Dir::L,
        Dir::L => Dir::U,
        Dir::R => Dir::D,
    }
}

pub fn part1(input: &str) -> i32 {
    let (grid, mut pos) = build_grid(input);
    let mut dir = Dir::U;
    let mut visited = HashSet::new();

    while let Some((nextp, nextv)) =
        next_pos(pos, dir).and_then(|next| grid.get(next).map(|v| (next, v)))
    {
        visited.insert(pos);

        match nextv {
            V::Open => pos = nextp,
            V::Obstacle => dir = next_dir(dir),
        }
    }

    visited.insert(pos);

    return visited.len().try_into().unwrap();
}

struct GridWithObstacle<'a> {
    grid: &'a Grid,
    new_obstacle_pos: Pos,
}

impl<'a> GridWithObstacle<'a> {
    fn get(&self, index: Pos) -> Option<&'a V> {
        match index {
            p if p == self.new_obstacle_pos => Some(&V::Obstacle),
            _ => self.grid.get(index),
        }
    }
}

fn check_loop_in_grid(
    grid: &GridWithObstacle,
    visited: &HashSet<(Pos, Dir)>,
    mut pos: Pos,
    mut dir: Dir,
    loop_visited: &mut HashSet<(Pos, Dir)>,
) -> bool {
    loop_visited.clear();

    while let Some((nextp, nextv)) =
        next_pos(pos, dir).and_then(|next| grid.get(next).map(|v| (next, v)))
    {
        loop_visited.insert((pos, dir));

        if loop_visited.contains(&(nextp, dir)) || visited.contains(&(nextp, dir)) {
            return true;
        }

        match nextv {
            V::Open => pos = nextp,
            V::Obstacle => dir = next_dir(dir),
        }
    }

    false
}

fn contains_any_direction(set: &HashSet<(Pos, Dir)>, pos: Pos) -> bool {
    set.contains(&(pos, Dir::U))
        || set.contains(&(pos, Dir::D))
        || set.contains(&(pos, Dir::L))
        || set.contains(&(pos, Dir::R))
}

pub fn part2(input: &str) -> i32 {
    let (grid, mut pos) = build_grid(input);
    let mut dir = Dir::U;
    let mut visited = HashSet::new();
    let mut working_space = HashSet::new();
    let mut result = HashSet::new();

    while let Some((nextp, nextv)) =
        next_pos(pos, dir).and_then(|next| grid.get(next).map(|v| (next, v)))
    {
        visited.insert((pos, dir));

        if *nextv == V::Open
            && !result.contains(&nextp)
            && !contains_any_direction(&visited, nextp)
            && {
                let new_grid = GridWithObstacle {
                    grid: &grid,
                    new_obstacle_pos: nextp,
                };
                check_loop_in_grid(&new_grid, &visited, pos, dir, &mut working_space)
            }
        {
            result.insert(nextp);
        }

        match nextv {
            V::Open => pos = nextp,
            V::Obstacle => dir = next_dir(dir),
        }
    }

    return result.len().try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 41)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 6)
    }
}
