#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]
use aoc2024::{Solver, Step};
use std::collections::HashSet;

pub fn main() {
    let solver = Solution {};
    solver.solve("day12");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![
                ("test0.txt", String::from("140")),
                ("test1.txt", String::from("772")),
                ("test2.txt", String::from("1930")),
            ],
            Step::Second => vec![
                ("test0.txt", String::from("80")),
                ("test1.txt", String::from("436")),
            ],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
        count(&data, step).to_string()
    }
}

fn calc_region(loc: (usize, usize), data: &[Vec<char>], region: &mut HashSet<(usize, usize)>) {
    if !region.contains(&loc) {
        let plant = data[loc.0][loc.1];
        region.insert((loc.0, loc.1));
        if loc.0 > 0 && plant == data[loc.0 - 1][loc.1] {
            calc_region((loc.0 - 1, loc.1), data, region);
        }
        if loc.0 < data.len() - 1 && plant == data[loc.0 + 1][loc.1] {
            calc_region((loc.0 + 1, loc.1), data, region);
        }
        if loc.1 > 0 && plant == data[loc.0][loc.1 - 1] {
            calc_region((loc.0, loc.1 - 1), data, region);
        }
        if loc.1 < data[0].len() - 1 && plant == data[loc.0][loc.1 + 1] {
            calc_region((loc.0, loc.1 + 1), data, region);
        }
    }
}

fn perimeter(region: &HashSet<(usize, usize)>, limit: (usize, usize), discount: bool) -> usize {
    region
        .iter()
        .map(|loc| {
            let mut result = 0;
            if (loc.0 == 0 || !region.contains(&(loc.0 - 1, loc.1)))
                && (!discount
                    || loc.1 == 0
                    || (loc.0 > 0 && region.contains(&(loc.0 - 1, loc.1 - 1)))
                    || !region.contains(&(loc.0, loc.1 - 1)))
            {
                result += 1;
            }
            if (loc.0 == limit.0 - 1 || !region.contains(&(loc.0 + 1, loc.1)))
                && (!discount
                    || loc.1 == 0
                    || (loc.0 < limit.0 - 1 && region.contains(&(loc.0 + 1, loc.1 - 1)))
                    || !region.contains(&(loc.0, loc.1 - 1)))
            {
                result += 1;
            }
            if (loc.1 == 0 || !region.contains(&(loc.0, loc.1 - 1)))
                && (!discount
                    || loc.0 == 0
                    || (loc.1 > 0 && region.contains(&(loc.0 - 1, loc.1 - 1)))
                    || !region.contains(&(loc.0 - 1, loc.1)))
            {
                result += 1;
            }
            if (loc.1 == limit.1 - 1 || !region.contains(&(loc.0, loc.1 + 1)))
                && (!discount
                    || loc.0 == 0
                    || (loc.1 < limit.1 - 1 && region.contains(&(loc.0 - 1, loc.1 + 1)))
                    || !region.contains(&(loc.0 - 1, loc.1)))
            {
                result += 1;
            }
            result
        })
        .sum()
}

fn count(data: &[Vec<char>], step: &Step) -> usize {
    let mut taken: HashSet<(usize, usize)> = HashSet::new();
    let mut regions = Vec::new();
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if !taken.contains(&(y, x)) {
                let mut region = HashSet::new();
                calc_region((y, x), data, &mut region);
                taken.extend(region.iter());
                regions.push(region);
            }
        }
    }
    regions
        .iter()
        .map(|region| {
            region.len()
                * perimeter(
                    region,
                    (data.len(), data[0].len()),
                    matches!(step, Step::Second),
                )
        })
        .sum()
}
