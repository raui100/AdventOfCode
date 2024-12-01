use std::collections::HashMap;

use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::common::solution::Solution;

pub struct Day {
    numbers: Vec<(u32, u32)>,
}

impl Solution for Day {
    fn name() -> &'static str {
        "--- Day 1: Historian Hysteria ---"
    }

    fn new(input: &str) -> Self {
        let (_, numbers) = parse_input(input).unwrap();
        Self { numbers }
    }

    fn part_a(&self) -> Option<String> {
        let (mut left, mut right) = (Vec::new(), Vec::new());
        for number in &self.numbers {
            left.push(number.0);
            right.push(number.1);
        }
        left.sort();
        right.sort();
        let result: u32 = left
            .into_iter()
            .zip(right)
            .map(|(l, r)| u32::max(l, r) - u32::min(l, r))
            .sum();
        Some(result.to_string())
    }

    fn part_b(&self) -> Option<String> {
        // Counting the number of occurence in the right side of the list
        let mut right_count: HashMap<u32, u32> = HashMap::new();
        for (_, num) in &self.numbers {
            let value = right_count.entry(*num).or_insert(0);
            *value += 1;
        }

        let mut result = 0;
        for (num, _) in &self.numbers {
            if let Some(count) = right_count.get(num) {
                result += num * count
            }
        }

        Some(result.to_string())
    }
}

// Function to parse the whole input
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
