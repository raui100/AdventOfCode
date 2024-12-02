use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};

use crate::common::{parse::parse_input, solution::Solution};

pub struct DayX {}

impl Solution for DayX {
    fn name() -> &'static str {
        todo!()
    }

    fn new(input: &str) -> Self {
        let (_, _data) = parse_input(input, parse_row).unwrap();
        todo!()
    }

    fn part_a(&self) -> Option<String> {
        None
    }

    fn part_b(&self) -> Option<String> {
        None
    }
}

fn parse_row(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, complete::u32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a() {
        let day = DayX::new(TEST_INPUT.into());
        assert_eq!(day.part_a().unwrap(), "");
    }
    #[test]
    fn b() {
        let day = DayX::new(TEST_INPUT.into());
        assert_eq!(day.part_b().unwrap(), "");
    }

    const TEST_INPUT: &'static str = "";
}
