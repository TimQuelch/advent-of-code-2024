use ndarray::{s, Array2};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Val {
    X,
    M,
    A,
    S,
}

fn build_grid(input: &str) -> Array2<Val> {
    let ncols = input.lines().next().unwrap().len();
    let nrows = ((input.len() + 1) / ncols) - 1;

    Array2::from_shape_vec(
        (nrows, ncols),
        input
            .chars()
            .filter_map(|c| match c {
                'X' => Some(Val::X),
                'M' => Some(Val::M),
                'A' => Some(Val::A),
                'S' => Some(Val::S),
                _ => None,
            })
            .collect(),
    )
    .unwrap()
}

const P1_MATCH: [Val; 4] = [Val::X, Val::M, Val::A, Val::S];
const P1_REV_MATCH: [Val; 4] = [Val::S, Val::A, Val::M, Val::X];

fn p1_check_match<I>(iter: I) -> bool
where
    I: Iterator<Item = Val> + Clone,
{
    iter.clone().eq(P1_MATCH.into_iter()) || iter.eq(P1_REV_MATCH.into_iter())
}

pub fn part1(input: &str) -> i32 {
    let grid = build_grid(input);

    let horizontal = grid
        .windows((1, 4))
        .into_iter()
        .filter(|m| p1_check_match(m.into_iter().cloned()))
        .count();

    let vertical = grid
        .t()
        .windows((1, 4))
        .into_iter()
        .filter(|m| p1_check_match(m.into_iter().cloned()))
        .count();

    let diags: usize = grid
        .windows((4, 4))
        .into_iter()
        .map(|w| {
            let diag = p1_check_match(w.diag().into_iter().cloned());
            let antidiag = p1_check_match(w.slice(s![..,..;-1]).diag().into_iter().cloned());
            [diag, antidiag].into_iter().filter(|x| *x).count()
        })
        .sum();

    return (horizontal + vertical + diags).try_into().unwrap();
}

const P2_MATCH: [Val; 3] = [Val::M, Val::A, Val::S];
const P2_REV_MATCH: [Val; 3] = [Val::S, Val::A, Val::M];

fn p2_check_match<I>(iter: I) -> bool
where
    I: Iterator<Item = Val> + Clone,
{
    iter.clone().eq(P2_MATCH.into_iter()) || iter.eq(P2_REV_MATCH.into_iter())
}

pub fn part2(input: &str) -> i32 {
    let grid = build_grid(input);

    grid.windows((3, 3))
        .into_iter()
        .filter(|w| {
            let diag = p2_check_match(w.diag().into_iter().cloned());
            let antidiag = p2_check_match(w.slice(s![..,..;-1]).diag().into_iter().cloned());
            diag && antidiag
        })
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 18)
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 9)
    }
}
