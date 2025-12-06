//! Puzzle 06: Trash Compactor
//! ==========================
//!
//! Hey, while you're down here,
//! can you help with some math homework?

use advent_2025::{read_file, AdventError, Grid, Puzzle};

#[derive(Clone, Debug)]
struct Worksheet(Vec<Equation>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Equation {
    operands: Vec<String>,
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
    /// An equation is a series of numbers, and an operation. In the
    /// input file, however, they are represented by the _columns_ of
    /// the array, not the rows. [str::lines] only gets you so far.
    fn parse_input(file: &str) -> Result<Self, AdventError> {
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
                // We need to delay the conversion because each
                // part reads the numbers differently, so we
                // simply take ownership of the operands here.
                let operands = operands
                    .iter()
                    .map(|&num| num.to_string())
                    .collect::<Vec<_>>();

                // Finally return the equation to go into the final result.
                Ok(Equation {
                    operands,
                    operation,
                })
            })
            .collect::<Result<Vec<_>, AdventError>>()?;
        Ok(Worksheet(sheet))
    }

    /// Find the sum of all of the correct answers.
    fn part_one(&self) -> Result<String, AdventError> {
        let sum = self
            .0
            .iter()
            .map(|equation| {
                // Convert the operands into numbers we can use.
                let operands = equation
                    .operands
                    .iter()
                    .map(|num| {
                        num.parse::<u64>()
                            .map_err(|_| AdventError::Parse(format!("invalid operand {0}", num)))
                    })
                    .collect::<Result<Vec<_>, AdventError>>()?;
                match equation.operation {
                    Operation::Add => Ok(operands.iter().sum::<u64>()),
                    Operation::Mul => Ok(operands.iter().product::<u64>()),
                }
            })
            .collect::<Result<Vec<u64>, AdventError>>()?;
        let sum = sum.iter().sum::<u64>();
        Ok(sum.to_string())
    }

    /// Find the sum of all of the correct answers, reading
    /// the sheet correctly this time.
    ///
    /// It turns out that cephalopods read numbers in a completely
    /// different fashion than we do, necessitating a completely
    /// different parsing of the worksheet. AAAAAAAAAAAAAAA.
    fn part_two(&self) -> Result<String, AdventError> {
        todo!()
    }
}

fn parse_little_endian(num: &str) -> Result<u64, AdventError> {
    let num = num.chars().rev().collect::<String>();
    num.parse::<u64>()
        .map_err(|_| AdventError::Data(format!("invalid operand {0}", num)))
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
    fn parse_input() {
        let data = Worksheet::parse_input(&TEST_INPUT).expect("could not parse input file");

        assert_eq!(
            data.0[0],
            Equation {
                operands: vec!["123".to_string(), "45".to_string(), "6".to_string()],
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
    fn part_two() {
        let data = Worksheet::parse_input(&TEST_INPUT).expect("could not parse input file");

        let answer = data.part_two().unwrap();
        assert_eq!(answer, "3263827");
    }
}
