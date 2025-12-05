//! Puzzle 05: Cafeteria

use advent_2025::{read_file, Puzzle, AdventError};

#[derive(Clone, Debug)]
struct Database {
    ranges: Vec<(u32, u32)>,
    ingredients: Vec<u32>,
}

impl Puzzle for Database {
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
    let file = read_file("src/input/puzzle05.txt")?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> =
        LazyLock::new(|| read_file("src/input/puzzle05-test.txt").expect("could not read input"));
}
