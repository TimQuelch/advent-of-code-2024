use std::{cmp::Ordering, collections::HashSet};

type RuleSet = HashSet<(u8, u8)>;

fn build_rules(input: &str) -> RuleSet {
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|l| {
            let mut iter = l.split('|').map(|n| n.parse::<u8>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect()
}

fn compare<'a>(rules: &'a RuleSet) -> impl Fn(&u8, &u8) -> Ordering + 'a {
    // we assume that at most one of the orders of the values is present. This is valid if we assume
    // there is no implicit orderings. With this assumption if the pair (a, b) is not present then
    // it is implied that (b, a) must be present instead. We don't need to check for it explicitly.
    move |a: &u8, b: &u8| match (*a, *b) {
        p if rules.contains(&p) => Ordering::Less,
        _ => Ordering::Greater,
    }
}

fn is_sorted_compare(cmp: impl Fn(&u8, &u8) -> Ordering) -> impl Fn(&u8, &u8) -> bool {
    move |a, b| match cmp(a, b) {
        Ordering::Greater => false,
        _ => true,
    }
}

fn update_iter(input: &str) -> impl Iterator<Item = &str> {
    input.lines().skip_while(|line| !line.is_empty()).skip(1)
}

pub fn part1(input: &str) -> i32 {
    let rules = build_rules(input);
    let compare_fn = is_sorted_compare(compare(&rules));

    let mut working_space: Vec<u8> = vec![];

    let result = update_iter(input)
        .filter_map(|l| {
            working_space.clear();
            working_space.extend(l.split(',').map(|n| n.parse::<u8>().unwrap()));

            match working_space.is_sorted_by(&compare_fn) {
                true => Some(working_space.get(working_space.len() / 2).unwrap().clone() as i32),
                false => None,
            }
        })
        .sum();

    return result;
}

pub fn part2(input: &str) -> i32 {
    let rules = build_rules(input);
    let compare_fn = compare(&rules);
    let is_sorted_compare_fn = is_sorted_compare(&compare_fn);

    let mut working_space: Vec<u8> = vec![];

    let result = update_iter(input)
        .filter_map(|l| {
            working_space.clear();
            working_space.extend(l.split(',').map(|n| n.parse::<u8>().unwrap()));

            let mid = working_space.len() / 2;

            match working_space.is_sorted_by(&is_sorted_compare_fn) {
                true => None,
                false => Some(
                    working_space
                        .select_nth_unstable_by(mid, &compare_fn)
                        .1
                        .clone() as i32,
                ),
            }
        })
        .sum();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 143)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 123)
    }
}

pub mod old {
    use std::{cmp::Ordering, collections::HashSet};

    use petgraph::{
        algo::{toposort, DfsSpace},
        prelude::DiGraphMap,
    };
    type Graph = DiGraphMap<u8, ()>;
    type WorkingSpace = DfsSpace<u8, HashSet<u8>>;

    fn build_graph(input: &str) -> Graph {
        Graph::from_edges(input.lines().take_while(|line| !line.is_empty()).map(|l| {
            let mut iter = l.split('|').map(|n| n.parse::<u8>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        }))
    }

    fn get_order(whole_graph: &Graph, vals: &Vec<u8>, space: &mut WorkingSpace) -> Vec<u8> {
        let sub_graph = Graph::from_edges(
            whole_graph
                .all_edges()
                .filter(|(a, b, _)| vals.contains(a) && vals.contains(b)),
        );

        toposort(&sub_graph, Some(space)).unwrap()
    }

    pub fn part1(input: &str) -> i32 {
        let whole_graph = build_graph(input);

        let mut dfs_space = DfsSpace::new(&whole_graph);

        let result = input
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .filter_map(|l| {
                let vals = l
                    .split(',')
                    .map(|n| n.parse::<u8>().unwrap())
                    .collect::<Vec<_>>();

                let order = get_order(&whole_graph, &vals, &mut dfs_space);

                match order.iter().eq(vals.iter()) {
                    true => Some(vals.get(vals.len() / 2).unwrap().clone() as i32),
                    false => None,
                }
            })
            .sum();

        return result;
    }

    pub fn part2(input: &str) -> i32 {
        let whole_graph = build_graph(input);

        let mut dfs_space = DfsSpace::new(&whole_graph);

        let result = input
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .filter_map(|l| {
                let mut vals = l
                    .split(',')
                    .map(|n| n.parse::<u8>().unwrap())
                    .collect::<Vec<_>>();

                let order = get_order(&whole_graph, &vals, &mut dfs_space);

                let compare = |a: &u8, b: &u8| {
                    let first = order.iter().find(|&&n| n == *a || n == *b).unwrap();
                    match *first {
                        x if x == *a => Ordering::Less,
                        x if x == *b => Ordering::Greater,
                        _ => panic!("weird result for first {}", first),
                    }
                };

                let mid = vals.len() / 2;
                match vals.iter().eq(order.iter()) {
                    true => None,
                    false => Some(vals.select_nth_unstable_by(mid, compare).1.clone() as i32),
                }
            })
            .sum();

        return result;
    }
}
