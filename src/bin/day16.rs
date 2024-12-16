#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]
use aoc2024::{Solver, Step};
use std::collections::HashSet;

pub fn main() {
    let solver = Solution {};
    solver.solve("day16");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![
                ("test0.txt", String::from("7036")),
                ("test1.txt", String::from("11048")),
            ],
            Step::Second => vec![
                ("test0.txt", String::from("45")),
                ("test1.txt", String::from("64")),
            ],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
        count(&data, step).to_string()
    }
}

fn rot_cost(i: usize, j: usize) -> usize {
    let res = (i + 4 - j) % 4;
    (res - 2 * (res / 3)) * 1000
}

fn count(data: &[Vec<char>], step: &Step) -> usize {
    let mut count: Vec<Vec<[usize; 4]>> = vec![vec![[usize::MAX; 4]; data[0].len()]; data.len()];
    count[data.len() - 2][1] = [0, 1000, 2000, 1000];
    let mut changed = true;
    while changed {
        changed = false;
        for y in 1..data.len() - 1 {
            for x in 1..data[0].len() - 1 {
                if data[y][x] != '#' && count[y][x][0] != usize::MAX {
                    for (j, (y1, x1)) in [(y, x + 1), (y - 1, x), (y, x - 1), (y + 1, x)]
                        .into_iter()
                        .enumerate()
                    {
                        if data[y1][x1] != '#' {
                            let min = count[y][x]
                                .iter()
                                .enumerate()
                                .map(|(i, val)| val + rot_cost(i, j))
                                .min()
                                .unwrap();
                            for (i, val) in count[y1][x1].iter_mut().enumerate() {
                                let cost = min + rot_cost(i, j) + 1;
                                if *val > cost {
                                    *val = cost;
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let (index, min) = count[1][data[0].len() - 2]
        .iter()
        .copied()
        .enumerate()
        .min_by(|(_, va), (_, vb)| va.cmp(vb))
        .unwrap();
    match step {
        Step::First => min,
        Step::Second => {
            let mut best: HashSet<(usize, usize)> = HashSet::new();
            best_spots(&count, (1, data[0].len() - 2), index, &mut best);
            best.len()
        }
    }
}

fn best_spots(
    count: &[Vec<[usize; 4]>],
    loc: (usize, usize),
    index: usize,
    best: &mut HashSet<(usize, usize)>,
) {
    best.insert(loc);
    for (i, (y, x)) in [
        (loc.0, loc.1 - 1),
        (loc.0 + 1, loc.1),
        (loc.0, loc.1 + 1),
        (loc.0 - 1, loc.1),
    ]
    .into_iter()
    .enumerate()
    {
        if count[y][x][i] != usize::MAX
            && count[y][x][i] + rot_cost(index, i) + 1 == count[loc.0][loc.1][index]
        {
            best_spots(count, (y, x), i, best);
        }
    }
}
