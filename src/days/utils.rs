use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::{BinaryHeap, HashSet},
    hash::Hash,
    ops::Add,
};

// The std library version of this is still in unstable
pub fn minmax_by<T, F>(a: T, b: T, mut compare: F) -> (T, T)
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    match compare(&a, &b) {
        Ordering::Greater => (b, a),
        _ => (a, b),
    }
}

pub struct BfsWorkingSpace {
    forward_queue: Vec<(usize, usize)>,
    reverse_queue: Vec<(usize, usize)>,
    visited: HashSet<(usize, usize)>,
    next_values: Vec<(usize, usize)>,
}

impl BfsWorkingSpace {
    pub fn new() -> Self {
        Self {
            forward_queue: Vec::new(),
            reverse_queue: Vec::new(),
            visited: HashSet::new(),
            next_values: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.forward_queue.clear();
        self.reverse_queue.clear();
        self.visited.clear();
        self.next_values.clear();
    }
}

pub fn bi_bfs<F, I>(
    s: (usize, usize),
    e: (usize, usize),
    neighbours: F,
    ws: &mut BfsWorkingSpace,
) -> Option<u32>
where
    F: Fn((usize, usize)) -> I,
    I: Iterator<Item = (usize, usize)>,
{
    let mut depth = 0;

    ws.clear();

    ws.forward_queue.push(s);
    ws.reverse_queue.push(e);
    ws.visited.extend([s, e]);

    loop {
        // Get the queue with smallest number of branches
        let (next_queue, other_queue) =
            minmax_by(&mut ws.forward_queue, &mut ws.reverse_queue, |q1, q2| {
                q1.len().cmp(&q2.len())
            });

        ws.next_values.clear();

        while let Some(p) = next_queue.pop() {
            let ns = neighbours(p);
            let old_l = ws.next_values.len();
            for n in ns {
                if other_queue.contains(&n) {
                    return Some(depth + 1);
                }
                if !ws.visited.contains(&n) {
                    ws.next_values.push(n);
                }
            }
            ws.visited.extend(ws.next_values[old_l..].iter());
        }
        if ws.next_values.is_empty() {
            return None;
        }
        next_queue.extend(ws.next_values.iter());
        depth += 1;
    }
}

pub trait Cost: Default + Copy + Eq + PartialEq + Ord + Add<Output = Self> {}

impl<T: Default + Copy + Eq + PartialEq + Ord + Add<Output = Self>> Cost for T {}

pub struct WithCost<T, C: Cost> {
    node: T,
    cost: C,
    heuristic: C,
}

impl<T, C: Cost> WithCost<T, C> {
    fn total(&self) -> C {
        self.cost + self.heuristic
    }
}

impl<T, C: Cost> PartialEq for WithCost<T, C> {
    fn eq(&self, other: &Self) -> bool {
        self.total() == other.total()
    }
}

impl<T, C: Cost> Eq for WithCost<T, C> {}

impl<T, C: Cost> Ord for WithCost<T, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total().cmp(&self.total())
    }
}

impl<T, C: Cost> PartialOrd for WithCost<T, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AstarWorkingSpace<T, C: Cost> {
    queue: BinaryHeap<WithCost<T, C>>,
    visited: HashSet<T>,
}

impl<T, C: Cost> AstarWorkingSpace<T, C> {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            visited: HashSet::new(),
        }
    }
}

pub fn astar<T, C: Cost, FN, I, FH>(
    s: T,
    e: T,
    neighbours: FN,
    heuristic: FH,
    ws: &mut AstarWorkingSpace<T, C>,
) -> Option<C>
where
    T: Copy + Eq + Hash,
    C: Cost,
    FN: Fn(T) -> I,
    I: Iterator<Item = (T, C)>,
    FH: Fn(T) -> C,
{
    ws.queue.clear();
    ws.visited.clear();

    ws.queue.push(WithCost {
        node: s,
        cost: Default::default(),
        heuristic: heuristic(s),
    });

    while let Some(current) = ws.queue.pop() {
        if current.node == e {
            return Some(current.cost);
        }

        if ws.visited.contains(&current.node) {
            continue;
        }

        ws.visited.insert(current.node);

        for (next, edge_weight) in neighbours(current.node) {
            if ws.visited.contains(&next) {
                continue;
            }

            ws.queue.push(WithCost {
                node: next,
                cost: current.cost + edge_weight,
                heuristic: heuristic(next),
            })
        }
    }

    None
}
