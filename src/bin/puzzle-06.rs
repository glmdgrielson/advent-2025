//! Puzzle 06: Trash Compactor
//! ==========================
//!
//! Hey, while you're down here,
//! can you help with some math homework?

use advent_2025::{read_file, AdventError, Grid, Puzzle};

#[derive(Clone, Debug)]
struct Worksheet {
    sheet: Grid<char>,
    operations: Vec<Operation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Equation {
    operands: Vec<u64>,
    operation: Operation,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Operation {
    Add,
    Mul,
}

impl Puzzle for Worksheet {
    /// Puzzle input consists of a series of oddly formatted equations.
    ///
    /// An equation is a series of numbers, and an operation. However,
    /// we can't parse the numbers here because they're read differently
    /// in each part, so we need to store the text more or less verbatim.
    /// The row of operations is the same, though, so we can convert that
    /// here.
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        let lines = file.lines().collect::<Vec<_>>();
        let Some((ops, sheet)) = lines.split_last() else {
            return Err(AdventError::Parse("file should not be empty".to_string()));
        };

        // Calculate the operation.
        let ops = ops
            .split_ascii_whitespace()
            .map(|op| match op {
                "+" => Ok(Operation::Add),
                "*" => Ok(Operation::Mul),
                err => Err(AdventError::Parse(format!("invalid operation {0}", err))),
            })
            .collect::<Result<Vec<_>, AdventError>>()?;

        // Before we assemble the grid, we need to make sure every line
        // has the same length, or [Grid::push_row] will panic.
        // So we find the longest line...
        let Some(line_len) = sheet.into_iter().max_by_key(|line| line.len()) else {
            return Err(AdventError::Data("file should have operands".to_string()));
        };
        // And get its length.
        let max_len = line_len.len();
        let sheet = sheet
            .iter()
            // Make sure all of the lines are the same length.
            .map(|line| {
                let mut new_line = line.to_string();
                for _ in 0..(max_len - line.len()) {
                    new_line.push(' ');
                }
                new_line
            })
            // Because `Grid` nees them all the same length.
            .fold(Grid::new(0, 0, vec![]), |mut grid, line| {
                grid.push_row(line.chars().collect());
                grid
            });
        Ok(Worksheet {
            sheet,
            operations: ops,
        })
    }

    /// Find the sum of all of the correct answers.
    fn part_one(&self) -> Result<String, AdventError> {
        let equations = self.parse_one()?;
        let sum = equations.iter().map(|equation| match equation.operation {
            Operation::Add => equation.operands.iter().sum::<u64>(),
            Operation::Mul => equation.operands.iter().product::<u64>(),
        }).sum::<u64>();
        Ok(sum.to_string())
    }

    fn part_two(&self) -> Result<String, AdventError> {
        let equations = self.parse_two()?;
        let sum = equations.iter().map(|equation| match equation.operation {
            Operation::Add => equation.operands.iter().sum::<u64>(),
            Operation::Mul => equation.operands.iter().product::<u64>(),
        }).sum::<u64>();
        Ok(sum.to_string())
    }
}

impl Worksheet {
    fn parse_one(&self) -> Result<Vec<Equation>, AdventError> {
        let grid = self
            .sheet
            .rows()
            .try_fold(Grid::new(0, 0, vec![]), |mut grid, idx| -> Result<_, AdventError> {
                let line = self.sheet.row_iter(idx).collect::<String>();
                let line = line
                    .split_ascii_whitespace()
                    .map(|cell| cell.to_owned())
                    .collect::<Vec<_>>();
                grid.push_row(line);
                Ok(grid)
            })?;
        //assert_eq!(grid.width(), self.operations.len());
        if grid.width() != self.operations.len() {
            return Err(AdventError::Parse("improperly formed worksheet".to_string()));
        }

        let sheet = grid
            .columns()
            .map(|idx| {
                // Get the column this index corresponds to.
                let col = grid.column_iter(idx).cloned()
                    .map(|num| {
                        num.parse::<u64>()
                            .map_err(|_| AdventError::Data(format!("invalid operand {0}", num)))
                    })
                    .collect::<Result<Vec<_>, AdventError>>()?;
                let operation = self.operations[idx];
                Ok(Equation {
                    operands: col,
                    operation,
                })
            }).collect::<Result<Vec<_>, AdventError>>()?;
        Ok(sheet)
    }

    fn parse_two(&self) -> Result<Vec<Equation>, AdventError> {
        let columns = self.sheet.columns().map(|col| {
            self.sheet.column_iter(col).collect::<String>()
        }).map(|num| num.trim_ascii().to_owned()).collect::<Vec<_>>();

        let operands = columns.split(|col| col.is_empty()).collect::<Vec<_>>();
        // We need to make sure we have one set of operands
        // for every operation we already had.
        if operands.len() != self.operations.len() {
            return Err(AdventError::Data("improperly formed worksheet".to_string()))
        };
        let sheet = operands
            .iter()
            .enumerate()
            .map(|(idx, nums)| {
                let nums = nums
                    .iter()
                    .map(|num| {
                        num.parse::<u64>()
                            .map_err(|_| AdventError::Data(format!("invalid operand {0}", num)))
                    })
                    .collect::<Result<Vec<_>, AdventError>>()?;
                let operation = self.operations[idx];
                Ok(Equation {
                    operands: nums,
                    operation,
                })
            })
            .collect::<Result<Vec<_>, AdventError>>()?;
        Ok(sheet)
    }
}

fn main() -> Result<(), AdventError> {
    let file = read_file("src/input/puzzle06.txt")?;

    let data = Worksheet::parse_input(&file)?;

    println!("The grand total solution is {0}", data.part_one()?);
    println!("The correctly read solution is {0}", data.part_two()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> = LazyLock::new(|| {
        read_file("src/input/puzzle06-test.txt").expect("could not find input file")
    });

    #[test]
    fn parse_one() {
        let data = Worksheet::parse_input(&TEST_INPUT).expect("could not parse input file");
        let sheet = data.parse_one().unwrap();

        assert_eq!(
            sheet[0],
            Equation {
                operands: vec![123, 45, 6],
                operation: Operation::Mul
            }
        );
    }

    #[test]
    fn part_one() {
        let data = Worksheet::parse_input(&TEST_INPUT).expect("could not parse input file");

        let answer = data.part_one().unwrap();
        assert_eq!(answer, "4277556");
    }

    #[test]
    fn parse_two() {
        let data = Worksheet::parse_input(&TEST_INPUT).expect("could not parse input file");
        let sheet = data.parse_two().unwrap();

        assert_eq!(
            sheet[0],
            // This is technically the _last_ of the operations,
            // since we're reading right to left, but at this
            // point, that's a technicality I can't be bothered with.
            Equation {
                operands: vec![1, 24, 356],
                operation: Operation::Mul,
            }
        );
    }

    #[test]
    fn part_two() {
        let data = Worksheet::parse_input(&TEST_INPUT).expect("could not parse input file");

        let answer = data.part_two().unwrap();
        assert_eq!(answer, "3263827");
    }
}
