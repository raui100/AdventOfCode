use crate::common::io::read_day;
use crate::common::solution::Solution;

pub struct Day {}

impl Day {
    pub fn new() -> Self {
        let _ = read_day(0).unwrap();
        Self {}
    }
}


impl Solution for Day {
    fn name(&self) -> &'static str { "Day 0: Insert title" }

    fn part_a(&self) -> Option<String> {
        None
    }

    fn part_b(&self) -> Option<String> {
        None
    }
}

