use std::collections::HashSet;

use ndarray::{s, Array2};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Map {
    Open,
    Box,
    Wall,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    U,
    D,
    L,
    R,
}

type Pos = (usize, usize);
type Grid = Array2<Map>;

fn build_grid(input: &str) -> (Grid, Pos) {
    let ncols = input.lines().next().unwrap().len();
    let nrows = ((input.len() + 1) / ncols) - 1;

    let grid = Array2::from_shape_vec(
        (nrows, ncols),
        input
            .chars()
            .filter_map(|c| match c {
                '#' => Some(Map::Wall),
                '.' | '@' => Some(Map::Open),
                'O' => Some(Map::Box),
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
                    '@' => Some(j),
                    _ => None,
                })
                .and_then(|j| j.try_into().ok())
                .and_then(|j| i.try_into().ok().map(|i| (i, j)))
        })
        .unwrap();

    (grid, pos)
}

fn incr_pos(pos: Pos, dir: Dir) -> Pos {
    match dir {
        Dir::U => (pos.0 - 1, pos.1),
        Dir::D => (pos.0 + 1, pos.1),
        Dir::L => (pos.0, pos.1 - 1),
        Dir::R => (pos.0, pos.1 + 1),
    }
}

fn do_move_p1(grid: &mut Grid, pos: Pos, dir: Dir) -> Pos {
    let mut full_slice = match dir {
        Dir::U => grid.slice_mut(s![..pos.0;-1, pos.1]),
        Dir::D => grid.slice_mut(s![(pos.0 + 1).., pos.1]),
        Dir::L => grid.slice_mut(s![pos.0, ..pos.1;-1]),
        Dir::R => grid.slice_mut(s![pos.0, (pos.1 + 1)..]),
    };

    let first_wall = full_slice
        .iter()
        .position(|x| matches!(*x, Map::Wall))
        .unwrap();

    let mut move_slice = full_slice.slice_mut(s![..first_wall]);

    if move_slice.len() == 0 {
        return pos;
    }

    if let Some(first_open) = move_slice.iter().position(|x| matches!(*x, Map::Open)) {
        if first_open != 0 {
            move_slice.slice_mut(s![1..(first_open + 1)]).fill(Map::Box);
            move_slice[0] = Map::Open;
        }

        incr_pos(pos, dir)
    } else {
        pos
    }
}

pub fn part1(input: &str) -> i64 {
    let (map, commands) = input.split_once("\n\n").unwrap();
    let (mut grid, mut pos) = build_grid(map);

    let command_iter = commands.chars().filter_map(|c| match c {
        '^' => Some(Dir::U),
        'v' => Some(Dir::D),
        '<' => Some(Dir::L),
        '>' => Some(Dir::R),
        _ => None,
    });

    for c in command_iter {
        pos = do_move_p1(&mut grid, pos, c);
    }

    let result = grid
        .indexed_iter()
        .filter(|(_, x)| **x == Map::Box)
        .map(|((x, y), _)| x * 100 + y)
        .sum::<usize>();

    result.try_into().unwrap()
}

fn wide_box(pos: Pos) -> Pos {
    (pos.0, pos.1 + 1)
}

fn do_move_p2(pos: Pos, dir: Dir, walls: &HashSet<Pos>, boxes: &mut Vec<Pos>) -> Pos {
    let new_pos = incr_pos(pos, dir);

    if walls.contains(&new_pos) {
        return pos;
    }

    let box_hit = boxes
        .iter()
        .enumerate()
        .find(|(_, &b)| b == new_pos || wide_box(b) == new_pos);

    match box_hit {
        None => new_pos,
        Some((i, _)) => {
            let mut boxes_to_check = vec![i];
            let mut boxes_to_move = vec![];

            let can_move_boxes = loop {
                if let Some(bi) = boxes_to_check.pop() {
                    let new_box_pos = incr_pos(boxes[bi], dir);

                    if walls.contains(&new_box_pos) || walls.contains(&wide_box(new_box_pos)) {
                        break false;
                    }

                    boxes_to_check.extend(
                        boxes
                            .iter()
                            .enumerate()
                            .filter(|&(_, &b)| {
                                wide_box(b) == new_box_pos
                                    || b == new_box_pos
                                    || b == wide_box(new_box_pos)
                            })
                            .map(|(i, _)| i)
                            .filter(|&i| !boxes_to_move.contains(&i) && i != bi),
                    );
                    boxes_to_move.push(bi);
                } else {
                    break true;
                }
            };

            if can_move_boxes {
                for bi in boxes_to_move {
                    let new_box_pos = incr_pos(boxes[bi], dir);
                    boxes[bi] = new_box_pos;
                }

                new_pos
            } else {
                pos
            }
        }
    }
}

pub fn part2(input: &str) -> i64 {
    let (map, commands) = input.split_once("\n\n").unwrap();

    let mut walls = HashSet::new();
    let mut boxes = Vec::new();
    let mut pos = (0, 0);

    for (i, l) in map.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((i, j * 2));
                    walls.insert((i, j * 2 + 1));
                }
                'O' => {
                    boxes.push((i, j * 2));
                }
                '@' => {
                    pos = (i, j * 2);
                }
                _ => {}
            }
        }
    }

    let command_iter = commands.chars().filter_map(|c| match c {
        '^' => Some(Dir::U),
        'v' => Some(Dir::D),
        '<' => Some(Dir::L),
        '>' => Some(Dir::R),
        _ => None,
    });

    for c in command_iter {
        pos = do_move_p2(pos, c, &walls, &mut boxes);
    }

    let result = boxes
        .into_iter()
        .map(|(x, y)| x * 100 + y)
        .sum::<usize>()
        .try_into()
        .unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    const EXAMPLE_2: &str = "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const EXAMPLE_3: &str = "
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 10092)
    }

    #[test]
    fn example2_part1() {
        let result = part1(EXAMPLE_2.trim());
        assert_eq!(result, 2028)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 9021)
    }

    #[test]
    fn example2_part2() {
        let result = part2(EXAMPLE_3.trim());
        assert_eq!(result, (1 * 100 + 5) + (2 * 100 + 7) + (3 * 100 + 6))
    }
}
