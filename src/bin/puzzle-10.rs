//! Puzzle 10: Factory
//! ==================
//!
//! This is what happens when Mira runs the
//! North Pole instead of Silent Hill.

use std::sync::LazyLock;

use advent_2025::{read_file, Puzzle, AdventError};

use regex::Regex;

/// Ye olde swiss army chainsaw for matching text.
static MACHINE_RE: LazyLock<Regex> = LazyLock::new(||
    Regex::new(r"\[(?P<pattern>[.#]+)\] (?P<buttons>(\(\d+(?:,\d+)*\) )+)\{(?P<joltages>\d+(?:,\d+)*)\}")
    .expect("regex should compile"));

#[derive(Clone, Debug)]
#[allow(dead_code)] // One of the fields is unused for part one.
struct Machine {
    pattern: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

#[derive(Clone, Debug)]
struct Manual(Vec<Machine>);

impl Puzzle for Manual {
    /// Input consists of a series of machine specifications.
    ///
    /// A specification consists of:
    /// 1. An indicator pattern, which consists of a series of
    ///    off or on states, marked by `.` or `#` respectively,
    ///    surrounded by square brackets (`[]`).
    /// 2. A series of buttons, which take the form of a series of
    ///    numbers (corresponding to indicators affected) separated
    ///    by a single comma surrounded by parentheses (`()`).
    /// 3. A series of joltages, which take the form of a series of
    ///    numbers surrounded by braces (`{}`).
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        let machines = file.lines().map(|line| {
            // Ensure that this line matches the spec of a machine.
            let Some(machine) = &MACHINE_RE.captures(line) else {
                return Err(AdventError::Parse(format!("invalid machine spec found: {0}", line)));
            };
            
            // Convert the pattern.
            let pattern = machine.name("pattern").expect("regex should find all subgroups").as_str();
            let pattern = pattern.chars().map(|ch| match ch {
                '.' => Ok(false),
                '#' => Ok(true),
                ch => Err(AdventError::Parse(format!("invalid indicator character {0}", ch)))
            }).collect::<Result<Vec<_>, AdventError>>()?;

            // Parse the joltages.
            let joltages = machine.name("joltages").expect("regex should find all subgroups").as_str();
            let joltages = joltages.split(',').map(|jolt| {
                jolt.parse::<u32>()
                    .map_err(|e| AdventError::Parse(format!("could not parse joltage: {0}", e)))
            }).collect::<Result<Vec<_>, AdventError>>()?;

            // Parse the buttons.
            let buttons = machine.name("buttons").expect("regex should find all subgroups").as_str();
            // We capture multiple buttons separated by whitespace, so we split them here.
            let buttons = buttons.split_ascii_whitespace().map(|button| {
                // Remove the parentheses.
                let Some(button) = button.strip_prefix('(') else {
                    return Err(AdventError::Parse(format!("invalid button format: {0}", button)));
                };
                let Some(button) = button.strip_suffix(')') else {
                    return Err(AdventError::Parse(format!("invalid button format: {0}", button)));
                };

                // Split at the commas and convert to numbers.
                let button = button.split(',').map(|num| {
                    // `usize` is chosen because it needs to map
                    // to indices in the indicator pattern, and
                    // indices are always `usize` in Rust.
                    num.parse::<usize>()
                        .map_err(|e| AdventError::Parse(format!("Invalid button number: {0}", e)))
                }).collect::<Result<Vec<_>, AdventError>>()?;
                Ok(button)
            }).collect::<Result<Vec<_>, AdventError>>()?;

            Ok(Machine { pattern, buttons, joltages })
        }).collect::<Result<Vec<_>, AdventError>>()?;
        Ok(Manual(machines))
    }

    fn part_one(&self) -> Result<String, AdventError> {
        todo!()
    }

    fn part_two(&self) -> Result<String, AdventError> {
        todo!()
    }
}

fn main() -> Result<(), AdventError> {
    let file = read_file("src/input/puzzle10.md")?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> = LazyLock::new(
        || read_file("src/input/puzzle10-test.txt").expect("could not read input file"));

    #[test]
    fn parse_input() {
        let data = Manual::parse_input(&TEST_INPUT).expect("could not parse input file");

        let machine = data.0[0].clone();
        // Test indicator pattern.
        assert_eq!(machine.pattern, vec![false, true, true, false], "incorrect indicator pattern");
        // Test buttons.
        assert_eq!(machine.buttons[1], vec![1, 3], "incorrect buttons");
        // Test joltages.
        assert_eq!(machine.joltages, vec![3, 5, 4, 7], "incorrect joltages");
    }
}
