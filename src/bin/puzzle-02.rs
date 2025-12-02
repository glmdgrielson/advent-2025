//! Puzzle 02: Gift Shop
//! ====================
//!
//! Somebody has been having "fun" with our product lines.

use std::fs::read_to_string;

use advent_2025::{Puzzle, AdventError};

#[derive(Clone, Debug)]
struct Ranges(Vec<(u32, u32)>);

impl Puzzle for Ranges {
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

fn main() -> Result<(), AdventError> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> = LazyLock::new(|| {
        read_to_string("src/input/puzzle02-test.txt")
            .expect("Could not find test input")
    });

}
