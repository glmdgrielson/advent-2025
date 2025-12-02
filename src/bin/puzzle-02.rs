//! Puzzle 02: Gift Shop
//! ====================
//!
//! Somebody has been having "fun" with our product lines.

use std::fs::read_to_string;

use advent_2025::{AdventError, Puzzle};

#[derive(Clone, Debug)]
struct Ranges(Vec<(u64, u64)>);

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
                    let one = range.0.parse::<u64>().map_err(|_| {
                        AdventError::Parse(format!("invalid product ID {0}", range.0))
                    })?;
                    let two = range.1.parse::<u64>().map_err(|_| {
                        AdventError::Parse(format!("invalid product ID {0}", range.1))
                    })?;
                    Ok((one, two))
                })
                .collect::<Result<Vec<_>, AdventError>>()?;
        Ok(Ranges(ranges))
    }

    /// Find all the bad ids and sum them.
    fn part_one(&self) -> Result<String, AdventError> {
        let bad_ids: u64 = self
            .0
            .iter()
            .map(|&(one, two)| one..=two)
            .map(|range| range.filter(|&id| check_id(id)).sum::<u64>())
            .sum();
        Ok(bad_ids.to_string())
    }

    /// Find all the repetitive IDs and sum them.
    ///
    /// Oh Eric WHY...
    fn part_two(&self) -> Result<String, AdventError> {
        let bad_ids = self.0.iter().map(|&(one, two)| one..=two)
            .map(|range| {
                range.filter(|&id| check_repeats(id)).sum::<u64>()
            })
        .sum::<u64>();
       Ok(bad_ids.to_string()) 
    }
}

/// Detect a bad ID.
///
/// A bad ID is any number that can be expressed
/// as a string of the form `XX` (that is, a reptition
/// of some pattern of digits.)
fn check_id(id: u64) -> bool {
    let id = id.to_string();
    let len = id.len();
    if len % 2 != 0 {
        return false;
    }
    let index = len / 2;
    let (one, two) = id.split_at(index);
    one == two
}

/// Detect a repetitive ID.
///
/// A repetitive ID is anything that matches
/// the regex `(\d+)\1+`. Careful viewers may
/// note that the `regex` crate explicitly does not
/// provide backreferences. AAAAAAAAAAAAAAAA.
fn check_repeats(id: u64) -> bool {
    let id = id.to_string();
    let digits = id.chars().collect::<Vec<_>>();
    let len = digits.len();
    (1..len / 2).any(|len| {
        let first = &digits[..len];
        digits.chunks(len).all(|chunk| chunk == first)
    })
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/puzzle02.txt")?;

    let data = Ranges::parse_input(&file)?;

    println!("The sum of bad IDs is {0}", data.part_one()?);
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

        assert_eq!(data.0.len(), 11);
        assert_eq!(data.0[0], (11, 22));
    }

    #[test]
    fn part_one() {
        let data = Ranges::parse_input(&TEST_INPUT).expect("Could not parse test input");

        let answer = data.part_one().expect("infalliable result");
        assert_eq!(answer, "1227775554");
    }

    #[test]
    fn part_two() {
        let data = Ranges::parse_input(&TEST_INPUT).expect("Could not parse test input");

        let answer = data.part_two().unwrap();
        assert_eq!(answer, "4174379265");
    }

    #[test]
    fn test_repeats() {
        assert!(check_repeats(99));
        assert!(check_repeats(11111));
    }
}
