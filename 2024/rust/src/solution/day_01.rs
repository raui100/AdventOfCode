use std::collections::BTreeMap;

use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::common::solution::Solution;

pub struct Day {
    left: BTreeMap<u32, u32>,
    right: BTreeMap<u32, u32>,
}

impl Solution for Day {
    fn name() -> &'static str {
        "--- Day 1: Historian Hysteria ---"
    }

    fn new(input: &str) -> Self {
        let (_, numbers) = parse_input(input).unwrap();
        let (mut left, mut right) = (BTreeMap::new(), BTreeMap::new());
        for (l, r) in numbers {
            *left.entry(l).or_insert(0u32) += 1;
            *right.entry(r).or_insert(0u32) += 1;
        }
        Self { left, right }
    }

    fn part_a(&self) -> Option<String> {
        let left = repeated_tree(&self.left);
        let right = repeated_tree(&self.right);
        let result: u32 = left.zip(right).map(|(l, r)| (l.max(r) - l.min(r))).sum();

        Some(result.to_string())
    }

    fn part_b(&self) -> Option<String> {
        // Counting the number of occurence in the right side of the list
        let result: u32 = repeated_tree(&self.left)
            .map(|l| l * self.right.get(l).unwrap_or(&0))
            .sum();
        Some(result.to_string())
    }
}

fn repeated_tree(tree: &BTreeMap<u32, u32>) -> impl Iterator<Item = &'_ u32> {
    tree.iter()
        .flat_map(|(k, &v)| std::iter::repeat_n(k, v as usize))
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
