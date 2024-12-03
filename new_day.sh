#!/bin/bash -e

if [ -z "$1" ] || [[ $1 != day* ]]; then
  echo "Usage: $0 <day>"
  echo "  Where <day> has to have 'day' prefix"
  exit 1
fi

DAY=$1
ROOT="."

if [ ! -e $ROOT/Cargo.toml ]; then
  echo "Script should run from a Cargo repo"
  exit 1
fi
crate=$(sed -n 's/name = "\(.*\)"/\1/p' $ROOT/Cargo.toml)
if [[ $crate != aoc* ]]; then
  echo "Script should run from a Advent Of Code report"
  exit 1
fi
dir=$(basename $(pwd))
if [[ $crate != $dir ]]; then
  echo "Then name in Cargo.toml should match the directory"
  exit 1
fi

if [ -e $ROOT/input/$DAY ] || [ -e $ROOT/src/bin/$DAY.rs ]; then
  echo "Day $DAY already exists"
  exit 1
fi

mkdir -p $ROOT/input/$DAY
touch $ROOT/input/$DAY/input.txt
mkdir -p $ROOT/src/bin
cat > $ROOT/src/bin/$1.rs <<TEMPLATE
#![warn(clippy::pedantic)]
#![allow(clippy::match_same_arms)]
use $crate::{Solver, Step};

pub fn main() {
    let solver = Solution {};
    solver.solve("$1");
}
struct Solution {}

impl Solver for Solution {
    fn test_results(&self, step: &Step) -> Vec<(&'static str, String)> {
        match step {
            Step::First => vec![], //vec![("test0.txt", String::from("0"))],
            Step::Second => vec![], //vec![("test0.txt", String::from("0"))],
        }
    }

    fn solution(&self, step: &Step, input: &[String]) -> String {
        let data: Vec<&String> = input.iter().collect();
        match step {
            Step::First => count(&data).to_string(),
            Step::Second => count(&data).to_string(),
        }
    }
}

fn count(_: &[&String]) -> u64 {
    0
}
TEMPLATE
