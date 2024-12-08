#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
use aoc2024::{Solver, Step};
use std::collections::{HashMap, HashSet};

pub fn main() {
    let solver = Solution {};
    solver.solve("day08");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("14"))],
            Step::Second => vec![("test0.txt", String::from("34"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let limit = (input.len() as i64, input[0].len() as i64);
        let mut data: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
        for (y, line) in input.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char != '.' {
                    data.entry(char).or_default().push((y as i64, x as i64));
                }
            }
        }
        count(&data, &limit, step).to_string()
    }
}

fn antinodes1(one: (i64, i64), two: (i64, i64), limit: &(i64, i64)) -> Vec<(i64, i64)> {
    [
        (2 * one.0 - two.0, 2 * one.1 - two.1),
        (2 * two.0 - one.0, 2 * two.1 - one.1),
    ]
    .into_iter()
    .filter(|(y, x)| *y >= 0 && *x >= 0 && *y < limit.0 && *x < limit.1)
    .collect()
}

fn antinodes2(one: (i64, i64), two: (i64, i64), limit: &(i64, i64)) -> Vec<(i64, i64)> {
    let mut result = vec![one];
    for dir in [-1, 1] {
        for dist in 1..(limit.0 + limit.1) {
            let next = (
                dir * dist * (two.0 - one.0) + one.0,
                dir * dist * (two.1 - one.1) + one.1,
            );
            if next.0 < 0 || next.1 < 0 || next.0 >= limit.0 || next.1 >= limit.1 {
                break;
            }
            result.push(next);
        }
    }
    result
}

fn count(data: &HashMap<char, Vec<(i64, i64)>>, limit: &(i64, i64), step: &Step) -> usize {
    data.iter()
        .flat_map(|(_, locs)| {
            (0..locs.len())
                .flat_map(|i| {
                    ((i + 1)..locs.len())
                        .flat_map(move |j| match step {
                            Step::First => antinodes1(locs[i], locs[j], limit),
                            Step::Second => antinodes2(locs[i], locs[j], limit),
                        })
                        .collect::<Vec<(i64, i64)>>()
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .collect::<HashSet<(i64, i64)>>()
        .len()
}
