//! Puzzle 09: Movie Theater
//! ========================
//!
//! No, look down at the floor! It's better entertainment
//! than the movie they're showing here, anyway!

use advent_2025::{read_file, Puzzle, AdventError};

use itertools::Itertools;

type Point = (u64, u64);

#[derive(Clone, Debug)]
struct Floor(Vec<Point>);

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

    /// Find the largest rectangle bounded in the shape
    /// implied by the connections between adjacent points.
    ///
    /// TODO: This takes a noticeable second, so I could
    /// probably optimize this a bit, but that's for after
    /// puzzles are done.
    fn part_two(&self) -> Result<String, AdventError> {
        let edges = self.0.iter()
            .circular_tuple_windows()
            .map(|(one, two)| Line {one: *one, two: *two})
            .collect::<Vec<_>>();

        let max = self.0.iter().tuple_combinations().filter_map(|(one, two)| {
            if edges.iter().all(|line| line.out_of_bounds(*one, *two)) {
                Some((one.0.abs_diff(two.0) + 1) * (one.1.abs_diff(two.1) + 1))
            } else {
                None
            }
        }).max();
        if let Some(max) = max {
            Ok(max.to_string())
        } else {
            Err(AdventError::Data("could not find bounded rectangle".to_string()))
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Line {
    one: Point,
    two: Point,
}

impl Line {
    fn out_of_bounds(&self, p1: Point, p2: Point) -> bool {
        let ax = p1.0.min(p2.0);
        let bx = p1.0.max(p2.0);
        let ay = p1.1.min(p2.1);
        let by = p1.1.max(p2.1);

        let x_min = self.one.0.min(self.two.0);
        let x_max = self.one.0.max(self.two.0);
        let y_min = self.one.1.min(self.two.1);
        let y_max = self.one.1.max(self.two.1);

        x_max <= ax
        || x_min >= bx
        || y_max <= ay
        || y_min >= by
    }
}

fn main() -> Result<(), AdventError> {
    let file = read_file("src/input/puzzle09.txt")?;
    let data = Floor::parse_input(&file)?;

    println!("The largest rectangle is {0}", data.part_one()?);
    println!("The largest bounded rectangle is {0}", data.part_two()?);
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

    #[test]
    fn part_two() {
        let data = Floor::parse_input(&TEST_INPUT).expect("could not parse input file");

        let answer = data.part_two().unwrap();
        assert_eq!(answer, "24");
    }
}
