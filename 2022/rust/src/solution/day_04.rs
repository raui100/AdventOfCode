use crate::common::io::read_day;
use crate::common::solution::Solution;
use itertools::Itertools;

struct Section {
    min: u32,
    max: u32,
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlap(&self, other: &Section) -> bool {
        !(self.min > other.max || self.max < other.max)
    }
}

pub struct Day {
    data: Vec<Section>,
}

impl Day {
    pub fn new() -> Self {
        let mut data: Vec<Section> = Vec::new();
        for line in read_day(4).unwrap().lines() {
            for section in line.split(',') {
                let nums: (u32, u32) = section.split('-')
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect_tuple().unwrap();
                data.push(Section {min: nums.0, max: nums.1});
            }
        }

        Self { data }
    }
}


impl Solution for Day {
    fn name(&self) -> &'static str { "Day 4: Camp Cleanup" }

    fn part_a(&self) -> Option<String> {
        let score = self.data.chunks_exact(2)
            .filter(|c| (c[0].contains(&c[1]) || c[1].contains(&c[0])))
            .count();

        Some(score.to_string())  // 547
    }

    fn part_b(&self) -> Option<String> {
        let score = self.data.chunks_exact(2)
            .filter(|c| (c[0].overlap(&c[1]) || c[1].overlap(&c[0])))
            .count();

        Some(score.to_string())  // 843
    }
}

