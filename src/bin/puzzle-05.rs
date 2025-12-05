//! Puzzle 05: Cafeteria
//! ====================
//!
//! Hey, can you make sure we're eating good
//! this week? We don't have time to check
//! everything by hand. Or by nose.

use advent_2025::{read_file, AdventError, Puzzle};

#[derive(Clone, Debug)]
struct Database {
    /// A list of ranges of known good
    /// ingredients, inclusive of both ends.
    ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

impl Puzzle for Database {
    /// Puzzle input consists of a series of ingredient ranges
    /// and a list of ingredients.
    ///
    /// An ingredient, as always, is represented by a number.
    /// A range is a pair of such numbers separated by a dash.
    /// The two halves of the input are separated by a blank line.
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        let (ranges, ingredients) = file
            .split_once("\n\n")
            .ok_or_else(|| AdventError::Parse("could not find ingredients list".to_string()))?;
        
        let ranges = ranges
            .lines()
            .map(|line| {
                let Some((one, two)) = line.split_once('-') else {
                    return Err(AdventError::Parse(format!("invalid range {0}", line)));
                };
                let Ok(one) = one.parse::<u64>() else {
                    return Err(AdventError::Parse(format!("invalid ingredient {0}", one)));
                };
                let Ok(two) = two.parse::<u64>() else {
                    return Err(AdventError::Parse(format!("invalid ingredient {0}", two)));
                };
                Ok((one, two))
            })
            .collect::<Result<Vec<_>, AdventError>>()?;

        let ingredients = ingredients
            .lines()
            .map(|item| {
                item.parse::<u64>()
                    .map_err(|_| AdventError::Parse(format!("invalid ingredient {0}", item)))
            })
            .collect::<Result<Vec<_>, AdventError>>()?;

        let database = Database {
            ranges,
            ingredients,
        };

        Ok(database)
    }

    /// Find the sum of all of the fresh ingredients.
    ///
    /// An ingredient is considered fresh if it is contained
    /// inside of any of the ranges stored in the database.
    fn part_one(&self) -> Result<String, AdventError> {
        let sum = self
            .ingredients
            .iter()
            .filter(|&item| {
                self.ranges
                    .iter()
                    .any(|&(one, two)| (one..=two).contains(item))
            })
            .count();
        Ok(sum.to_string())
    }

    /// Find the maximum number of possible fresh ingredients.
    fn part_two(&self) -> Result<String, AdventError> {
        let mut bounds = self.ranges.clone();
        bounds.sort_by_key(|range| range.0); // Get all of the ranges sorted.

        // Credit to AkalUstat for this technique, by the way.
        let bounds = bounds.into_iter().fold(Vec::new(), |mut acc, range| {
            let Some(last) = acc.last() else {
                // The vector is empty, so we
                // can just push what we have
                // and be done with it.
                acc.push(range);
                return acc;
            };
            // Copy here to avoid lifetime issues.
            let last = *last;

            // If there is overlap, combine the ranges.
            if ranges_overlap(last, range) {
                acc.pop();
                let range = (last.0.min(range.0), last.1.max(range.1));
                acc.push(range);
            } else {
                // Otherwise we can add the new range without worry.
                acc.push(range);
            }
            acc
        });

        let sum = bounds
            .iter()
            .map(|&(one, two)| one..=two)
            .map(|range| range.count())
            .sum::<usize>();
        Ok(sum.to_string())
    }
}

/// Check whether two ranges overlap.
///
/// This checks that the second range starts
/// after the first one ends, or that the second
/// range ends before the first one starts.
fn ranges_overlap(one: (u64, u64), two: (u64, u64)) -> bool {
    !(one.1 < two.0 || two.1 < one.0)
}

fn main() -> Result<(), AdventError> {
    let file = read_file("src/input/puzzle05.txt")?;

    let data = Database::parse_input(&file)?;

    println!("The number of fresh ingredients is {0}", data.part_one()?);
    println!(
        "The most fresh ingredients possible is {0}",
        data.part_two()?
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> =
        LazyLock::new(|| read_file("src/input/puzzle05-test.txt").expect("could not read input"));

    #[test]
    fn parse_input() {
        let data = Database::parse_input(&TEST_INPUT).expect("could not parse input");

        assert_eq!(data.ranges[0], (3, 5));
        assert_eq!(data.ingredients, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn part_one() {
        let data = Database::parse_input(&TEST_INPUT).expect("could not parse input");

        let answer = data.part_one().expect("operation should be infalliable");
        assert_eq!(answer, "3");
    }

    #[test]
    fn part_two() {
        let data = Database::parse_input(&TEST_INPUT).expect("could not parse input");

        let answer = data.part_two().expect("operation should be infalliable");
        assert_eq!(answer, "14");
    }
}
