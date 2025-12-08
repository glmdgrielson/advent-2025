//! Puzzle 08: Playground
//! =====================
//!
//! Playing with wires is perfectly safe!

use advent_2025::{read_file, AdventError, Puzzle};

#[derive(Clone, Debug)]
struct JunctionMap(Vec<Junction>);

#[derive(Clone, Debug, PartialEq)]
struct Junction {
    position: (u64, u64, u64),
    connections: Vec<Junction>,
}

impl Puzzle for JunctionMap {
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        let res = file
            .lines()
            .map(|line| {
                let pos = line
                    .split(',')
                    .map(|num| {
                        // Make sure everything is a number.
                        num.parse::<u64>().map_err(|e| {
                            AdventError::Parse(format!("Could not read number: {0}", e))
                        })
                    })
                    .collect::<Result<Vec<_>, AdventError>>()?;

                // Make sure we have exactly three numbers.
                if pos.len() != 3 {
                    Err(AdventError::Parse(format!(
                        "invalid number of positions in point {0}",
                        line
                    )))
                } else {
                    Ok(Junction {
                        position: (pos[0], pos[1], pos[2]),
                        connections: vec![],
                    })
                }
            })
            .collect::<Result<Vec<_>, AdventError>>()?;
        Ok(JunctionMap(res))
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
        read_file("src/input/puzzle08-test.txt").expect("could not read input file")
    });

    #[test]
    fn parse_input() {
        let data = JunctionMap::parse_input(&TEST_INPUT).expect("could not parse input file");

        assert_eq!(
            data.0[0],
            Junction {
                position: (162, 817, 812),
                connections: Default::default(),
            }
        );
    }
}
