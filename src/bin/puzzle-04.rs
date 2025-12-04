//! Puzzle 04: Printing Department
//! ==============================
//!
//! Cleanup on aisle everywhere!

use advent_2025::{read_file, Puzzle, AdventError, Grid, GridIndex};

#[derive(Clone, Debug)]
struct Floor(Grid<bool>);

impl Puzzle for Floor {
    /// Grid consists of a floor layout consisting of cells that
    /// may or may not have paper all over it.
    ///
    /// A cell has `@` if it has paper, and '.' if it does not.
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        let mut grid: Grid<_> = Grid::new(0, 0, vec![]);
        for line in file.lines() {
            let line = line.chars().map(|ch| match ch {
                '@' => Ok(true),
                '.' => Ok(false),
                ch => Err(AdventError::Parse(format!("invalid floor character {0}", ch))),
            }).collect::<Result<Vec<_>, AdventError>>()?;
            grid.push_row(line);
        }
        Ok(Floor(grid))
    }

    fn part_one(&self) -> Result<String, AdventError> {
        todo!()
    }

    fn part_two(&self) -> Result<String, AdventError> {
        todo!()
    }
}

fn main() -> Result<(), AdventError> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> = LazyLock::new(|| {
        read_file("src/input/puzzle04-test.txt").expect("could not read input")
    });

    #[test]
    // Annoyingly, `simple_grid` doesn't seem to allow me to check the contents
    // of a particular row or column in a simple way, so this is the best I'm
    // gonna get without changing how I do grids.
    fn parse_input() {
        let data = Floor::parse_input(&TEST_INPUT).expect("could not parse input");

        assert_eq!(data.0.dimensions(), (10, 10));
    }
}
