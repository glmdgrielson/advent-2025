//! Puzzle 09: Movie Theater
//! ========================
//!
//! No, look down at the floor! It's better entertainment
//! than the movie they're showing here, anyway!

use advent_2025::{read_file, Puzzle, AdventError};

use itertools::Itertools;

#[derive(Clone, Debug)]
struct Floor(Vec<(u64, u64)>);

impl Puzzle for Floor {
    /// Input consists of a list of pairs of numbers,
    /// representing coordinate points.
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        let pairs = file.lines().map(|line| {
            let pair = line.split(',')
                .map(|num| num.parse::<u64>()
                    .map_err(|e| AdventError::Parse(format!("invalid coordinate {0}", e))))
                .collect::<Result<Vec<_>, AdventError>>()?;
            if pair.len() != 2 {
                Err(AdventError::Parse(format!("invalid coordinate pair {0}", line)))
            } else {
                Ok((pair[0], pair[1]))
            }
        })
        .collect::<Result<Vec<_>, AdventError>>()?;
        Ok(Floor(pairs))
    }

    /// Find the biggest rectangle between two points on the floor.
    fn part_one(&self) -> Result<String, AdventError> {
        let max = self.0.iter().tuple_combinations()
            // Fix off by one errors.
            .map(|(one, two)| (one.0.abs_diff(two.0) + 1) * (one.1.abs_diff(two.1) + 1))
            .max();
        if let Some(max) = max {
            Ok(max.to_string())
        } else {
            Err(AdventError::Data("could not find enough pairs".to_string()))
        }
    }

    fn part_two(&self) -> Result<String, AdventError> {
        todo!()
    }
}

fn main() -> Result<(), AdventError> {
    let file = read_file("src/input/puzzle09.txt")?;
    let data = Floor::parse_input(&file)?;

    println!("The largest rectangle is {0}", data.part_one()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> = LazyLock::new(
        || read_file("src/input/puzzle09-test.txt").expect("could not read input file"));

    #[test]
    fn parse_input() {
        let data = Floor::parse_input(&TEST_INPUT).expect("could not parse input file");

        assert_eq!(data.0[0], (7, 1));
    }

    #[test]
    fn part_one() {
        let data = Floor::parse_input(&TEST_INPUT).expect("could not parse input file");

        let answer = data.part_one().unwrap();
        assert_eq!(answer, "50");
    }
}
