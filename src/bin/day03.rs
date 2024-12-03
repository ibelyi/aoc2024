use aoc2024::{Solver, Step};
use regex::Regex;

pub fn main() {
    let solver = Solution {};
    solver.solve("day03");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("161"))],
            Step::Second => vec![("test1.txt", String::from("48"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        match step {
            Step::First => {
                let reg = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("Bad expression");
                input
                    .iter()
                    .map(|line| {
                        reg.captures_iter(line)
                            .map(|c| {
                                c.get(1).unwrap().as_str().parse::<u64>().unwrap()
                                    * c.get(2).unwrap().as_str().parse::<u64>().unwrap()
                            })
                            .sum::<u64>()
                    })
                    .sum::<u64>()
            }
            Step::Second => {
                let reg = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|don't\(\)|do\(\)")
                    .expect("Bad expression");
                let mut count = true;
                input
                    .iter()
                    .map(|line| {
                        reg.captures_iter(line)
                            .map(|c| {
                                let m = c.get(0).unwrap().as_str();
                                if m.starts_with("do") {
                                    count = m.len() == 4;
                                    0
                                } else if count {
                                    c.get(1).unwrap().as_str().parse::<u64>().unwrap()
                                        * c.get(2).unwrap().as_str().parse::<u64>().unwrap()
                                } else {
                                    0
                                }
                            })
                            .sum::<u64>()
                    })
                    .sum::<u64>()
            }
        }
        .to_string()
    }
}
