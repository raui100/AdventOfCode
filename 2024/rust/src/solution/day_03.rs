use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::u64,
    combinator::{map, value},
    multi::fold_many0,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::common::{parse::parse_input, solution::Solution};

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Do,
    Dont,
    Mul(u64),
}

pub struct Day3 {
    a: u64,
    b: u64,
}

impl Solution for Day3 {
    fn name() -> &'static str {
        "--- Day 3: Mull It Over ---"
    }

    fn new(input: &str) -> Self {
        let (_, a_nums) = parse_input(input, parse_row_a).unwrap();
        let (_, b_nums) = parse_input(input, parse_row_b).unwrap();
        let a: u64 = a_nums.into_iter().sum();
        let b: u64 = b_nums.into_iter().sum();

        Self { a, b }
    }

    fn part_a(&self) -> Option<String> {
        self.a.to_string().into()
    }

    fn part_b(&self) -> Option<String> {
        self.b.to_string().into()
    }
}

fn parse_row_a(input: &str) -> IResult<&str, u64> {
    fold_many0(
        alt((parse_mul, value(0, take(1usize)))),
        || 0,
        |acc, e| acc + e,
    )(input)
}

fn parse_row_b(input: &str) -> IResult<&str, u64> {
    let (rest, (_, n)) = fold_many0(
        alt((
            map(parse_mul, Instruction::Mul),
            value(Instruction::Do, tag("do()")),
            value(Instruction::Dont, tag("don't()")),
            value(Instruction::Mul(0), take(1usize)),
        )),
        || (true, 0),
        |(add, acc), instruction| match instruction {
            Instruction::Do => (true, acc),
            Instruction::Dont => (false, acc),
            Instruction::Mul(n) => match add {
                true => (true, acc + n),
                false => (false, acc),
            },
        },
    )(input)?;

    Ok((rest, n))
}

fn parse_mul(input: &str) -> IResult<&str, u64> {
    map(
        delimited(tag("mul("), separated_pair(u64, tag(","), u64), tag(")")),
        |(a, b)| a * b,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let day = Day3::new(TEST_INPUT.into());
        assert_eq!(day.part_a().unwrap(), "161");
    }
    #[test]
    fn b() {
        let day = Day3::new(TEST_INPUT_2.into());
        assert_eq!(day.part_b().unwrap(), "48");
    }

    const TEST_INPUT: &'static str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_2: &'static str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
}
