//! Puzzle 02: Gift Shop
//! ====================
//!
//! Somebody has been having "fun" with our product lines.

use std::fs::read_to_string;

use advent_2025::{AdventError, Puzzle};

#[derive(Clone, Debug)]
struct Ranges(Vec<(u32, u32)>);

impl Puzzle for Ranges {
    /// Input consists of a series of product ID ranges.
    ///
    /// A product ID is just a number (thankfully). A product
    /// ID range is two product IDs separated by a dash (`-`).
    /// These ranges then are joined together with commas.
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        let ranges = file
            // Remove the trailing newline, oopsies.
            .trim()
            .split(',')
            .map(|range| {
                let range = range.split('-').collect::<Vec<_>>();
                match range.len() {
                    2 => Ok((range[0], range[1])),
                    _ => Err(AdventError::Parse(
                        "range should only have two elements".to_string(),
                    )),
                }
            })
            .collect::<Result<Vec<_>, AdventError>>()?;
        let ranges =
            ranges
                .iter()
                .map(|range| {
                    let one = range.0.parse::<u32>().map_err(|_| {
                        AdventError::Parse(format!("invalid product ID {0}", range.0))
                    })?;
                    let two = range.1.parse::<u32>().map_err(|_| {
                        AdventError::Parse(format!("invalid product ID {0}", range.1))
                    })?;
                    Ok((one, two))
                })
                .collect::<Result<Vec<_>, AdventError>>()?;
        Ok(Ranges(ranges))
    }

    fn part_one(&self) -> Result<String, AdventError> {
        todo!()
    }

    fn part_two(&self) -> Result<String, AdventError> {
        todo!()
    }
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/puzzle02.txt")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> = LazyLock::new(|| {
        read_to_string("src/input/puzzle02-test.txt").expect("Could not find test input")
    });

    #[test]
    fn parse_input() {
        let data = Ranges::parse_input(&*TEST_INPUT).expect("Could not parse test input");

        assert_eq!(data.0[0], (11, 22));
    }

    #[test]
    fn part_one() {
        let data = Ranges::parse_input(&TEST_INPUT).expect("Could not parse test input");

        let answer = data.part_one().expect("failed to find answer");
        assert_eq!(answer, "1227775554");
    }
}
