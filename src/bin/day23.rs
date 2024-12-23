#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]
use std::collections::{HashMap, HashSet};

use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day23");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("7"))],
            Step::Second => vec![("test0.txt", String::from("co,de,ka,ta"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<(String, String)> = input
            .iter()
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                (a.to_string(), b.to_string())
            })
            .collect();
        match step {
            Step::First => count(&data).to_string(),
            Step::Second => count2(&data),
        }
    }
}

fn count2(data: &[(String, String)]) -> String {
    let mut cons: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in data {
        cons.entry(a).or_default().insert(b);
        cons.entry(b).or_default().insert(a);
    }
    let mut res: HashMap<Vec<&str>, HashSet<&str>> = cons
        .iter()
        .map(|(&a, others)| (vec![a], others.clone()))
        .collect();
    loop {
        let mut next: HashMap<Vec<&str>, HashSet<&str>> = HashMap::new();
        for (party, others) in &res {
            for a in others {
                let invited: HashSet<&str> =
                    cons.get(a).unwrap().intersection(others).copied().collect();
                if !invited.is_empty() {
                    let mut key = party.clone();
                    key.push(*a);
                    key.sort_unstable();
                    next.insert(key, invited);
                }
            }
        }
        if next.is_empty() {
            break;
        }
        res = next;
    }
    res.into_iter()
        .next()
        .map(|(mut k, v)| {
            k.extend(v.iter().copied());
            k.sort_unstable();
            k
        })
        .unwrap()
        .join(",")
}

fn count(data: &[(String, String)]) -> usize {
    let mut cons: HashMap<&str, Vec<&str>> = HashMap::new();
    for (a, b) in data {
        cons.entry(a).or_default().push(b);
        cons.entry(b).or_default().push(a);
    }
    let mut res: HashSet<Vec<&str>> = HashSet::new();
    for (a, others) in &cons {
        if !a.starts_with('t') || others.len() < 2 {
            continue;
        }
        for b in 0..others.len() {
            for c in (b + 1)..others.len() {
                if cons.get(&others[b]).unwrap().contains(&others[c]) {
                    let mut triplet = vec![a, others[b], others[c]];
                    triplet.sort_unstable();
                    res.insert(triplet);
                }
            }
        }
    }
    res.len()
}
