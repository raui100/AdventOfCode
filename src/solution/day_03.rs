use crate::common;
use crate::common::Solution;

const DATA: &str = include_str!("./data/3");

pub struct Day {
    data: String,
}

impl Day {
    pub fn new() -> Day {
        Day { data: DATA.to_string() }
    }
}

impl Solution for Day {
    fn name(&self) -> &'static str { "Binary Diagnostic!" }

    fn part_a(&self) -> Option<String> {
        let num_len = self.data.lines().next().unwrap().len();
        let mut gammas: Vec<u8> = Vec::new();
        for ind in 0..num_len {
            let mut gamma = 0;
            self.data.lines().for_each(|row| match row.chars().nth(ind).unwrap() {
                '0' => gamma += 1,
                '1' => gamma -= 1,
                err => panic!("Failed parsing: {}", err)
            });
            match gamma > 0 {
                true => gammas.push(1),
                false => gammas.push(0),
            }
        }
        let gamma = common::bin_to_int(gammas);
        let epsilon = 2u32.pow(num_len as u32) - gamma - 1;
        Some(format!("{}", gamma * epsilon))
    }
}