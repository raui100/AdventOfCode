use crate::common::Solution;

const DATA: &str = include_str!("./data/2");
const NAME: &str = "Sonar Sweep";

pub struct Day01 {}

impl Day01 {
    fn get_numbers() -> Vec<u32> {
        DATA.lines().map(|l| l.parse::<u32>().expect(format!("Failed parsing {l} to u32").as_str())).collect()
    }
}
impl Solution for Day01 {
    fn name(&self) -> &'static str { NAME }

    fn part_a(&self) -> String {
        let numbers = Day01::get_numbers();
        let increases = numbers.windows(2).filter(|numbers| numbers[0] < numbers[1]).count();
        increases.to_string()
    }

    fn part_b(&self) -> String {
        let numbers = Day01::get_numbers();
        let increases = numbers.windows(4).filter(|numbers| numbers[0] < numbers[3]).count();
        increases.to_string()
    }
}