//! Puzzle 06: Trash Compactor
//! ==========================
//!
//! Hey, while you're down here,
//! can you help with some math homework?

use advent_2025::{read_file, Puzzle, AdventError, Grid};

#[derive(Clone, Debug)]
struct Worksheet(Vec<Equation>);

#[derive(Clone, Debug)]
struct Equation {
    operands: Vec<u64>,
    operation: Operation,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Operation {
    Add,
    Mul,
}

impl Puzzle for Worksheet {
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        todo!()
    }

    fn part_one(&self) -> Result<String, AdventError> {
        todo!()
    }

    fn part_two(&self) -> Result<String, AdventError> {
        todo!()
    }
}

fn main() {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> =
        LazyLock::new(|| read_file("src/input/puzzle0=06-test.txt").expect("could not find input file"));

}
