#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]
use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day13");
}
struct Solution {}

struct Claw {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("480"))],
            Step::Second => vec![], //vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut data: Vec<Claw> = Vec::new();
        let mut button_a = (0, 0);
        let mut button_b = (0, 0);
        for line in input {
            if let Some(button) = line.strip_prefix("Button A: X+") {
                let button = button.split_once(", Y+").unwrap();
                button_a = (button.0.parse().unwrap(), button.1.parse().unwrap());
            } else if let Some(button) = line.strip_prefix("Button B: X+") {
                let button = button.split_once(", Y+").unwrap();
                button_b = (button.0.parse().unwrap(), button.1.parse().unwrap());
            } else if let Some(prize) = line.strip_prefix("Prize: X=") {
                let prize = prize.split_once(", Y=").unwrap();
                let extra = match step {
                    Step::First => 0i64,
                    Step::Second => 10_000_000_000_000i64,
                };
                data.push(Claw {
                    button_a,
                    button_b,
                    prize: (
                        extra + prize.0.parse::<i64>().unwrap(),
                        extra + prize.1.parse::<i64>().unwrap(),
                    ),
                });
            } else if !line.is_empty() {
                panic!("Unexpected non empty line: {line}");
            }
        }
        count(&data).to_string()
    }
}

fn prize_price(claw: &Claw) -> i64 {
    let b = (claw.prize.0 * claw.button_a.1 - claw.prize.1 * claw.button_a.0)
        / (claw.button_b.0 * claw.button_a.1 - claw.button_b.1 * claw.button_a.0);
    let a = (claw.prize.0 * claw.button_b.1 - claw.prize.1 * claw.button_b.0)
        / (claw.button_a.0 * claw.button_b.1 - claw.button_a.1 * claw.button_b.0);
    let loc = (
        claw.button_a.0 * a + claw.button_b.0 * b,
        claw.button_a.1 * a + claw.button_b.1 * b,
    );
    if loc == claw.prize {
        3 * a + b
    } else {
        0
    }
}

fn count(data: &[Claw]) -> i64 {
    data.iter().map(prize_price).sum()
}
