#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day09");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("1928"))],
            Step::Second => vec![("test0.txt", String::from("2858"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<u64> = input[0]
            .chars()
            .map(|c| u64::from(c.to_digit(10).unwrap()))
            .collect();
        match step {
            Step::First => count(&data).to_string(),
            Step::Second => count2(&data).to_string(),
        }
    }
}

fn aggregate(index: u64, size: u64) -> u64 {
    size * (2 * index + size - 1) / 2
}

fn count2(data: &[u64]) -> u64 {
    let mut index = 0;
    let mut data: Vec<(u64, u64)> = data
        .iter()
        .map(|&size| {
            index += size;
            (index - size, size)
        })
        .collect();
    for i in (0..data.len()).step_by(2).rev() {
        for s in (1..i).step_by(2) {
            if data[s].1 >= data[i].1 {
                data[i].0 = data[s].0;
                data[s].0 += data[i].1;
                data[s].1 -= data[i].1;
                break;
            }
        }
    }
    (0..data.len())
        .step_by(2)
        .map(|i| (i / 2) as u64 * aggregate(data[i].0, data[i].1))
        .sum()
}

fn count(data: &[u64]) -> u64 {
    let mut result = 0;
    let mut index = 0;
    let mut start = 0;
    let mut end = ((data.len() - 1) >> 1) << 1;
    let mut left = data[end];
    while start < end {
        let id = (start / 2) as u64;
        result += id * aggregate(index, data[start]);
        index += data[start];
        start += 1;
        let mut hole = data[start];
        while hole > 0 {
            if left == 0 {
                end -= 2;
                if start > end {
                    break;
                }
                left = data[end];
            }
            let take = std::cmp::min(left, hole);
            let id = (end / 2) as u64;
            result += id * aggregate(index, take);
            hole -= take;
            left -= take;
            index += take;
        }
        start += 1;
    }
    if left > 0 {
        let id = (end / 2) as u64;
        result += id * aggregate(index, left);
    }
    result
}
