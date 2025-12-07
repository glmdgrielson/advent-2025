//! Puzzle 07: Laboratories
//! =======================
//!
//! Lasers are fun and effective!

use advent_2025::{read_file, AdventError, Grid, GridIndex, Puzzle};

use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Manifold(Grid<Cell>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Start,
    Split,
    Empty,
}

impl Puzzle for Manifold {
    /// Input consists of a tachyon manifold.
    ///
    /// No idea what _that_ means, but it's a grid of cells consisting of
    /// either an `S` representing the starting beam, a period (`.`)
    /// representing empty space, or a caret (`|`) representing
    /// a beam splitter.
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        let grid = file
            .lines()
            .try_fold(Grid::new(0, 0, Vec::new()), |mut grid, line| -> Result<_, AdventError> {
                let line = line
                    .chars()
                    .map(|ch| match ch {
                        'S' => Ok(Cell::Start),
                        '.' => Ok(Cell::Empty),
                        '^' => Ok(Cell::Split),
                        err => Err(AdventError::Parse(format!("invalid cell {0}", err))),
                    })
                    .collect::<Result<Vec<Cell>, AdventError>>()?;
                grid.push_row(line);
                Ok(grid)
            })?;
        Ok(Manifold(grid))
    }

    /// Find how many times the beam was split.
    ///
    /// It helps to actually read the task before
    /// attempting to solve it! Had a real Alex
    /// Horne moment here.
    fn part_one(&self) -> Result<String, AdventError> {
        // Sanity check here.
        if self.0.row_iter(0).filter(|&cell| *cell == Cell::Start).count() != 1 {
            return Err(AdventError::Data("first row does not have exactly one start".to_string()));
        }
        let Some(start) = self.0.row_iter(0).position(|&cell| cell == Cell::Start) else {
            unreachable!("should have just checked for start's existence");
        };

        // Multiple beams coalesce, so
        // we should only count each column
        // once. Sounds like a set to me.
        let mut beams = HashSet::new();
        // Add the column of our initial starting position.
        beams.insert(start);
        let beams: Result<_, AdventError> = self.0.rows()
            .skip(1) // ignore the first row
            .try_fold((0, beams), |(splits, beams), row| {
                let mut new_beams = beams.clone();
                let mut splits = splits;
                for &col in beams.iter() {
                    match self.0.get(GridIndex::new(col, row)) {
                        Some(Cell::Empty) => {
                            // Nothing happens.
                        }
                        Some(Cell::Split) => {
                            // The beam stops at a splitter,
                            // so remove it from the set.
                            new_beams.remove(&col);
                            // Add to the split counter.
                            splits += 1;

                            // These are both sanity checks,
                            // but they're good to have.
                            if col != 0 {
                                new_beams.insert(col - 1);
                            }
                            if col != self.0.width() - 1 {
                                new_beams.insert(col + 1);
                            }
                        },
                        Some(Cell::Start) => return Err(AdventError::Data("multiple starts found".to_string())),
                        None => unreachable!("escaped grid somehow {0}:{1}", row, col)
                    }
                }
                Ok((splits, new_beams))
            });
        // This is here mostly for type checking.
        let (splits, _) = beams?;
        Ok(splits.to_string())
    }

    /// Find the total number of paths the particle can take.
    fn part_two(&self) -> Result<String, AdventError> {
        if self.0.row_iter(0).filter(|&cell| *cell == Cell::Start).count() != 1 {
            return Err(AdventError::Data("first row should contain exactly one start".to_string()));
        }

        let init = self.0.row_iter(0).map(|cell| match cell {
            Cell::Start => Ok(1),
            Cell::Empty => Ok(0),
            Cell::Split => Err(AdventError::Data("unreachable splitter".to_string())),
        }).collect::<Result<Vec<usize>, AdventError>>()?;

        let res: Result<_, AdventError> = self.0.rows()
            .skip(1) // ignore the first row
            .try_fold(init, |totals, row| {
                let mut next = vec![0; totals.len()];
                self.0.row_iter(row).enumerate().try_for_each(|(idx, &col)| {
                    match col {
                        Cell::Empty => {
                            next[idx] = totals[idx]; 
                        }
                        Cell::Split => {
                            next[idx] = 0;
                            if idx != 0 {
                                next[idx - 1] += totals[idx];
                            }
                            if idx != totals.len() - 1 {
                                // This isn't getting called
                                // and I don't know why.
                                next[idx + 1] += totals[idx];
                            }
                        }
                        Cell::Start => return Err(
                            AdventError::Data("multiple starts found".to_string())
                        ),
                    }
                    Ok(())
                })?;
                dbg!(&next);
                Ok(next)
            });
        let sum = res?.iter().sum::<usize>();
        Ok(sum.to_string())
    }
}

fn main() -> Result<(), AdventError> {
    let file = read_file("src/input/puzzle07.txt")?;

    let data = Manifold::parse_input(&file)?;
    println!("The total number of splits is {0}", data.part_one()?);
    println!("The total number of possibilities is {0}", data.part_two()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> = LazyLock::new(|| {
        read_file("src/input/puzzle07-test.txt").expect("could not read input file")
    });

    #[test]
    fn parse_input() {
        let data = Manifold::parse_input(&TEST_INPUT).expect("could not parse input file");

        assert_eq!(data.0.get((7, 0)), Some(&Cell::Start));
    }

    #[test]
    fn part_one() {
        let data = Manifold::parse_input(&TEST_INPUT).expect("could not parse input file");

        let answer = data.part_one().unwrap();
        assert_eq!(answer, "21");
    }

    #[test]
    fn part_two() {
        let data = Manifold::parse_input(&TEST_INPUT).expect("could not parse input file");

        let answer = data.part_two().unwrap();
        assert_eq!(answer, "48");
    }
}
