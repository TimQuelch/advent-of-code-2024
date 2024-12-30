use core::str;
use std::collections::HashSet;

use itertools::Itertools;
use petgraph::graphmap::UnGraphMap;

fn str_to_key(s: &str) -> u16 {
    let mut i = s.bytes();
    let a = i.next().unwrap() as u16;
    let b = i.next().unwrap() as u16;
    (a << 8) | b
}

fn key_starts_with_t(n: u16) -> bool {
    (n >> 8) as u8 == b't'
}

pub fn part1(input: &str) -> i64 {
    let g = UnGraphMap::<_, ()>::from_edges(input.lines().map(|x| {
        let (a, b) = x.split_once('-').unwrap();
        (str_to_key(a), str_to_key(b))
    }));
    let result = g
        .nodes()
        .tuple_combinations()
        .filter(|&(a, b, c)| key_starts_with_t(a) || key_starts_with_t(b) || key_starts_with_t(c))
        .filter(|&(a, b, c)| {
            g.contains_edge(a, b) && g.contains_edge(a, c) && g.contains_edge(b, c)
        })
        .count();
    return result.try_into().unwrap();
}

fn bron_kerbosch(
    g: &UnGraphMap<u16, ()>,
    current: HashSet<u16>,
    mut potential: HashSet<u16>,
    mut excluded: HashSet<u16>,
    mut max_cliques: &mut Vec<HashSet<u16>>,
) {
    if potential.len() == 0 && excluded.len() == 0 {
        max_cliques.push(current);
    } else {
        let vs = potential.clone();
        for v in vs {
            let mut next = current.clone();
            next.insert(v);
            let v_ns = HashSet::from_iter(g.neighbors(v));
            bron_kerbosch(
                &g,
                next,
                potential.intersection(&v_ns).cloned().collect(),
                excluded.intersection(&v_ns).cloned().collect(),
                &mut max_cliques,
            );

            potential.remove(&v);
            excluded.insert(v);
        }
    }
}

fn max_cliques(g: &UnGraphMap<u16, ()>) -> Vec<HashSet<u16>> {
    let current = HashSet::new();
    let potential = g.nodes().collect();
    let excluded = HashSet::new();
    let mut max_cliques = vec![];

    bron_kerbosch(g, current, potential, excluded, &mut max_cliques);

    max_cliques
}

fn solve_p2(input: &str) -> Vec<u16> {
    let g = UnGraphMap::<_, ()>::from_edges(input.lines().map(|x| {
        let (a, b) = x.split_once('-').unwrap();
        (str_to_key(a), str_to_key(b))
    }));

    let mut result = max_cliques(&g)
        .into_iter()
        .max_by_key(|s| s.len())
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();

    result.sort();

    result
}

pub fn part2(input: &str) -> i64 {
    let result = solve_p2(input);

    // Uncomment to print the required answer

    // fn key_to_str(x: u16) -> String {
    //     let a = (x >> 8) as u8;
    //     let b = (x % (1 << 8)) as u8;
    //     str::from_utf8(&[a, b]).unwrap().to_owned()
    // }
    // let x = result
    //     .iter()
    //     .map(|&x| key_to_str(x))
    //     .collect::<Vec<_>>()
    //     .join(",");
    // println!("{}", x);

    // Convert into a numeric result for my interfaces
    return result
        .into_iter()
        .map(|x| x as i64)
        .sum::<i64>()
        .try_into()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 7)
    }

    #[test]
    fn example_part2() {
        let result = solve_p2(EXAMPLE.trim());
        assert_eq!(
            result,
            vec![
                str_to_key("co"),
                str_to_key("de"),
                str_to_key("ka"),
                str_to_key("ta")
            ]
        )
    }
}
