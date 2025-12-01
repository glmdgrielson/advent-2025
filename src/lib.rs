//! Helper systems for Advent of Code puzzles.

use std::io::Error as IOError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdventError {
    /// An error reading the file used as input.
    #[error("Error reading file: {0}")]
    File(#[from] IOError),
    /// An error in the format of the input file.
    #[error("Parsing error: {0}")]
    Parse(String),
    /// A contract violation in the input file.
    #[error("Contract violation: {0}")]
    Data(String),
}

pub trait Puzzle: Sized {
    fn parse_input(file: &str) -> Result<Self, AdventError>;

    fn part_one(&self) -> Result<String, AdventError>;

    fn part_two(&self) -> Result<String, AdventError> {
        todo!()
    }
}
