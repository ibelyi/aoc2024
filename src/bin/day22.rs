#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};
use std::collections::HashMap;

pub fn main() {
    let solver = Solution {};
    solver.solve("day22");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("37327623"))],
            Step::Second => vec![("test1.txt", String::from("23"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<u64> = input.iter().map(|line| line.parse().unwrap()).collect();
        match step {
            Step::First => count(&data).to_string(),
            Step::Second => count2(&data).to_string(),
        }
    }
}

fn count2(data: &[u64]) -> u64 {
    let mut seq_price: HashMap<Vec<i64>, Vec<u64>> = HashMap::new();
    for &init in data {
        let mut curr = init;
        let mut seq = vec![];
        let mut prev_price = init % 10;
        for _ in 0..4 {
            curr = find_next(curr);
            let price = curr % 10;
            seq.push(i64::try_from(price).unwrap() - i64::try_from(prev_price).unwrap());
            prev_price = price;
        }
        let mut local: HashMap<Vec<i64>, u64> = HashMap::new();
        local.insert(seq.clone(), prev_price);
        for _ in 4..2000 {
            curr = find_next(curr);
            let price = curr % 10;
            seq.remove(0);
            seq.push(i64::try_from(price).unwrap() - i64::try_from(prev_price).unwrap());
            prev_price = price;
            if local.contains_key(&seq) {
                continue;
            }
            local.insert(seq.clone(), prev_price);
        }
        for entry in local {
            seq_price.entry(entry.0).or_default().push(entry.1);
        }
    }
    seq_price
        .values()
        .map(|vals| vals.iter().sum())
        .max()
        .unwrap()
}

fn find_next(mut curr: u64) -> u64 {
    curr = ((curr * 64) ^ curr) % 16_777_216;
    curr = ((curr / 32) ^ curr) % 16_777_216;
    curr = ((curr * 2048) ^ curr) % 16_777_216;
    curr
}

fn count(data: &[u64]) -> u64 {
    data.iter()
        .map(|&init| (0..2000).fold(init, |init, _| find_next(init)))
        .sum()
}
