//! Puzzle 07: Laboratories
//! =======================
//!
//! Lasers are fun and effective!

use advent_2025::{read_file, Puzzle, AdventError, Grid};

#[derive(Clone, Debug)]
struct Manifold(Grid<Cell>);

#[derive(Clone, Debug, PartialEq, Eq)]
enum Cell {
    Start,
    Split,
    Empty,
}

impl Puzzle for Manifold {
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
    let file = read_file("src/input/puzzle-07.rs")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> = LazyLock::new(
        || read_file("src/input/puzzle07-test.txt").expect("could not read input file"));
}
