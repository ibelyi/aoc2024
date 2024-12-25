#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day25");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("3"))],
            Step::Second => vec![], //vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut keys: Vec<Vec<u8>> = Vec::new();
        let mut locks: Vec<Vec<u8>> = Vec::new();
        let mut is_key = false;
        let mut pins = [0; 5];
        for (index, line) in input.iter().enumerate() {
            if index % 8 == 0 {
                is_key = line == ".....";
            } else if index % 8 < 6 {
                line.chars().enumerate().for_each(|(i, c)| {
                    if c == '#' {
                        pins[i] += 1;
                    }
                });
            } else if index % 8 == 6 {
                if is_key {
                    keys.push(pins.to_vec());
                } else {
                    locks.push(pins.to_vec());
                }
                pins = [0; 5];
            }
        }
        match step {
            Step::First => count(&keys, &locks).to_string(),
            Step::Second => String::from("Merry Christmas!"),
        }
    }
}

fn count(keys: &[Vec<u8>], locks: &[Vec<u8>]) -> usize {
    keys.iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| key.iter().zip(lock.iter()).all(|(k, l)| k + l < 6))
                .count()
        })
        .sum()
}
