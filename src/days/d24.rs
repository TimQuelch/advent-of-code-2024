use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug, PartialEq, Eq)]
struct Gate<'a> {
    gate_type: GateType,
    in1: &'a str,
    in2: &'a str,
}

fn resolve<'a>(
    mut cache: &mut HashMap<&'a str, bool>,
    gates: &HashMap<&'a str, Gate<'a>>,
    gate: &'a str,
) -> bool {
    if let Some(initial_value) = cache.get(gate) {
        return *initial_value;
    }

    let outputting_gate = gates.get(gate).expect("output gate not found");

    let a = resolve(&mut cache, &gates, outputting_gate.in1);
    let b = resolve(&mut cache, &gates, outputting_gate.in2);

    let result = match outputting_gate.gate_type {
        GateType::And => a && b,
        GateType::Or => a || b,
        GateType::Xor => a ^ b,
    };

    cache.insert(gate, result);
    result
}

fn parse(input: &str) -> (HashMap<&str, bool>, HashMap<&str, Gate>) {
    let (init_input, gates_input) = input.split_once("\n\n").unwrap();
    let init = init_input
        .lines()
        .map(|i| {
            let (gate, value) = i.split_once(": ").unwrap();
            (gate, value == "1")
        })
        .collect::<HashMap<_, _>>();

    let gates = gates_input
        .lines()
        .map(|g| {
            let mut iter = g.split(' ');
            let in1 = iter.next().unwrap();
            let gate_type_raw = iter.next().unwrap();
            let in2 = iter.next().unwrap();
            let out = iter.nth(1).unwrap();

            let gate_type = match gate_type_raw {
                "AND" => GateType::And,
                "OR" => GateType::Or,
                "XOR" => GateType::Xor,
                _ => panic!("invalid gate type {}", gate_type_raw),
            };

            (
                out,
                Gate {
                    in1,
                    in2,
                    gate_type,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    (init, gates)
}

pub fn part1(input: &str) -> i64 {
    let (mut init, gates) = parse(input);

    let result = gates
        .keys()
        .filter_map(|&g| match g {
            o if o.starts_with('z') => Some(o),
            _ => None,
        })
        .sorted()
        .rev()
        .map(|g| resolve(&mut init, &gates, g))
        .fold(0, |acc, b| (acc << 1) + (b as u64));

    result.try_into().unwrap()
}

pub fn part2<'a>(input: &'a str) -> i64 {
    // Full adder
    // 1. (X,  Y) XOR -> A
    // 2. (X,  Y) AND -> B
    // 3. (A, CI) XOR -> Z
    // 4. (A, CI) AND -> C
    // 5. (C,  B)  OR -> CO

    let gates = input
        .split_once("\n\n")
        .unwrap()
        .1
        .lines()
        .map(|g| {
            let mut iter = g.split(' ');
            let in1 = iter.next().unwrap();
            let gate_type_raw = iter.next().unwrap();
            let in2 = iter.next().unwrap();
            let out = iter.nth(1).unwrap();

            let gate_type = match gate_type_raw {
                "AND" => GateType::And,
                "OR" => GateType::Or,
                "XOR" => GateType::Xor,
                _ => panic!("invalid gate type {}", gate_type_raw),
            };

            (gate_type, in1, in2, out)
        })
        .collect::<Vec<_>>();

    let find_by_in = |input: &'a str| {
        gates
            .iter()
            .filter(move |(_, in1, in2, _)| *in1 == input || *in2 == input)
    };

    let broken = gates
        .iter()
        .filter_map(|(ty, in1, in2, out)| {
            // If the gate is an XOR then either its inputs are x?? and y?? or the output is z??. If
            // it is not a z?? output then the outputs must be an XOR and an AND gate
            if *ty == GateType::Xor && !out.starts_with('z') {
                if !((in1.starts_with('x') && in2.starts_with('y'))
                    || (in1.starts_with('y') && in2.starts_with('x')))
                {
                    return Some(out);
                }

                let mut out_gates = find_by_in(out).map(|(ty, _, _, _)| ty);
                let gates = out_gates.next().zip(out_gates.next());
                if gates.map_or(true, |(a, b)| {
                    *a == *b
                        || !(*a == GateType::Xor || *a == GateType::And)
                        || !(*b == GateType::Xor || *b == GateType::And)
                }) {
                    return Some(out);
                }
            }

            // If the gate is an AND then the output must be a single OR gate.
            // The exception is the LSB
            if *ty == GateType::And && (*in1 != "x00" && *in2 != "x00") {
                let mut out_gates = find_by_in(out).map(|(ty, _, _, _)| ty);
                if !(out_gates.next().map_or(false, |ty| *ty == GateType::Or)
                    && out_gates.next().is_none())
                {
                    return Some(out);
                }
            }

            // If the gate is an OR then the output must be a XOR gate and a AND gate.
            // The exception is the MSB
            if *ty == GateType::Or && *out != "z45" {
                let mut out_gates = find_by_in(out).map(|(ty, _, _, _)| ty);
                let gates = out_gates.next().zip(out_gates.next());
                if gates.map_or(true, |(a, b)| {
                    *a == *b
                        || !(*a == GateType::Xor || *a == GateType::And)
                        || !(*b == GateType::Xor || *b == GateType::And)
                }) {
                    return Some(out);
                }
            }

            None
        })
        .sorted()
        .join(",");

    // Uncomment to print the required answer
    // println!("{}", broken);

    // Return a numeric answer for the templates
    broken.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    const EXAMPLE2: &str = "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    #[test]
    fn example_part1() {
        let result = part1(EXAMPLE.trim());
        assert_eq!(result, 4)
    }

    #[test]
    fn example_part1_2() {
        let result = part1(EXAMPLE2.trim());
        assert_eq!(result, 2024)
    }
}
