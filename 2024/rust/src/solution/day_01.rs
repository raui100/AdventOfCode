use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::common::{deduplication::Deduplicated, solution::Solution};

pub struct Day {
    left: Deduplicated<u32>,
    right: Deduplicated<u32>,
}

impl Solution for Day {
    fn name() -> &'static str {
        "--- Day 1: Historian Hysteria ---"
    }

    fn new(input: &str) -> Self {
        let (_, numbers) = parse_input(input).unwrap();

        Self {
            left: Deduplicated::from(numbers.iter().map(|v| v.0)),
            right: Deduplicated::from(numbers.iter().map(|v| v.1)),
        }
    }

    fn part_a(&self) -> Option<String> {
        let result: u32 = self
            .left
            .duplicated()
            .zip(self.right.duplicated())
            .map(|(l, r)| (l.max(r) - l.min(r)))
            .sum();

        Some(result.to_string())
    }

    fn part_b(&self) -> Option<String> {
        let result: u32 = self
            .left
            .duplicated()
            .map(|l| *self.right.get(l).unwrap_or(&0) as u32 * l)
            .sum();
        Some(result.to_string())
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(line_ending, parse_row)(input)
}

// Function to parse a single row
fn parse_row(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (num1, _, num2)) = tuple((complete::u32, space1, complete::u32))(input)?;
    Ok((input, (num1, num2)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let (rest, nums) = parse_input(A).unwrap();
        assert_eq!(rest, "");
        assert_eq!(nums[0], (3, 4));
        assert_eq!(nums.last().unwrap(), &(3, 3));
    }
    #[test]
    fn a() {
        let day = Day::new(A);
        assert_eq!(day.part_a().unwrap(), "11");
    }
    #[test]
    fn b() {
        let day = Day::new(A);
        assert_eq!(day.part_b().unwrap(), "31");
    }

    const A: &'static str = "3   4
4   3
2   5
1   3
3   9
3   3";
}
