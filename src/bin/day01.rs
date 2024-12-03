use std::collections::HashMap;

use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day01");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("11"))],
            Step::Second => vec![("test0.txt", String::from("31"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut first: Vec<u64> = Vec::new();
        let mut second = Vec::new();
        for line in input {
            let (one, two) = line.split_once("   ").expect("Missing delimiter");
            first.push(one.parse().expect("First number"));
            second.push(two.parse().expect("Second number"))
        }
        match step {
            Step::First => {
                first.sort_unstable();
                second.sort_unstable();
                first
                    .into_iter()
                    .zip(second)
                    .map(|(one, two)| if one > two { one - two } else { two - one })
                    .sum::<u64>()
            }
            Step::Second => {
                let second = second.into_iter().fold(HashMap::new(), |mut count, two| {
                    *count.entry(two).or_default() += 1;
                    count
                });
                first
                    .into_iter()
                    .map(|one| one * second.get(&one).unwrap_or(&0))
                    .sum::<u64>()
            }
        }
        .to_string()
    }
}
