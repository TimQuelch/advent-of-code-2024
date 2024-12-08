use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn in_bounds(pos: (i16, i16), bounds: (i16, i16)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < bounds.0 && pos.1 < bounds.1
}

// build iterator over pairs of the same type of antennas
fn build_positions(input: &str) -> ((i16, i16), impl Iterator<Item = ((i16, i16), (i16, i16))>) {
    let ncols: i16 = (input.lines().next().unwrap().len()).try_into().unwrap();
    let nrows: i16 = (((input.len() + 1) / (ncols as usize)) - 1)
        .try_into()
        .unwrap();

    let mut positions = HashMap::new();

    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars().enumerate().filter_map(move |(j, c)| match c {
                '.' => None,
                x => Some((x, (i as i16, j as i16))),
            })
        })
        .for_each(|(c, pos)| positions.entry(c).or_insert_with(Vec::new).push(pos));
    let positions_iter = positions
        .into_values()
        .flat_map(|ps| ps.into_iter().tuple_combinations::<(_, _)>());

    ((nrows, ncols), positions_iter)
}

struct NodeIterator {
    pos1: (i16, i16),
    step: (i16, i16),
    bounds: (i16, i16),
    coeff: i16,
}

impl NodeIterator {
    fn new(pos1: (i16, i16), pos2: (i16, i16), bounds: (i16, i16)) -> Self {
        return Self {
            pos1,
            step: (pos2.0 - pos1.0, pos2.1 - pos1.1),
            bounds,
            coeff: 0,
        };
    }

    fn calc_next(&self) -> (i16, i16) {
        (
            self.pos1.0 + self.coeff * self.step.0,
            self.pos1.1 + self.coeff * self.step.1,
        )
    }
}

impl Iterator for NodeIterator {
    type Item = (i16, i16);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.calc_next();
        match self.coeff {
            // iterate backwards
            x if x <= 0 && in_bounds(next, self.bounds) => {
                self.coeff -= 1;
                Some(next)
            }
            // out of bounds going backwards, switch to forwards and run again
            x if x <= 0 => {
                self.coeff = 1;
                self.next()
            }
            // iterate forwards
            _ if in_bounds(next, self.bounds) => {
                self.coeff += 1;
                Some(next)
            }
            // out of bounds going forward, end iteration
            _ => None,
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let ((nrows, ncols), positions) = build_positions(input);

    positions
        .flat_map(|((a, b), (x, y))| [(2 * x - a, 2 * y - b), (2 * a - x, 2 * b - y)])
        .filter(|&p| in_bounds(p, (nrows, ncols)))
        .collect::<HashSet<(i16, i16)>>()
        .len()
        .try_into()
        .unwrap()
}

pub fn part2(input: &str) -> i64 {
    let ((nrows, ncols), positions) = build_positions(input);

    positions
        .flat_map(|(p1, p2)| NodeIterator::new(p1, p2, (nrows, ncols)))
        .collect::<HashSet<(i16, i16)>>()
        .len()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 14)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 34)
    }
}
