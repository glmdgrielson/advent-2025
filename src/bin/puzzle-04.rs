//! Puzzle 04: Printing Department
//! ==============================
//!
//! Cleanup on aisle everywhere!

use advent_2025::{read_file, AdventError, Grid, Puzzle};

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
            let line = line
                .chars()
                .map(|ch| match ch {
                    '@' => Ok(true),
                    '.' => Ok(false),
                    ch => Err(AdventError::Parse(format!(
                        "invalid floor character {0}",
                        ch
                    ))),
                })
                .collect::<Result<Vec<_>, AdventError>>()?;
            grid.push_row(line);
        }
        Ok(Floor(grid))
    }

    /// Find all of the cells that the forklifts can clear.
    ///
    /// The forklift can access any cell that has less than
    /// four obstructed neighbors.
    fn part_one(&self) -> Result<String, AdventError> {
        let total = self
            .0
            .indices()
            .filter(|&idx| self.0[idx])
            .filter(|&idx| self.0.neighbor_cells_of(idx).filter(|&cell| *cell).count() < 4)
            .count();
        Ok(total.to_string())
    }

    /// Find all of the cells it will ever be able to clear.
    ///
    /// This time we are iteratively clearing away the floor
    /// in the hopes that we can clear up more of the way.
    fn part_two(&self) -> Result<String, AdventError> {
        let mut grid = self.0.clone();
        let mut counter = 0;
        loop {
            let pass = grid
                .indices()
                .filter(|&idx| grid[idx])
                .filter(|&idx| grid.neighbor_cells_of(idx).filter(|&cell| *cell).count() < 4)
                .collect::<Vec<_>>();
            // If there are no cells we can clear, our job here is done.
            if pass.is_empty() {
                break;
            }
            // Otherwise add the number of cells we can reach to the counter...
            counter += pass.len();
            // And mark them as clear for the next pass.
            pass.iter().for_each(|&idx| grid[idx] = false);
        }
        Ok(counter.to_string())
    }
}

fn main() -> Result<(), AdventError> {
    let file = read_file("src/input/puzzle04.txt")?;
    let data = Floor::parse_input(&file)?;

    println!(
        "The number of accessible paper cells is {0}",
        data.part_one()?
    );
    println!("The number of clearable cells is {0}", data.part_two()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> =
        LazyLock::new(|| read_file("src/input/puzzle04-test.txt").expect("could not read input"));

    #[test]
    // Annoyingly, `simple_grid` doesn't seem to allow me to check the contents
    // of a particular row or column in a simple way, so this is the best I'm
    // gonna get without changing how I do grids.
    fn parse_input() {
        let data = Floor::parse_input(&TEST_INPUT).expect("could not parse input");

        assert_eq!(data.0.dimensions(), (10, 10));
    }

    #[test]
    fn part_one() {
        let data = Floor::parse_input(&TEST_INPUT).expect("could not parse input");

        let answer = data.part_one().expect("operation is infalliable");
        assert_eq!(answer, "13");
    }

    #[test]
    fn part_two() {
        let data = Floor::parse_input(&TEST_INPUT).expect("could not parse input");

        let answer = data.part_two().expect("operation is infalliable");
        assert_eq!(answer, "43");
    }
}
