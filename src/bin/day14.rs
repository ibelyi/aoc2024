#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};
use std::{cmp::Ordering, collections::HashSet};

pub fn main() {
    let solver = Solution {};
    solver.solve("day14");
}
struct Solution {}

#[derive(Clone)]
struct Robot {
    loc: (i64, i64),
    speed: (i64, i64),
}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("12"))],
            Step::Second => vec![], //vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Robot> = input
            .iter()
            .map(|line| {
                let res = line.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
                let loc = res.0.split_once(',').unwrap();
                let speed = res.1.split_once(',').unwrap();
                Robot {
                    loc: (loc.0.parse().unwrap(), loc.1.parse().unwrap()),
                    speed: (speed.0.parse().unwrap(), speed.1.parse().unwrap()),
                }
            })
            .collect();
        let limit = if data.len() < 15 { (11, 7) } else { (101, 103) };
        match step {
            Step::First => count(&data, limit).to_string(),
            Step::Second => count2(&data, limit).to_string(),
        }
    }
}

fn count2(data: &[Robot], limit: (i64, i64)) -> usize {
    let mut data = data.to_vec();
    let mut count = 0;
    loop {
        let locs: HashSet<(i64, i64)> = data.iter().map(|robot| robot.loc).collect();
        let done = locs
            .iter()
            .filter(|loc| {
                locs.contains(&(loc.0 + 1, loc.1))
                    || locs.contains(&(loc.0 - 1, loc.1))
                    || locs.contains(&(loc.0, loc.1 + 1))
                    || locs.contains(&(loc.0, loc.1 - 1))
            })
            .count();
        if done > locs.len() / 2 {
            for y in 0..limit.1 {
                eprintln!(
                    "{}",
                    (0..limit.0)
                        .map(|x| if locs.contains(&(x, y)) { '#' } else { '.' })
                        .collect::<String>()
                );
            }
            break;
        }
        for robot in &mut data {
            robot.loc = (
                (robot.loc.0 + robot.speed.0 + limit.0) % limit.0,
                (robot.loc.1 + robot.speed.1 + limit.1) % limit.1,
            );
        }
        count += 1;
    }
    count
}

fn count(data: &[Robot], limit: (i64, i64)) -> usize {
    let mut data = data.to_vec();
    for _ in 0..100 {
        for robot in &mut data {
            robot.loc = (
                (robot.loc.0 + robot.speed.0 + limit.0) % limit.0,
                (robot.loc.1 + robot.speed.1 + limit.1) % limit.1,
            );
        }
    }
    data.iter()
        .fold([0; 4], |mut res, robot| {
            match (
                robot.loc.0.cmp(&(limit.0 / 2)),
                (robot.loc.1.cmp(&(limit.1 / 2))),
            ) {
                (Ordering::Less, Ordering::Less) => res[0] += 1,
                (Ordering::Less, Ordering::Greater) => res[1] += 1,
                (Ordering::Greater, Ordering::Less) => res[2] += 1,
                (Ordering::Greater, Ordering::Greater) => res[3] += 1,
                _ => {}
            }
            res
        })
        .iter()
        .product()
}
