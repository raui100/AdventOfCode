use crate::common::Solution;

pub const DATA: &str = include_str!("./data/1");

pub struct Day {
    pub data: String
}

impl Day {
    pub fn new() -> Day {
        Day {data: DATA.to_string()}
    }
}

impl Day {
    fn get_numbers(&self) -> Vec<u32> {
        self.data.lines().map(|l| l.parse::<u32>().expect(format!("Failed parsing {l} to u32").as_str())).collect()
    }
}
impl Solution for Day {
    fn name(&self) -> &'static str { "Sonar Sweep" }

    fn part_a(&self) -> String {
        let numbers = self.get_numbers();
        let increases = numbers.windows(2).filter(|numbers| numbers[0] < numbers[1]).count();
        increases.to_string()
    }

    fn part_b(&self) -> String {
        let numbers = self.get_numbers();
        let increases = numbers.windows(4).filter(|numbers| numbers[0] < numbers[3]).count();
        increases.to_string()

    }
}