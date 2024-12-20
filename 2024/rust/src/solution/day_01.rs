use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::common::{deduplication::Deduplicated, solution::Solution};

pub struct Day1 {
    left: Deduplicated<usize>,
    right: Deduplicated<usize>,
}

impl Solution for Day1 {
    fn name() -> &'static str {
        "--- Day 1: Historian Hysteria ---"
    }

    fn new(input: &str) -> Self {
        let (_, numbers) = parse_input(input).unwrap();

        Self {
            left: Deduplicated::from(numbers.iter().map(|v| v.0 as usize)),
            right: Deduplicated::from(numbers.iter().map(|v| v.1 as usize)),
        }
    }

    fn part_a(&self) -> Option<String> {
        let result: usize = self
            .left
            .duplicated()
            .zip(self.right.duplicated())
            .map(|(l, r)| (l.abs_diff(*r)))
            .sum();

        Some(result.to_string())
    }

    fn part_b(&self) -> Option<String> {
        let result: usize = self
            .left
            .iter()
            .map(|(k, v)| self.right.get(k).unwrap_or(&0) * k * v)
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
        let day = Day1::new(A);
        assert_eq!(day.part_a().unwrap(), "11");
    }
    #[test]
    fn b() {
        let day = Day1::new(A);
        assert_eq!(day.part_b().unwrap(), "31");
    }

    const A: &'static str = "3   4
4   3
2   5
1   3
3   9
3   3";
}
