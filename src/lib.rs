#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#[derive(Debug)]
pub enum Step {
    First,
    Second,
}

use colored::Colorize;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

pub trait Solver {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)>;
    fn solution(&self, step: &Step, input: &[String]) -> String;

    fn solve(&self, day: &str) {
        let input = format!("./input/{day}/input.txt");
        if let Ok(lines) = lines_from_file(&input) {
            for s in &[Step::First, Step::Second] {
                for (file, expected) in self.test_results(s) {
                    let test_input = format!("./input/{day}/{file}");
                    if let Ok(test_lines) = lines_from_file(&test_input) {
                        let actual = self.solution(s, &test_lines);
                        if actual == expected {
                            continue;
                        }
                        eprintln!(
                            "{}: Test from {} got {}, expected {}",
                            format!("{s:?}").yellow(),
                            file.blue().italic(),
                            actual.red(),
                            expected.green()
                        );
                    } else {
                        eprintln!("Failed to read lines from {}", test_input.red().italic());
                    }
                    std::process::exit(1);
                }
                println!(
                    "{}: {}",
                    format!("{s:?}").yellow(),
                    self.solution(s, &lines).green()
                );
            }
        } else {
            eprintln!("Failed to load lines from {}", input.red().italic());
        }
    }
}
