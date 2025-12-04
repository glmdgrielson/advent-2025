//! Puzzle 04: Printing Department
//! ==============================
//!
//! Cleanup on aisle everywhere!

use advent_2025::{read_file, Puzzle, AdventError, Grid, GridIndex};

#[derive(Clone, Debug)]
struct Floor(Grid<bool>);

impl Puzzle for Floor {
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
        read_file("src/input/puzzle04-test.txt").expect("could not read input")
    });

}
