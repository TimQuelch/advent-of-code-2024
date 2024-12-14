pub fn solve(input: &str, offset: i64) -> i64 {
    input
        .split("\n\n")
        .map(|machine_input| {
            let mut ls = machine_input.lines();
            let a = ls.next().unwrap()[12..].split_once(", Y+").unwrap();
            let b = ls.next().unwrap()[12..].split_once(", Y+").unwrap();
            let x = ls.next().unwrap()[9..].split_once(", Y=").unwrap();
            (
                a.0.parse::<i64>().unwrap(),
                a.1.parse::<i64>().unwrap(),
                b.0.parse::<i64>().unwrap(),
                b.1.parse::<i64>().unwrap(),
                x.0.parse::<i64>().unwrap(),
                x.1.parse::<i64>().unwrap(),
            )
        })
        .map(|(x1, y1, x2, y2, x3, y3)| {
            let x3offset = x3 + offset;
            let y3offset = y3 + offset;

            let det = x1 * y2 - x2 * y1;

            if det == 0 {
                return 0;
            }

            let a = (x3offset * y2 - x2 * y3offset) / det;
            let b = (y3offset * x1 - x3offset * y1) / det;

            if a * x1 + b * x2 == x3offset && a * y1 + b * y2 == y3offset {
                a * 3 + b
            } else {
                0
            }
        })
        .sum::<i64>()
}

pub fn part1(input: &str) -> i64 {
    solve(input, 0)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 10000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 480)
    }
}
