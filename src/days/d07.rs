fn check_equation_p1(test_result: i64, vals: &Vec<i64>, stack: &mut Vec<(i64, usize)>) -> bool {
    stack.clear();
    stack.push((test_result, 0));

    while let Some((target, i)) = stack.pop() {
        if i == vals.len() && target == 0 {
            return true;
        }
        if let Some(x) = vals.get(i) {
            if target - x >= 0 {
                stack.push((target - x, i + 1));
            }
            if target % x == 0 {
                stack.push((target / x, i + 1));
            }
        }
    }

    false
}

fn remove_suffix(a: i64, b: i64) -> Option<i64> {
    let mut scale = 1;
    while scale <= b {
        scale *= 10
    }
    match a % scale {
        x if x == b => Some(a / scale),
        _ => None,
    }
}

fn check_equation_p2(test_result: i64, vals: &Vec<i64>, stack: &mut Vec<(i64, usize)>) -> bool {
    stack.clear();
    stack.push((test_result, 0));

    while let Some((target, i)) = stack.pop() {
        if i == vals.len() && target == 0 {
            return true;
        }
        if i < vals.len() {
            let x = vals.get(i).unwrap();
            if target - x >= 0 {
                stack.push((target - x, i + 1));
            }
            if target % x == 0 {
                stack.push((target / x, i + 1));
            }
            if let Some(new_target) = remove_suffix(target, *x) {
                stack.push((new_target, i + 1));
            }
        }
    }

    false
}

fn solve(input: &str, check_fn: impl Fn(i64, &Vec<i64>, &mut Vec<(i64, usize)>) -> bool) -> i64 {
    let mut vals: Vec<i64> = vec![];
    let mut to_check_stack: Vec<(i64, usize)> = vec![];

    let result: i64 = input
        .lines()
        .filter_map(|l| {
            let mut iter = l.split(":");
            let testn: i64 = iter.next().unwrap().parse().unwrap();

            vals.clear();
            vals.extend(
                iter.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .rev(),
            );

            match check_fn(testn, &vals, &mut to_check_stack) {
                true => Some(testn),
                false => None,
            }
        })
        .sum();

    result
}

pub fn part1(input: &str) -> i64 {
    solve(input, check_equation_p1)
}

pub fn part2(input: &str) -> i64 {
    solve(input, check_equation_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 3749)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 11387)
    }
}
