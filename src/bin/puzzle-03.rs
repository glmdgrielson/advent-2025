//! Puzzle 03: Lobby
//! ================
//!
//! Alright let's get our power back online!

use advent_2025::{read_file, AdventError, Puzzle};

#[derive(Clone, Debug)]
// This is never going to store more than a single digit in each cell,
// but I want to make space for when I add them all together.
struct Banks(Vec<Vec<u32>>);

impl Puzzle for Banks {
    /// Input consists of a list of batteries, gathered into rows.
    ///
    /// Every battery consists of one digit.
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        let lines = file
            .lines()
            .map(|line| {
                line.chars()
                    .map(|digit| {
                        digit.to_digit(10).ok_or_else(|| {
                            AdventError::Parse(format!("invalid battery {0}", digit))
                        })
                    })
                    .collect::<Result<Vec<_>, AdventError>>()
            })
            .collect::<Result<Vec<_>, AdventError>>()?;
        Ok(Banks(lines))
    }

    /// Find the sum of the highest battery totals.
    fn part_one(&self) -> Result<String, AdventError> {
        let voltages: Vec<u32> = self
            .0
            .iter()
            .map(|line| find_voltage(line))
            .collect::<Result<Vec<_>, AdventError>>()?;
        let sum: u32 = voltages.iter().sum();
        Ok(sum.to_string())
    }

    fn part_two(&self) -> Result<String, AdventError> {
        todo!()
    }
}

fn find_voltage(bank: &[u32]) -> Result<u32, AdventError> {
    let max = max_digit_position(bank)?;
    // Check whether there's anything after this position.
    // Beeping off by one errors got me again...
    if bank[max + 1..].is_empty() {
        // We got the last digit of this row, oops.
        let (line, _) = bank.split_at(max);
        let tens = max_digit_position(line)?;
        let tens = line[tens];
        let ones = bank[max];
        Ok(tens * 10 + ones)
    } else {
        let (_, line) = bank.split_at(max + 1);
        let ones = max_digit_position(line)?;
        let tens = bank[max];
        let ones = line[ones];
        Ok(tens * 10 + ones)
    }
}

fn max_digit_position(bank: &[u32]) -> Result<usize, AdventError> {
    let max = bank.iter().max().ok_or_else(|| AdventError::Data("empty row given".to_string()))?;
    let max_pos = bank.iter().position(|digit| digit == max).expect("Should find maximum");
    Ok(max_pos)
}

fn main() -> Result<(), AdventError> {
    let file = read_file("src/input/puzzle03.txt")?;
    let data = Banks::parse_input(&file)?;

    println!("Total output joltage is {0}", data.part_one()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> =
        LazyLock::new(|| read_file("src/input/puzzle03-test.txt").expect("Could not read input"));

    #[test]
    fn parse_input() {
        let data = Banks::parse_input(&TEST_INPUT).expect("could not parse input");

        assert_eq!(data.0[0][0..9], vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn voltage_calculation() {
        let voltage = find_voltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1]).unwrap();
        assert_eq!(voltage, 98);
        let voltage = find_voltage(&[2, 3, 4, 7, 8]).unwrap();
        assert_eq!(voltage, 78);
    }

    #[test]
    fn part_one() {
        let data = Banks::parse_input(&TEST_INPUT).expect("could not parse input");

        let answer = data.part_one().expect("calculation should succeed");
        assert_eq!(&answer, "357");
    }
}
