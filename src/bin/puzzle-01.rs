//! Puzzle 01: Secret Entrance
//! ==========================
//!
//! Project management is easy! Time management less so.

use std::fs::read_to_string;

use advent_2025::{AdventError, Puzzle};

#[derive(Debug)]
struct Safe(Vec<i16>);

impl Puzzle for Safe {
    /// Puzzle input consists of a series of directions.
    ///
    /// A direction takes the form of `Xnn` where X is
    /// either `L` or `R` and `nn` is an integer.
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        let ops: Vec<i16> = file
            .lines()
            .map(|line| {
                let (dir, magnitude) = line.split_at(1);
                let dir = match dir {
                    "L" => false,
                    "R" => true,
                    _ => return Err(AdventError::Parse("invalid direction".to_string())),
                };

                let magnitude = magnitude
                    .parse::<u32>()
                    .map_err(|_| AdventError::Parse("invalid magnitude of rotation".to_string()))?;
                // The dial only has 100 numbers, so we don't care about anything
                // above the hundreds.
                let magnitude = magnitude.rem_euclid(100);

                match dir {
                    true => Ok(magnitude as i16),
                    false => Ok(-(magnitude as i16)),
                }
            })
            .collect::<Result<_, _>>()?;
        Ok(Safe(ops))
    }

    /// Find the password.
    ///
    /// The safe starts at 50. The password is the number of times
    /// the combination lands on 0. (Passing 0 during a rotation does
    /// not count, it must come to a stop on 0.)
    fn part_one(&self) -> Result<String, AdventError> {
        let result = self.0.iter().fold((0, 50), |(counter, dial), op| {
            let dial = dial + op;
            let dial = dial % 100; // Dial only has 100 numbers.

            let counter = match dial {
                0 => counter + 1,
                _ => counter,
            };
            
            (counter, dial)
        });
        Ok(result.0.to_string())
    }

    fn part_two(&self) -> Result<String, AdventError> {
        todo!()
    }
}

fn main() -> Result<(), AdventError> {
    let file = read_to_string("src/input/puzzle01.txt")?;

    let data = Safe::parse_input(&file)?;

    println!("The password is {0}", data.part_one().unwrap());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_FILE: LazyLock<String> = LazyLock::new(|| {
        read_to_string("src/input/puzzle01-test.txt").expect("Could not read input file")
    });

    #[test]
    fn parse_input() {
        let input = &*TEST_FILE;

        let data = Safe::parse_input(input).expect("Could not parse input file");

        assert_eq!(data.0, vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]);
    }

    #[test]
    fn part_one() {
        let input = &*TEST_FILE;

        let data = Safe::parse_input(input).expect("Could not parse input file");

        let answer = data.part_one().expect("Should be infalliable");

        assert_eq!(answer, "3".to_string());
    }
}
