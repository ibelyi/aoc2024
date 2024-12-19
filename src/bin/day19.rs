#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let solver = Solution {};
    solver.solve("day19");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("6"))],
            Step::Second => vec![("test0.txt", String::from("16"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut iter = input.iter();

        let towels: Vec<String> = iter
            .next()
            .unwrap()
            .split(", ")
            .map(std::string::ToString::to_string)
            .collect();
        iter.next();
        let patterns: Vec<String> = iter.cloned().collect();
        match step {
            Step::First => count(&towels, &patterns).to_string(),
            Step::Second => count2(&towels, &patterns).to_string(),
        }
    }
}

fn diff_ways(pat: &str, towels: &HashSet<String>, acc: &mut HashMap<String, usize>) -> usize {
    if pat.is_empty() {
        return 1;
    } else if let Some(&val) = acc.get(pat) {
        return val;
    }
    let mut total = 0;
    for len in 1..=pat.len() {
        if towels.contains(&pat[0..len]) {
            total += diff_ways(&pat[len..], towels, acc);
        }
    }
    acc.insert(pat.to_string(), total);
    total
}

fn count2(towels: &[String], patterns: &[String]) -> usize {
    let towels: HashSet<String> = towels.iter().cloned().collect();
    patterns
        .iter()
        .map(|pat| diff_ways(pat, &towels, &mut HashMap::new()))
        .sum()
}

fn count(towels: &[String], patterns: &[String]) -> usize {
    let towels = Regex::new(&format!("^({})+$", towels.to_vec().join("|"))).unwrap();
    patterns.iter().filter(|pat| towels.is_match(pat)).count()
}
