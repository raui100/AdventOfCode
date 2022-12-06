use itertools::Itertools;

use crate::common::io::read_day;
use crate::common::solution::Solution;

pub struct Day {
    data: Vec<char>,
}

impl Day {
    pub fn new() -> Self {
        Self { data: read_day(6).unwrap().chars().collect() }
    }

    pub fn solve(&self, window_size: usize) -> Option<usize> {
        self.data
            .windows(window_size)
            .enumerate()
            .find(|(_, window) | window.iter().all_unique())
            .map(|(ind, _) | ind + window_size)
    }
}


impl Solution for Day {
    fn name(&self) -> &'static str { "Day 0: Insert title" }

    fn part_a(&self) -> Option<String> {
        self.solve(4).map(|num| num.to_string())  // 1034
    }

    fn part_b(&self) -> Option<String> {
        self.solve(14).map(|num| num.to_string())  // 2472
    }
}

