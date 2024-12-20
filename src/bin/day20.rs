#![warn(clippy::pedantic)]
use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day20");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![("test0.txt", String::from("5"))],
            Step::Second => vec![("test0.txt", String::from("29"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
        count(&data, step).to_string()
    }
}

fn dist(a: (usize, usize), b: (usize, usize)) -> usize {
    let dy = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let dx = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
    dy + dx
}

fn twenty_sec_cheats(times: &[Vec<usize>], cheat: usize) -> usize {
    let limit = if times.len() < 16 {
        if cheat == 2 {
            20
        } else {
            72
        }
    } else {
        100
    };
    let mut result = 0;
    for y in 1..(times.len() - 1) {
        for x in 1..(times[0].len() - 1) {
            if times[y][x] != usize::MAX {
                let start_y = if y > cheat { y - cheat } else { 1 };
                let end_y = if y + cheat < times.len() - 2 {
                    y + cheat
                } else {
                    times.len() - 2
                };
                let start_x = if x > cheat { x - cheat } else { 1 };
                let end_x = if x + cheat < times[0].len() - 2 {
                    x + cheat
                } else {
                    times[0].len() - 2
                };
                for ny in start_y..=end_y {
                    for nx in start_x..=end_x {
                        if times[ny][nx] != usize::MAX {
                            let extra = dist((y, x), (ny, nx));
                            let new_time = times[y][x] + extra;
                            if extra <= cheat
                                && new_time < times[ny][nx]
                                && times[ny][nx] - new_time >= limit
                            {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

fn count(data: &[Vec<char>], step: &Step) -> usize {
    let start = data
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, &c)| if c == 'S' { Some((y, x)) } else { None })
        })
        .unwrap();
    let mut times: Vec<Vec<usize>> = vec![vec![usize::MAX; data[0].len()]; data.len()];
    times[start.0][start.1] = 0;
    let mut curr = start;
    while data[curr.0][curr.1] != 'E' {
        for next in [
            (curr.0 - 1, curr.1),
            (curr.0, curr.1 + 1),
            (curr.0 + 1, curr.1),
            (curr.0, curr.1 - 1),
        ] {
            if data[next.0][next.1] != '#' && times[next.0][next.1] == usize::MAX {
                times[next.0][next.1] = times[curr.0][curr.1] + 1;
                curr = next;
                break;
            }
        }
    }
    match step {
        Step::First => twenty_sec_cheats(&times, 2),
        Step::Second => twenty_sec_cheats(&times, 20),
    }
}
