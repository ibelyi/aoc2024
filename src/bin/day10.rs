#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};
use std::collections::HashSet;

pub fn main() {
    let solver = Solution {};
    solver.solve("day10");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("36"))],
            Step::Second => vec![("test0.txt", String::from("81"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<u32>> = input
            .iter()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        count(&data, step).to_string()
    }
}

fn count_up(head: (usize, usize), data: &[Vec<u32>], count: &mut HashSet<(usize, usize)>) -> usize {
    let curr = data[head.0][head.1];
    if curr == 9 {
        count.insert(head);
        return 1;
    }
    let mut result = 0;
    if head.0 > 0 && data[head.0 - 1][head.1] == curr + 1 {
        result += count_up((head.0 - 1, head.1), data, count);
    }
    if head.0 < data.len() - 1 && data[head.0 + 1][head.1] == curr + 1 {
        result += count_up((head.0 + 1, head.1), data, count);
    }
    if head.1 > 0 && data[head.0][head.1 - 1] == curr + 1 {
        result += count_up((head.0, head.1 - 1), data, count);
    }
    if head.1 < data[0].len() - 1 && data[head.0][head.1 + 1] == curr + 1 {
        result += count_up((head.0, head.1 + 1), data, count);
    }
    result
}

fn count(data: &[Vec<u32>], step: &Step) -> usize {
    data.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(x, &v)| if v == 0 { Some((y, x)) } else { None })
        })
        .map(|head| match step {
            Step::First => {
                let mut count = HashSet::new();
                count_up(head, data, &mut count);
                count.len()
            }
            Step::Second => count_up(head, data, &mut HashSet::new()),
        })
        .sum()
}
