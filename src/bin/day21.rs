#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]
use aoc2024::{Solver, Step};
use std::collections::HashMap;

pub fn main() {
    let solver = Solution {};
    solver.solve("day21");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("126384"))],
            Step::Second => vec![], //vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
        match step {
            Step::First => count(&data, 2).to_string(),
            Step::Second => count(&data, 25).to_string(),
        }
    }
}

fn keypad_loc(from: char, numeric: bool) -> (usize, usize) {
    match from {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (if numeric { 3 } else { 0 }, 2),
        '^' => (0, 1),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        c => panic!("invalid keypad char {c:#}"),
    }
}

fn sequence(from: char, to: char, numeric: bool) -> Vec<char> {
    let from = keypad_loc(from, numeric);
    let to = keypad_loc(to, numeric);
    let mut res = Vec::new();
    if numeric && from.0 == 3 && to.1 == 0 {
        res.extend(vec!['^'; from.0 - to.0]);
        res.extend(vec!['<'; from.1 - to.1]);
    } else if numeric && from.1 == 0 && to.0 == 3 {
        res.extend(vec!['>'; to.1 - from.1]);
        res.extend(vec!['v'; to.0 - from.0]);
    } else if !numeric && from.0 == 0 && to.1 == 0 {
        res.extend(vec!['v']);
        res.extend(vec!['<'; from.1 - to.1]);
    } else if !numeric && from.1 == 0 && to.0 == 0 {
        res.extend(vec!['>'; to.1 - from.1]);
        res.extend(vec!['^']);
    } else {
        if from.1 > to.1 {
            res.extend(vec!['<'; from.1 - to.1]);
        }
        if from.0 < to.0 {
            res.extend(vec!['v'; to.0 - from.0]);
        } else {
            res.extend(vec!['^'; from.0 - to.0]);
        }
        if from.1 < to.1 {
            res.extend(vec!['>'; to.1 - from.1]);
        }
    }
    res.push('A');
    res
}

fn line2map(line: &[char]) -> HashMap<(char, char), u64> {
    let mut count: HashMap<(char, char), u64> = HashMap::new();
    let mut prev = 'A';
    for &c in line {
        *count.entry((prev, c)).or_default() += 1;
        prev = c;
    }
    count
}

fn controller(line: &[char], level: usize) -> u64 {
    let mut result = Vec::new();
    let mut from = 'A';
    for &c in line {
        result.extend(sequence(from, c, true));
        from = c;
    }
    let cost: HashMap<(char, char), HashMap<(char, char), u64>> = ['<', 'v', '>', '^', 'A']
        .iter()
        .flat_map(|&from| {
            ['<', 'v', '>', '^', 'A']
                .iter()
                .map(move |&to| ((from, to), line2map(&sequence(from, to, false))))
        })
        .collect();
    let mut count = line2map(&result);
    for _ in 0..level {
        let mut new_count = HashMap::new();
        for (tr, val) in count {
            for (&tr2, &val2) in cost.get(&tr).unwrap() {
                *new_count.entry(tr2).or_default() += val * val2;
            }
        }
        count = new_count;
    }
    count.values().sum()
}

fn count(data: &[Vec<char>], level: usize) -> u64 {
    data.iter()
        .map(|code| {
            code.iter()
                .take(3)
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
                * controller(code, level)
        })
        .sum()
}
