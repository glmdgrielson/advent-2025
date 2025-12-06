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

        let sheet = sheet
            .iter()
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
        todo!()
        // let sum = self.0.iter().map(|equation| match equation.operation {
        //     Operation::Add => equation.operands.iter().sum::<u64>(),
        //     Operation::Mul => equation.operands.iter().product::<u64>(),
        // }).sum::<u64>();
        // Ok(sum.to_string())
    }

    fn part_two(&self) -> Result<String, AdventError> {
        todo!()
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
        assert_eq!(grid.width(), self.operations.len());

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
}

fn parse_input_one(file: &str) -> Result<Vec<Equation>, AdventError> {
    // Assemble the worksheet. `Grid` is a useful type to handle
    // the two dimensional nature of the sheet, and it was already
    // a dependency anyway because of earlier puzzles.
    let grid: Grid<&str> = file
        .lines()
        .fold(Grid::new(0, 0, vec![]), |mut grid, line| {
            let line = line.split_ascii_whitespace().collect();
            grid.push_row(line);
            grid
        });

    // Convert each column into a proper equation.
    let sheet = grid
        .columns()
        .map(|idx| {
            // `Grid` only indirectly lets me access a column,
            // so I do this to actually get at the data I want.
            // `.cloned()` lets me get rid of an indirection.
            let equation = grid.column_iter(idx).cloned().collect::<Vec<_>>();
            // The last element of each equation is special, so
            // make sure that it's separate from all of the operands.
            let Some((operation, operands)) = equation.split_last() else {
                return Err(AdventError::Parse("empty column found".to_string()));
            };

            // The operation should realistically be only one of two
            // things, so we check for that here and error out if we
            // get something weird.
            let operation = match *operation {
                "+" => Operation::Add,
                "*" => Operation::Mul,
                op => return Err(AdventError::Parse(format!("invalid operation {0}", op))),
            };
            // Everything else in the equation should be a number
            // so we check for that and perform the conversion.
            let operands = operands
                .iter()
                .map(|num| {
                    num.parse::<u64>()
                        .map_err(|_| AdventError::Parse(format!("invalid operand {0}", num)))
                })
                .collect::<Result<Vec<_>, AdventError>>()?;

            // Finally return the equation to go into the final result.
            Ok(Equation {
                operands,
                operation,
            })
        })
        .collect::<Result<Vec<_>, AdventError>>()?;
    Ok(sheet)
}

fn main() -> Result<(), AdventError> {
    let file = read_file("src/input/puzzle06.txt")?;

    let data = Worksheet::parse_input(&file)?;

    println!("The grand total solution is {0}", data.part_one()?);
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
}
