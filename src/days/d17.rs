#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Op {
    fn from(source: u8) -> Self {
        match source {
            0 => Op::Adv,
            1 => Op::Bxl,
            2 => Op::Bst,
            3 => Op::Jnz,
            4 => Op::Bxc,
            5 => Op::Out,
            6 => Op::Bdv,
            7 => Op::Cdv,
            _ => panic!("invalid op code: {}", source),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Machine {
    a: u64,
    b: u64,
    c: u64,
    pointer: usize,
}

impl Machine {
    fn new(a: u64) -> Self {
        Self {
            a,
            b: 0,
            c: 0,
            pointer: 0,
        }
    }
}

fn parse_input(input: &str) -> (Machine, Vec<u8>) {
    let mut ls = input.lines();
    let a = ls.next().unwrap()[12..].parse::<u64>().unwrap();
    let b = ls.next().unwrap()[12..].parse::<u64>().unwrap();
    let c = ls.next().unwrap()[12..].parse::<u64>().unwrap();

    let program: Vec<_> = ls.skip(1).next().unwrap()[9..]
        .bytes()
        .step_by(2)
        .map(|x| x - b'0')
        .collect();

    (
        Machine {
            a,
            b,
            c,
            pointer: 0,
        },
        program,
    )
}

const fn combo(m: &Machine, literal: u8) -> u64 {
    match literal {
        x if x <= 3 => x as u64,
        4 => m.a,
        5 => m.b,
        6 => m.c,
        _ => panic!("invalid op code"),
    }
}

fn run(mut m: Machine, program: &Vec<u8>) -> (Machine, Vec<u8>) {
    let end = program.len();
    let mut out = Vec::new();
    while m.pointer < end - 1 {
        let opcode: Op = program[m.pointer].into();
        let operand = program[m.pointer + 1];
        match opcode {
            Op::Adv => m.a = m.a / (1 << combo(&m, operand)),
            Op::Bxl => m.b = m.b ^ operand as u64,
            Op::Bst => m.b = combo(&m, operand) % 8,
            Op::Jnz => {
                if m.a != 0 {
                    m.pointer = operand as usize
                }
            }
            Op::Bxc => m.b = m.b ^ m.c,
            Op::Out => {
                let x = (combo(&m, operand) % 8) as u8;
                out.push(x);
            }
            Op::Bdv => m.b = m.a / (1 << combo(&m, operand)),
            Op::Cdv => m.c = m.a / (1 << combo(&m, operand)),
        }
        if !(opcode == Op::Jnz && m.a != 0) {
            m.pointer += 2;
        }
    }
    (m, out)
}

pub fn part1(input: &str) -> i64 {
    let (m, program) = parse_input(input);
    let (_, out) = run(m, &program);

    // print as string
    // println!(
    //     "{:?}",
    //     out[1..].iter().fold(out[0].to_string(), |mut acc, x| {
    //         acc.push(',');
    //         acc.push_str(x.to_string().as_str());
    //         acc
    //     })
    // );

    // convert the output to a number because that's what my types expect
    let numeric: i64 = out.iter().fold(0, |acc, &x| acc * 10 + x as i64);
    return numeric;
}

pub fn part2(input: &str) -> i64 {
    let (_, program) = parse_input(input);

    let mut stack = vec![(0, 0)];

    let result = loop {
        let (x, n) = stack.pop().unwrap();
        if n == program.len() {
            break x;
        }
        for to_test in (0..8).rev() {
            let a = (x << 3) + to_test;
            let (_, out) = run(Machine::new(a), &program);
            if out.iter().eq(program.iter().skip(program.len() - n - 1)) {
                stack.push((a, n + 1));
            }
        }
    };
    return result.try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const EXAMPLE_2: &str = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn example_part1() {
        let (m, program) = parse_input(EXAMPLE.trim());
        let (_, out) = run(m, &program);
        assert_eq!(out, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0])
    }

    #[test]
    fn example_part1_parsed() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 4635635210)
    }

    #[test]
    fn example_part1_small_1() {
        let (m, _) = run(
            Machine {
                a: 0,
                b: 0,
                c: 9,
                pointer: 0,
            },
            &vec![2, 6],
        );

        assert_eq!(m.b, 1)
    }

    #[test]
    fn example_part1_small_2() {
        let (_, out) = run(
            Machine {
                a: 10,
                b: 0,
                c: 0,
                pointer: 0,
            },
            &vec![5, 0, 5, 1, 5, 4],
        );
        assert_eq!(out, vec![0, 1, 2])
    }

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE_2.trim());
        assert_eq!(result, 117440)
    }
}
