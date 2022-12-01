use itertools::Itertools;

use crate::common::io::read_day;
use crate::common::solution::Solution;

pub struct Day {
    data: Vec<usize>,
}

impl Day {
    pub fn new() -> Self {
        let data: Vec<usize> = read_day(1).unwrap().split("\n\n")
            .map(|group| group.lines().map(|l| l.parse::<usize>().unwrap()).sum())
            .sorted_unstable_by_key(|&num| std::cmp::Reverse(num))  // Descending
            .collect();

        Day { data }
    }
}


impl Solution for Day {
    fn name(&self) -> &'static str { "Day 1: Calorie Counting" }

    fn part_a(&self) -> Option<String> {
        Some(self.data.first().unwrap().to_string())  // 75501
    }

    fn part_b(&self) -> Option<String> {
        Some(self.data[..3].iter().sum::<usize>().to_string())  // 215594
    }
}

