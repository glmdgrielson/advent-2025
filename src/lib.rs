//! Helper systems for Advent of Code puzzles.

use std::io::Error as IOError;
use std::fs::read_to_string;

use thiserror::Error;
pub use simple_grid::{Grid, GridIndex};

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

/// Shortcut to load a data file.
///
/// This is really just [read_to_string][std::fs::read_to_string]
/// mapped to return the right kind of error.
pub fn read_file(name: &str) -> Result<String, AdventError> {
    let data = read_to_string(name)?;
    Ok(data)
}
