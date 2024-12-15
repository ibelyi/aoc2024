#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]
use aoc2024::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("day15");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![
                ("test0.txt", String::from("2028")),
                ("test1.txt", String::from("10092")),
            ],
            Step::Second => vec![("test1.txt", String::from("9021"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let mut iter = input.iter();
        let map: Vec<Vec<char>> = iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();
        let moves: Vec<char> = iter.flat_map(|line| line.chars()).collect();
        match step {
            Step::First => count(&map, &moves).to_string(),
            Step::Second => count2(&map, &moves).to_string(),
        }
    }
}

fn next_loc(loc: &(usize, usize), dir: char) -> (usize, usize) {
    match dir {
        '^' => (loc.0 - 1, loc.1),
        '>' => (loc.0, loc.1 + 1),
        'v' => (loc.0 + 1, loc.1),
        '<' => (loc.0, loc.1 - 1),
        c => panic!("Unknown direction {c}"),
    }
}

fn can_move(map: &[Vec<char>], loc: (usize, usize), dir: char) -> bool {
    let next = next_loc(&loc, dir);
    match map[next.0].get(next.1).unwrap() {
        '.' => true,
        '[' => {
            can_move(map, next, dir)
                && (dir == '<' || dir == '>' || can_move(map, (next.0, next.1 + 1), dir))
        }
        ']' => {
            can_move(map, next, dir)
                && (dir == '<' || dir == '>' || can_move(map, (next.0, next.1 - 1), dir))
        }
        'O' => can_move(map, next, dir),
        '#' => false,
        c => panic!("Unknown map element: {c}"),
    }
}

fn do_move(map: &mut [Vec<char>], loc: (usize, usize), dir: char) -> (usize, usize) {
    let next = next_loc(&loc, dir);
    match map[next.0].get(next.1).unwrap() {
        '.' => {}
        '[' => {
            do_move(map, next, dir);
            if dir == '^' || dir == 'v' {
                do_move(map, (next.0, next.1 + 1), dir);
            }
        }
        ']' => {
            do_move(map, next, dir);
            if dir == '^' || dir == 'v' {
                do_move(map, (next.0, next.1 - 1), dir);
            }
        }
        'O' => {
            do_move(map, next, dir);
        }
        c => panic!("Can't move due to: {c}"),
    }
    map[next.0][next.1] = map[loc.0][loc.1];
    map[loc.0][loc.1] = '.';
    next
}

fn count2(map: &[Vec<char>], moves: &[char]) -> usize {
    let map: Vec<Vec<char>> = map
        .iter()
        .map(|line| {
            line.iter()
                .flat_map(|c| match c {
                    '#' => vec!['#', '#'],
                    '.' => vec!['.', '.'],
                    'O' => vec!['[', ']'],
                    '@' => vec!['@', '.'],
                    c => panic!("Unknown map item: {c}"),
                })
                .collect()
        })
        .collect();
    count(&map, moves)
}

fn count(map: &[Vec<char>], moves: &[char]) -> usize {
    let mut robot = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, &c)| if c == '@' { Some((y, x)) } else { None })
        })
        .expect("Robot not found");
    let mut map = map.to_vec();
    for &dir in moves {
        if can_move(&map, robot, dir) {
            robot = do_move(&mut map, robot, dir);
        }
    }
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, &c)| if c == 'O' || c == '[' { y * 100 + x } else { 0 })
                .sum::<usize>()
        })
        .sum()
}
