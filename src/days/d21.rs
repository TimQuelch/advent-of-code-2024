use itertools::Itertools;
use petgraph::algo;
use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;
use std::iter;
use std::sync::LazyLock;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum NPad {
    A,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
}

impl From<u8> for NPad {
    fn from(value: u8) -> Self {
        match value {
            b'0' => N0,
            b'1' => N1,
            b'2' => N2,
            b'3' => N3,
            b'4' => N4,
            b'5' => N5,
            b'6' => N6,
            b'7' => N7,
            b'8' => N8,
            b'9' => N9,
            b'A' => NPad::A,
            _ => panic!("invalid input {}", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum DPad {
    A,
    Up,
    Down,
    Left,
    Right,
}

use DPad::*;
use NPad::*;

const NPAD_ADJACENCIES: [(NPad, NPad, DPad); 30] = [
    (NPad::A, N0, Left),
    (NPad::A, N3, Up),
    (N0, NPad::A, Right),
    (N0, N2, Up),
    (N1, N2, Right),
    (N1, N4, Up),
    (N2, N0, Down),
    (N2, N1, Left),
    (N2, N3, Right),
    (N2, N5, Up),
    (N3, NPad::A, Down),
    (N3, N2, Left),
    (N3, N6, Up),
    (N4, N1, Down),
    (N4, N5, Right),
    (N4, N7, Up),
    (N5, N2, Down),
    (N5, N4, Left),
    (N5, N6, Right),
    (N5, N8, Up),
    (N6, N3, Down),
    (N6, N5, Left),
    (N6, N9, Up),
    (N7, N4, Down),
    (N7, N8, Right),
    (N8, N5, Down),
    (N8, N7, Left),
    (N8, N9, Right),
    (N9, N6, Down),
    (N9, N8, Left),
];

const ALL_NPADS: [NPad; 11] = [NPad::A, N0, N1, N2, N3, N4, N5, N6, N7, N8, N9];

const ALL_DPADS: [DPad; 5] = [DPad::A, Up, Down, Left, Right];

// Manually calculated shortest paths for each pair of DPad buttons. There are alternate paths, but
// the ones listed here give the shortest responses (as determined by trial and error). A rigorous
// solution would return all paths and then calculate the cost of each one, choosing the minimum
static DPAD_PATHS: LazyLock<HashMap<(DPad, DPad), Vec<DPad>>> = LazyLock::new(|| {
    HashMap::from_iter(
        [
            ((DPad::A, Up), vec![Left]),
            ((DPad::A, Down), vec![Left, Down]), // or down, left
            ((DPad::A, Left), vec![Down, Left, Left]),
            ((DPad::A, Right), vec![Down]),
            ((Up, DPad::A), vec![Right]),
            ((Up, Down), vec![Down]),
            ((Up, Left), vec![Down, Left]),
            ((Up, Right), vec![Down, Right]),
            ((Down, DPad::A), vec![Up, Right]), // or right, up
            ((Down, Up), vec![Up]),
            ((Down, Left), vec![Left]),
            ((Down, Right), vec![Right]),
            ((Left, DPad::A), vec![Right, Right, Up]),
            ((Left, Up), vec![Right, Up]),
            ((Left, Down), vec![Right]),
            ((Left, Right), vec![Right, Right]),
            ((Right, DPad::A), vec![Up]),
            ((Right, Up), vec![Left, Up]), // or up, left
            ((Right, Down), vec![Left]),
            ((Right, Left), vec![Left, Left]),
        ]
        .into_iter(),
    )
});

fn build_npad_graph(depth: u32) -> DiGraphMap<(NPad, DPad), u64> {
    let mut cache = HashMap::new();

    let mut path_cost = |edge, depth| path_cost(edge, depth, &mut cache);

    // Node is (N, D), |N| = 11, |D| = 5
    // |E| is 205
    let mut g = DiGraphMap::with_capacity(11 * 5, 205);

    // Edges along the adjacencies of the NPad buttons
    for (s, e, d_edge) in NPAD_ADJACENCIES {
        for d_curr in ALL_DPADS {
            g.add_edge((s, d_curr), (e, d_edge), path_cost((d_curr, d_edge), depth));
        }
    }

    // Edges on each NPad button moving from the current DPad to the DPad A button
    for (np, dp) in ALL_NPADS.iter().cartesian_product(ALL_DPADS.iter()) {
        g.add_edge((*np, *dp), (*np, DPad::A), path_cost((*dp, DPad::A), depth));
    }

    g
}

fn path_cost(edge: (DPad, DPad), depth: u32, cache: &mut HashMap<((DPad, DPad), u32), u64>) -> u64 {
    if let Some(&k) = cache.get(&(edge, depth)) {
        return k;
    }

    if depth == 0 {
        return 1;
    }

    if edge.0 == edge.1 {
        return 1;
    }

    let path = DPAD_PATHS.get(&edge).unwrap();

    let result = iter::once(DPad::A)
        .chain(path.iter().cloned())
        .chain(iter::once(DPad::A))
        .tuple_windows()
        .map(|edge| path_cost(edge, depth - 1, cache))
        .sum::<u64>();

    cache.insert((edge, depth), result);
    result
}

fn solve(input: &str, depth: u32) -> i64 {
    let g = build_npad_graph(depth);

    let result = input
        .lines()
        .map(|s| {
            let n = s[..(s.len() - 1)].parse::<u64>().unwrap();

            let cost = iter::once(NPad::A)
                .chain(s.bytes().map(|c| c.into()))
                .tuple_windows()
                .map(|(a, b)| {
                    let start = (a, DPad::A);
                    let end = (b, DPad::A);
                    let result = algo::dijkstra(&g, start, Some(end), |(_, _, &c)| c);

                    result.get(&end).unwrap().clone()
                })
                .sum::<u64>();

            n * cost
        })
        .sum::<u64>();

    result.try_into().unwrap()
}

pub fn part1(input: &str) -> i64 {
    solve(input, 2)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
029A
980A
179A
456A
379A
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 126384)
    }

    #[test]
    fn short_2() {
        let result = solve("029A", 2);
        assert_eq!(result, 1972)
    }

    #[test]
    fn short_25() {
        let result = solve("029A", 25);
        assert_eq!(result, 2379451789590)
    }

    #[test]
    fn shorter_0() {
        let result = solve("1A", 0);
        assert_eq!(result, 8)
    }

    #[test]
    fn shorter_1() {
        let result = solve("1A", 1);
        assert_eq!(result, 18)
    }

    #[test]
    fn shorter_2() {
        let result = solve("1A", 2);
        assert_eq!(result, 48)
    }

    #[test]
    fn shorter_3() {
        let result = solve("1A", 3);
        assert_eq!(result, 118)
    }

    #[test]
    fn shorter_25() {
        let result = solve("1A", 25);
        assert_eq!(result, 58472946734)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 154115708116294)
    }
}
