#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]
use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day17");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("4,6,3,5,6,3,5,2,1,0"))],
            Step::Second => vec![], //vec![("test1.txt", String::from("117440"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut iter = input.iter();
        let state: Vec<u64> = ["Register A: ", "Register B: ", "Register C: "]
            .into_iter()
            .map(|prefix| {
                iter.next()
                    .and_then(|line| {
                        line.strip_prefix(prefix)
                            .and_then(|line| line.parse::<u64>().ok())
                    })
                    .unwrap()
            })
            .collect();
        let _ = iter.next();
        let program: Vec<u8> = iter
            .next()
            .and_then(|line| line.strip_prefix("Program: "))
            .unwrap()
            .split(',')
            .map(|v| v.parse::<u8>().unwrap())
            .collect();
        match step {
            Step::First => count(&program, &state),
            Step::Second => count2(&program).to_string(),
        }
    }
}

/*
 My program:
 B = A % 8
 B = B ^ 2
 C = A >> B
 A = A >> 3
 B = B ^ C
 B = B ^ 7
 output B % 8
 loop if A != 0
*/
fn count2(program: &[u8]) -> u64 {
    let mut result = 0;
    for output in (0..program.len()).rev() {
        let expected = to_output(&program[output..]);
        for v in 0..8 {
            let state = vec![(result << 3) + v, 0, 0];
            if count(program, &state) == expected {
                result = (result << 3) + v;
                break;
            }
        }
    }
    result
}

fn combo(op: u8, state: &[u64]) -> u64 {
    if op < 4 {
        u64::from(op)
    } else if op < 7 {
        state[op as usize - 4]
    } else {
        panic!("invalid combo operand {op}")
    }
}

fn to_output(output: &[u8]) -> String {
    output
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join(",")
}

fn count(program: &[u8], state: &[u64]) -> String {
    let mut state = state.to_vec();
    let mut step = 0;
    let mut output: Vec<u8> = Vec::new();
    while step < program.len() {
        match program[step] {
            0 => {
                state[0] >>= combo(program[step + 1], &state);
            }
            1 => {
                state[1] ^= u64::from(program[step + 1]);
            }
            2 => {
                state[1] = combo(program[step + 1], &state) % 8;
            }
            3 => {
                if state[0] != 0 {
                    step = program[step + 1] as usize;
                    continue;
                }
            }
            4 => {
                state[1] ^= state[2];
            }
            5 => {
                output.push(u8::try_from(combo(program[step + 1], &state) % 8).unwrap());
            }
            6 => {
                state[1] = state[0] >> combo(program[step + 1], &state);
            }
            7 => {
                state[2] = state[0] >> combo(program[step + 1], &state);
            }
            c => panic!("Unknown instruction {c}"),
        }
        step += 2;
    }
    to_output(&output)
}
