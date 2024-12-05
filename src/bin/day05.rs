#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};
use std::{cmp::Ordering, collections::HashSet};

pub fn main() {
    let solver = Solution {};
    solver.solve("day05");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("143"))],
            Step::Second => vec![("test0.txt", String::from("123"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut rules: HashSet<(u64, u64)> = HashSet::new();
        let mut updates = vec![];
        let mut iter = input.iter();
        for line in iter.by_ref() {
            if line.is_empty() {
                break;
            }
            let (a, b) = line.split_once('|').unwrap();
            rules.insert((a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()));
        }
        for line in iter {
            updates.push(
                line.split(',')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            );
        }
        match step {
            Step::First => count(&rules, &updates, true).to_string(),
            Step::Second => count(&rules, &updates, false).to_string(),
        }
    }
}

fn count(rules: &HashSet<(u64, u64)>, updates: &[Vec<u64>], good: bool) -> u64 {
    updates
        .iter()
        .filter_map(|update| {
            let mut sorted = update.clone();
            sorted.sort_by(|&a, &b| {
                if rules.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            if good == (&sorted == update) {
                Some(sorted[sorted.len() / 2])
            } else {
                None
            }
        })
        .sum()
}
