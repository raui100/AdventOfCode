use std::num::ParseIntError;
use crate::common::solution::Solution;

pub struct Day {
    data: Vec<i32>,
}

impl Day {
    pub fn new() -> Self {
        let mut data: Vec<i32> = Vec::new();
        let mut current = 0;
        let mut num: Result<i32, ParseIntError> = Ok(0);  // Init with 0
        for line in crate::common::io::read_day(1).unwrap().lines() {
            num = line.parse::<i32>();
            {
                match num {
                    Ok(num) => current += num,
                    Err(_) => {
                        data.push(current);
                        current = 0;
                    }
                }
            }
        }
        if num.is_ok() {  // The last number has to be pushed to the Vec
            data.push(current)
        }
        data.sort_by(|a, b| b.cmp(a));  // Reverse sorting

        Day { data }
    }
}


impl Solution for Day {
    fn name(&self) -> &'static str { "Day 1: Calorie Counting" }

    fn part_a(&self) -> Option<String> {
        Some(self.data.first().unwrap().to_string())  // 75501
    }

    fn part_b(&self) -> Option<String> {
        Some(self.data[..3].iter().sum::<i32>().to_string())  // 215594
    }

}

