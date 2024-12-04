#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day04");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("18"))],
            Step::Second => vec![("test0.txt", String::from("9"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
        match step {
            Step::First => count(&data).to_string(),
            Step::Second => count2(&data).to_string(),
        }
    }
}

fn count2(data: &[Vec<char>]) -> u64 {
    let mut count = 0;
    for x in 1..data.len() - 1 {
        for y in 1..data[0].len() - 1 {
            if data[x][y] == 'A'
                && ((data[x - 1][y - 1] == 'M' && data[x + 1][y + 1] == 'S')
                    || (data[x - 1][y - 1] == 'S' && data[x + 1][y + 1] == 'M'))
                && ((data[x - 1][y + 1] == 'M' && data[x + 1][y - 1] == 'S')
                    || (data[x - 1][y + 1] == 'S' && data[x + 1][y - 1] == 'M'))
            {
                count += 1;
            }
        }
    }
    count
}

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

fn count(data: &[Vec<char>]) -> u64 {
    let mut count = 0;
    for x in 0..data.len() {
        for y in 0..data[0].len() {
            if data[x][y] == WORD[0] {
                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let l = i64::try_from(WORD.len()).unwrap();
                        let xi = i64::try_from(x).unwrap();
                        let end = xi + dx * (l - 1);
                        if end < 0 || usize::try_from(end).unwrap() >= data.len() {
                            continue;
                        }
                        let yi = i64::try_from(y).unwrap();
                        let end = yi + dy * (l - 1);
                        if end < 0 || usize::try_from(end).unwrap() >= data[0].len() {
                            continue;
                        }
                        let mut found = true;
                        for (i, c) in WORD.into_iter().enumerate() {
                            let ii = i64::try_from(i).unwrap();
                            if data[usize::try_from(i64::try_from(x).unwrap() + dx * ii).unwrap()]
                                [usize::try_from(i64::try_from(y).unwrap() + dy * ii).unwrap()]
                                != c
                            {
                                found = false;
                                break;
                            }
                        }
                        if found {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}
