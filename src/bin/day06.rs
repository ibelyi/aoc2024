#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};
use std::collections::HashSet;

pub fn main() {
    let solver = Solution {};
    solver.solve("day06");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("41"))],
            Step::Second => vec![("test0.txt", String::from("6"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut guard = (0, 0);
        let limit = (
            i64::try_from(input.len()).unwrap(),
            i64::try_from(input[0].len()).unwrap(),
        );
        let mut map: HashSet<(i64, i64)> = HashSet::new();
        for (y, line) in input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let spot = (i64::try_from(y).unwrap(), i64::try_from(x).unwrap());
                if c == '#' {
                    map.insert(spot);
                } else if c == '^' {
                    guard = spot;
                }
            }
        }
        count(&map, limit, guard, step).to_string()
    }
}

const DIFF: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn count(
    map: &HashSet<(i64, i64)>,
    limit: (i64, i64),
    orig_guard: (i64, i64),
    step: &Step,
) -> usize {
    let mut guard = orig_guard;
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut dir = 0;
    loop {
        let next = (guard.0 + DIFF[dir].0, guard.1 + DIFF[dir].1);
        if next.0 < 0 || next.0 >= limit.0 || next.1 < 0 || next.1 >= limit.1 {
            visited.insert(guard);
            break;
        }
        if map.contains(&next) {
            dir = (dir + 1) % 4;
        } else {
            visited.insert(guard);
            guard = next;
        }
    }
    match step {
        Step::First => visited.len(),
        Step::Second => visited
            .into_iter()
            .filter(|&(y, x)| {
                if orig_guard == (y, x) {
                    return false;
                }
                let mut dir = 0;
                let mut guard = orig_guard;
                let mut visited: HashSet<(usize, (i64, i64))> = HashSet::new();
                let mut map = map.clone();
                map.insert((y, x));
                loop {
                    let next = (guard.0 + DIFF[dir].0, guard.1 + DIFF[dir].1);
                    if visited.contains(&(dir, next)) {
                        return true;
                    }
                    if next.0 < 0 || next.0 >= limit.0 || next.1 < 0 || next.1 >= limit.1 {
                        return false;
                    }
                    if map.contains(&next) {
                        visited.insert((dir, guard));
                        dir = (dir + 1) % 4;
                    } else {
                        visited.insert((dir, guard));
                        guard = next;
                    }
                }
            })
            .count(),
    }
}
