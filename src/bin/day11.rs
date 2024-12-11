#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]
use aoc2024::{Solver, Step};
use std::collections::HashMap;

pub fn main() {
    let solver = Solution {};
    solver.solve("day11");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("55312"))],
            Step::Second => vec![], //vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<String> = input[0]
            .split(' ')
            .map(std::string::ToString::to_string)
            .collect();
        match step {
            Step::First => count(&data, 25).to_string(),
            Step::Second => count(&data, 75).to_string(),
        }
    }
}

fn count(data: &[String], blinks: u8) -> u64 {
    let mut data: HashMap<String, u64> = data.iter().fold(HashMap::new(), |mut m, v| {
        *m.entry(v.to_string()).or_default() += 1;
        m
    });
    for _ in 0..blinks {
        let mut new_data: HashMap<String, u64> = HashMap::new();
        for (numb, count) in data {
            if numb == "0" {
                *new_data.entry("1".to_string()).or_default() += count;
            } else if numb.len() % 2 == 0 {
                let v = numb.split_at(numb.len() / 2);
                for v in [v.0.to_string(), v.1.parse::<u64>().unwrap().to_string()] {
                    *new_data.entry(v).or_default() += count;
                }
            } else {
                let v = (numb.parse::<u64>().unwrap() * 2024).to_string();
                *new_data.entry(v).or_default() += count;
            }
        }
        data = new_data;
    }
    data.values().sum()
}
