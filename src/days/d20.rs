use ndarray::Array2;

type Pos = (usize, usize);
type Grid = Array2<V>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum V {
    Open,
    Wall,
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

fn neighbours(grid: &Grid, p: Pos) -> impl Iterator<Item = Pos> + use<'_> {
    [
        p.0.checked_sub(1).map(|i| (i, p.1)),
        p.0.checked_add(1).map(|i| (i, p.1)),
        p.1.checked_sub(1).map(|j| (p.0, j)),
        p.1.checked_add(1).map(|j| (p.0, j)),
    ]
    .into_iter()
    .filter_map(move |p| p.filter(|p| grid.get(*p).map_or(false, |&y| y == V::Open)))
}

fn first_pass(grid: &Grid, start: Pos, end: Pos) -> Vec<(usize, Pos)> {
    let mut vals = Vec::with_capacity(10_000);

    let mut i = 0;

    let mut prev: Option<Pos> = None;
    let mut current = start;
    loop {
        vals.push((i, current));
        i += 1;

        if current == end {
            break;
        }

        let prev_copy = prev.clone();
        let mut ns =
            neighbours(&grid, current).filter(|n| prev_copy.map_or(true, |prev| *n != prev));
        prev = Some(current);
        current = ns.next().unwrap();
        assert!(ns.next().is_none());
    }
    vals
}

pub fn solve(input: &str, cheat_duration: u32, threshold: u32) -> i64 {
    let grid = build_grid(input);
    let start = find_pos(input, 'S');
    let end = find_pos(input, 'E');

    let path = first_pass(&grid, start, end);

    let result = path
        .iter()
        .flat_map(|(i, start)| {
            path.iter()
                .skip(i + threshold as usize)
                .map(move |(j, end)| ((i, start), (j, end)))
        })
        .filter(|((i, start), (j, end))| {
            let xdiff = (start.0 as i32 - end.0 as i32).unsigned_abs();

            // this saves some time
            if xdiff > cheat_duration {
                return false;
            }

            let ydiff = (start.1 as i32 - end.1 as i32).unsigned_abs();
            let shortcut_dist = xdiff + ydiff;

            shortcut_dist <= cheat_duration
                && ((**j) as i32 - (**i) as i32 - shortcut_dist as i32) >= threshold as i32
        })
        .count();

    result.try_into().unwrap()
}

pub fn part1(input: &str) -> i64 {
    solve(input, 2, 100)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 20, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn example_part1_64() {
        let result = solve(EXAMPLE.trim(), 2, 64);
        assert_eq!(result, 1)
    }

    #[test]
    fn example_part1_40() {
        let result = solve(EXAMPLE.trim(), 2, 40);
        assert_eq!(result, 2)
    }

    #[test]
    fn example_part1_38() {
        let result = solve(EXAMPLE.trim(), 2, 38);
        assert_eq!(result, 3)
    }

    #[test]
    fn example_part1_2() {
        let result = solve(EXAMPLE.trim(), 2, 2);
        assert_eq!(result, 44)
    }

    #[test]
    fn example_part2_76() {
        let result = solve(EXAMPLE.trim(), 20, 76);
        assert_eq!(result, 3)
    }

    #[test]
    fn example_part2_64() {
        let result = solve(EXAMPLE.trim(), 20, 64);
        assert_eq!(result, 86)
    }
}
