#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]
use std::collections::HashSet;

use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day18");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("22"))],
            Step::Second => vec![("test0.txt", String::from("6,1"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<(usize, usize)> = input
            .iter()
            .map(|line| {
                let b = line.split_once(',').unwrap();
                (b.0.parse().unwrap(), b.1.parse().unwrap())
            })
            .collect();
        let (limit, size) = if input.len() < 30 {
            (12, 7)
        } else {
            (1024, 71)
        };
        match step {
            Step::First => count(&data[0..limit], size).to_string(),
            Step::Second => count2(&data, size),
        }
    }
}

fn count2(data: &[(usize, usize)], size: usize) -> String {
    let mut tried = (0, data.len() - 1);
    while tried.0 + 1 != tried.1 {
        let mid = (tried.0 + tried.1) / 2;
        if count(&data[0..mid], size) == usize::MAX {
            tried = (tried.0, mid);
        } else {
            tried = (mid, tried.1);
        }
    }
    format!("{},{}", data[tried.0].0, data[tried.0].1)
}

fn count(data: &[(usize, usize)], size: usize) -> usize {
    let mut result = vec![vec![usize::MAX; size]; size];
    let data: HashSet<(usize, usize)> = data.iter().copied().collect();
    result[0][0] = 0;
    let mut changes = true;
    while changes {
        changes = false;
        for x in 0..size {
            for y in 0..size {
                if result[y][x] != usize::MAX {
                    let mut dist = vec![];
                    if y > 0 {
                        dist.push((x, y - 1));
                    }
                    if y < size - 1 {
                        dist.push((x, y + 1));
                    }
                    if x > 0 {
                        dist.push((x - 1, y));
                    }
                    if x < size - 1 {
                        dist.push((x + 1, y));
                    }
                    for (x1, y1) in dist {
                        if !data.contains(&(x1, y1)) && result[y1][x1] > result[y][x] + 1 {
                            result[y1][x1] = result[y][x] + 1;
                            changes = true;
                        }
                    }
                }
            }
        }
    }
    result[size - 1][size - 1]
}
