use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day02");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("2"))],
            Step::Second => vec![("test0.txt", String::from("4"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<u64>> = input
            .iter()
            .map(|line| {
                line.split(' ')
                    .map(|data| data.parse::<u64>().expect("Not a number"))
                    .collect()
            })
            .collect();
        match step {
            Step::First => self.count(&data, false).to_string(),
            Step::Second => self.count(&data, true).to_string(),
        }
    }
}

fn check(report: &[u64]) -> bool {
    if report.len() < 2 {
        return true;
    }
    let dir = report[0] < report[1];
    for i in 1..report.len() {
        if dir == (report[i - 1] > report[i]) {
            return false;
        }
        let diff = if dir {
            report[i] - report[i - 1]
        } else {
            report[i - 1] - report[i]
        };
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

impl Solution {
    fn count(&self, data: &[Vec<u64>], fix: bool) -> usize {
        data.iter()
            .filter(|report| {
                if check(report) {
                    return true;
                }
                if !fix {
                    return false;
                }
                for i in 0..report.len() {
                    let mut copy = report.to_vec();
                    copy.remove(i);
                    if check(&copy) {
                        return true;
                    }
                }
                false
            })
            .count()
    }
}
