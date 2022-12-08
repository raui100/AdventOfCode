use crate::common::io::read_day;
use crate::common::solution::Solution;

pub struct Day {
    directories: Vec<u32>,
}

impl Day {
    pub fn new() -> Self {
        let mut tmp_dir: Vec<u32> = Vec::new();
        let mut directories: Vec<u32> = Vec::new();

        for line in read_day(7).unwrap().lines() {
            match line.split(' ').collect::<Vec<&str>>().as_slice() {
                ["$", "cd", ".."] => directories.push(tmp_dir.pop().unwrap()),
                ["$", "cd", _] => tmp_dir.push(0),
                [size, _] => {
                    // Getting rid of "dir ..." and "$ ls" here
                    if let Ok(num) = size.parse::<u32>() {
                        tmp_dir.iter_mut().for_each(|n| *n += num)
                    }
                }
                [..] => unreachable!(),
            }
        }
        directories.extend(tmp_dir);

        Self { directories }
    }
}

impl Solution for Day {
    fn name(&self) -> &'static str { "Day 7: No Space Left On Device" }

    fn part_a(&self) -> Option<String> {
        let result = self.directories
                .iter()
                .filter(|&&size| size < 100_000)
                .sum::<u32>();

        Some(result.to_string()) // 2031851

    }

    fn part_b(&self) -> Option<String> {
        let root = *self.directories.iter().max().unwrap();  // Biggest directory
        let required = root + 30_000_000 - 70_000_000;  // Required size for dir
        let result = self.directories
            .iter()
            .filter(|&&dir| dir >= required)
            .min()
            .unwrap();

        Some(result.to_string())  // 2568781
    }
}

