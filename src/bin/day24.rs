#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};
use std::collections::HashMap;

pub fn main() {
    let solver = Solution {};
    solver.solve("day24");
}
struct Solution {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("2024"))],
            Step::Second => vec![], //vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut iter = input.iter();
        let init: HashMap<String, u64> = iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (name, val) = line.split_once(": ").unwrap();
                (name.to_string(), val.parse().unwrap())
            })
            .collect();
        let gates: HashMap<String, (String, String, GateType)> = iter
            .map(|line| {
                let (input, output) = line.split_once(" -> ").unwrap();
                let mut inputs = input.split(' ');
                let a = inputs.next().unwrap();
                let gate = match inputs.next().unwrap() {
                    "AND" => GateType::And,
                    "OR" => GateType::Or,
                    "XOR" => GateType::Xor,
                    c => panic!("Unknown gate type: {c}"),
                };
                let b = inputs.next().unwrap();
                (output.to_string(), (a.to_string(), b.to_string(), gate))
            })
            .collect();
        match step {
            Step::First => count(&init, &gates).to_string(),
            Step::Second => count2(&init, &gates),
        }
    }
}

fn check(
    init: &HashMap<String, u64>,
    gates: &HashMap<String, (String, String, GateType)>,
) -> Option<(String, String)> {
    let rev_gates: HashMap<(&str, &str, GateType), &str> = gates
        .iter()
        .flat_map(|(output, (a, b, gate))| {
            vec![
                ((a.as_str(), b.as_str(), *gate), output.as_str()),
                ((b.as_str(), a.as_str(), *gate), output.as_str()),
            ]
        })
        .collect();
    let mut carry: Option<&str> = None;
    let inputs = init.keys().filter(|key| key.starts_with('x')).count();
    for input in 0..inputs {
        let x = format!("x{input:02}");
        let y = format!("y{input:02}");
        let z = format!("z{input:02}");
        let xor = rev_gates.get(&(&x, &y, GateType::Xor)).unwrap();
        let and = rev_gates.get(&(&x, &y, GateType::And)).unwrap();
        if let Some(carry_prev) = carry {
            // Z_i = (X_i ^ Y_i) ^ C_(i-1)
            if let Some(carry_xor) = rev_gates.get(&(&xor, &carry_prev, GateType::Xor)) {
                if carry_xor != &z {
                    return Some(((*carry_xor).to_string(), z));
                }
            } else {
                let bad_gate = gates
                    .get(&z)
                    .map(|(a, b, _)| if a == carry_prev { b } else { a })
                    .unwrap();
                return Some(((*bad_gate).to_string(), (*xor).to_string()));
            }
            // C_i = (X_i & Y_i) || ((X_i ^ Y_i) & C_(i-1))
            if let Some(carry_and_xor) = rev_gates.get(&(&xor, &carry_prev, GateType::And)) {
                if let Some(carry_next) = rev_gates.get(&(&and, carry_and_xor, GateType::Or)) {
                    carry = Some(carry_next);
                } else {
                    let bad_gate = rev_gates
                        .keys()
                        .find(|(a, _, gate)| *gate == GateType::Or && a == carry_and_xor)
                        .unwrap();
                    return Some((bad_gate.1.to_string(), (*and).to_string()));
                }
            } else {
                let bad_gate = rev_gates
                    .keys()
                    .find(|(a, _, gate)| *gate == GateType::And && a == &carry_prev)
                    .unwrap();
                return Some((bad_gate.1.to_string(), (*xor).to_string()));
            }
        } else {
            // Z_0 = X_0 ^ Y_0
            if xor != &z {
                return Some(((*xor).to_string(), z));
            }
            // C_0 = X_0 & Y_0
            carry = Some(and);
        }
    }
    None
}

fn count2(
    init: &HashMap<String, u64>,
    gates: &HashMap<String, (String, String, GateType)>,
) -> String {
    let mut gates = gates.clone();
    let mut swaps: Vec<String> = Vec::new();
    while let Some((a, b)) = check(init, &gates) {
        let tmp1 = gates.remove(&a).unwrap();
        let tmp2 = gates.remove(&b).unwrap();
        gates.insert(a.clone(), tmp2);
        gates.insert(b.clone(), tmp1);
        swaps.push(a);
        swaps.push(b);
    }
    swaps.sort_unstable();
    swaps.join(",")
}

fn resolve(
    output: &str,
    init: &HashMap<String, u64>,
    gates: &HashMap<String, (String, String, GateType)>,
) -> u64 {
    if let Some(&val) = init.get(output) {
        val
    } else if let Some((a, b, gate)) = gates.get(output) {
        let a = resolve(a, init, gates);
        let b = resolve(b, init, gates);
        match gate {
            GateType::And => a & b,
            GateType::Or => a | b,
            GateType::Xor => a ^ b,
        }
    } else {
        panic!("Output {output} has no gate");
    }
}

fn count(init: &HashMap<String, u64>, gates: &HashMap<String, (String, String, GateType)>) -> u64 {
    let mut outputs: Vec<&str> = gates
        .keys()
        .filter_map(|key| {
            if key.starts_with('z') {
                Some(key.as_str())
            } else {
                None
            }
        })
        .collect();
    outputs.sort_unstable();
    outputs
        .iter()
        .rev()
        .map(|output| resolve(output, init, gates))
        .fold(0, |acc, val| (acc << 1) + val)
}
