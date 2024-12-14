use std::collections::HashSet;

fn solve_p1(input: &str, size: [i32; 2]) -> i64 {
    let mut quadrants = [0, 0, 0, 0];

    input
        .lines()
        .map(|l| {
            let (ps, vs) = l.split_once(' ').unwrap();

            let p = ps[2..].split_once(',').unwrap();
            let v = vs[2..].split_once(',').unwrap();

            (
                [p.0.parse::<i32>().unwrap(), p.1.parse::<i32>().unwrap()],
                [v.0.parse::<i32>().unwrap(), v.1.parse::<i32>().unwrap()],
            )
        })
        .map(|(p, v)| {
            [
                (p[0] + 100 * v[0]).rem_euclid(size[0]),
                (p[1] + 100 * v[1]).rem_euclid(size[1]),
            ]
        })
        .for_each(|p| {
            // println!("{:?}", p);
            if p[0] != size[0] / 2 && p[1] != size[1] / 2 {
                match (p[0] < size[0] / 2, p[1] < size[1] / 2) {
                    (false, false) => quadrants[0] += 1,
                    (false, true) => quadrants[1] += 1,
                    (true, false) => quadrants[2] += 1,
                    (true, true) => quadrants[3] += 1,
                }
            }
        });

    quadrants.into_iter().product()
}

pub fn part1(input: &str) -> i64 {
    solve_p1(input, [101, 103])
}

pub fn part2(input: &str) -> i64 {
    let size = [101, 103];
    let mut robots = input
        .lines()
        .map(|l| {
            let (ps, vs) = l.split_once(' ').unwrap();

            let p = ps[2..].split_once(',').unwrap();
            let v = vs[2..].split_once(',').unwrap();

            (
                [p.0.parse::<i32>().unwrap(), p.1.parse::<i32>().unwrap()],
                [v.0.parse::<i32>().unwrap(), v.1.parse::<i32>().unwrap()],
            )
        })
        .collect::<Vec<_>>();

    let mut uniques = HashSet::new();
    let mut i = 0;
    loop {
        uniques.clear();

        robots.iter_mut().for_each(|(p, v)| {
            p[0] = (p[0] + v[0]).rem_euclid(size[0]);
            p[1] = (p[1] + v[1]).rem_euclid(size[1]);

            if i > 8000 {
                uniques.insert(*p);
            }
        });

        i += 1;
        if i > 8000 && robots.len() == uniques.len() {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn example_part1() {
        let result = solve_p1(EXAMPLE.trim(), [11, 7]);
        assert_eq!(result, 12)
    }
}
