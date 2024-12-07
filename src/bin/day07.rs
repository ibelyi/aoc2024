#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day07");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("3749"))],
            Step::Second => vec![("test0.txt", String::from("11387"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<(u64, Vec<u64>)> = input
            .iter()
            .map(|line| {
                let (val, numbers) = line.split_once(": ").unwrap();
                (
                    val.parse().unwrap(),
                    numbers
                        .split(' ')
                        .map(|numb| numb.parse().unwrap())
                        .rev()
                        .collect(),
                )
            })
            .collect();
        count(&data, step).to_string()
    }
}

fn check(val: u64, numbers: &[u64], step: &Step) -> bool {
    let last = numbers[0];
    if numbers.len() == 1 {
        val == last
    } else {
        let rest = &numbers[1..];
        (val >= last && check(val - last, rest, step))
            || (val % last == 0 && check(val / last, rest, step))
            || (matches!(step, Step::Second) && {
                let last_str = last.to_string();
                let val_str = val.to_string();
                val_str.len() > last_str.len()
                    && val_str
                        .strip_suffix(&last_str)
                        .filter(|val| check(val.parse().unwrap(), rest, step))
                        .is_some()
            })
    }
}

fn count(data: &[(u64, Vec<u64>)], step: &Step) -> u64 {
    data.iter()
        .filter_map(|(val, numbers)| {
            if check(*val, numbers, step) {
                Some(val)
            } else {
                None
            }
        })
        .sum()
}
