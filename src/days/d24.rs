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

pub fn part2(input: &str) -> i64 {
    let (_, gates) = parse(input);

    let zkeys = gates.keys().filter(|g| g.starts_with('z')).cloned().sorted().collect::<Vec<_>>();

    // Assume the first bit is correct. (X, Y) XOR -> Z, the carry for this bit is (X, Y) AND -> CO
    assert_eq!(*gates.get(zkeys[0]).unwrap(), Gate { gate_type: GateType::Xor, in1: "y00", in2: "x00" });

    // Full adder
    // (X,  Y) XOR -> A
    // (X,  Y)  OR -> B
    // (A, CI) XOR -> Z
    // (A, CI)  OR -> C
    // (C,  B) AND -> CO

    for &zk in zkeys[1..].iter() {
        let top = gates.get(zk).unwrap();
        // In my input one half of each pair was swapped with a top level xor
        if top.gate_type != GateType::Xor {
            println!("top level of {} is not xor: {:?}", zk, top);

            if (top.gate_type == GateType::And) {
                println!("top level of {} is an and: {:?}", zk, top);
            }

        }
    }


    dbg!(gates.get("z00"));
    dbg!(gates.get("z01"));
    dbg!(gates.get("rvp"));
    dbg!(gates.get("tss"));

    dbg!(gates.get("z02"));
    dbg!(gates.get("tdp"));
    dbg!(gates.get("bcr"));
    dbg!(gates.get("jcr"));
    dbg!(gates.get("ccn"));



    // dbg!(gates.get("z02"));
    // dbg!(gates.get("z03"));

    return 0;
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

    #[test]
    fn example_part2() {
        let result = part2(EXAMPLE.trim());
        assert_eq!(result, 0)
    }
}
