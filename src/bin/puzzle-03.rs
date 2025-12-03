//! Puzzle 03: Lobby
//! ================
//!
//! Alright let's get our power back online!

use advent_2025::{read_file, AdventError, Puzzle};

#[derive(Clone, Debug)]
// This is never going to store more than a single digit in each cell,
// but I want to make space for when I add them all together.
struct Banks(Vec<Vec<u32>>);

impl Puzzle for Banks {
    /// Input consists of a list of batteries, gathered into rows.
    ///
    /// Every battery consists of one digit.
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
    let file = read_file("input/puzzle03.txt")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> =
        LazyLock::new(|| read_file("input/puzzle03-test.txt").expect("Could not read input"));

    #[test]
    fn parse_input() {
        let data = Banks::parse_input(&TEST_INPUT).expect("could not parse input");

        assert_eq!(data.0[0][0..9], vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
