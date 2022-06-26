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

fn parse_and_transpose(s: &str) -> Vec<Vec<u8>> {
    let row_len = s.to_string().lines().next().unwrap().len();
    let mut data: Vec<Vec<u8>> = Vec::new();
    for ind in 0..row_len {
        let mut vec: Vec<u8> = Vec::new();
        for row in s.to_string().lines() {
            vec.push(row.chars().nth(ind).unwrap().to_digit(10).unwrap() as u8);
        }
        data.push(vec);
    }
    data
}

impl Solution for Day {
    fn name(&self) -> &'static str { "Day 3: Binary Diagnostic!" }

    fn part_a(&self) -> Option<String> {
        let trans_data = parse_and_transpose(self.data.as_ref());
        let gammas: Vec<u8> = trans_data.iter()
            .map(|column| (column.iter()
                .map(|n| 2 * i32::from(*n) - 1)  // 0 -> -1; 1 -> 1
                .sum::<i32>() >= 0) as u8)  // false -> 0; true -> 1
            .collect();

        let gamma = common::bin_to_int(gammas.clone());
        let epsilon = 2u32.pow(trans_data.len() as u32) - gamma - 1;
        Some(format!("{}", gamma * epsilon))
    }

    fn part_b(&self) -> Option<String> {
        let num_len = self.data.lines().next().unwrap().len();
        let mut o2: Vec<u8> = Vec::new();
        let mut start_o2: String = String::new();
        for ind in 0..num_len {
            let mut counter = 0;
            self.data.lines().filter(|row| row.starts_with(&start_o2)).for_each(|row| match row.chars().nth(ind).unwrap() {
                '1' => counter += 1,
                '0' => counter -= 1,
                err => panic!("Failed parsing: {}", err)
            });
            match counter >= 0 {
                true => {
                    o2.push(1);
                    start_o2.push('1')
                }
                false => {
                    o2.push(0);
                    start_o2.push('0')
                }
            }
        }

        let mut co2: Vec<u8> = Vec::new();
        let mut start_co2: String = String::new();
        for ind in 0..num_len {
            let mut counter = 0;
            let rows = self.data.lines().map(|r| r.to_string()).filter(|row| row.starts_with(&start_co2)).collect::<Vec<String>>();
            if rows.len() == 1 {
                co2 = rows.iter().next().unwrap().chars().map(|c| {
                    match c {
                        '0' => 0,
                        '1' => 1,
                        err => panic!("Failed parsing: {}", err)
                    }
                }).collect();
                break;
            }
            for row in self.data.lines() {
                if row.starts_with(&start_co2) {
                    match row.chars().nth(ind).unwrap() {
                        '1' => counter += 1,
                        '0' => counter -= 1,
                        err => panic!("Failed parsing: {}", err)
                    }
                }
            }
            match counter >= 0 {
                true => {
                    start_co2.push('0')
                }
                false => {
                    start_co2.push('1')
                }
            }
        }
        let o2 = common::bin_to_int(o2);
        let co2 = common::bin_to_int(co2);
        Some(format!("{}", o2 * co2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_parse_and_transpose() {
        let s = "12\n34";
        assert_eq!(parse_and_transpose(s), vec![vec![1, 3], vec![2, 4]])
    }

    #[test]
    fn test_part_a() {
        let day = Day { data: TEST_DATA.to_string() };
        assert_eq!(day.part_a(), Some("198".to_string()))
    }

    #[test]
    fn test_part_b() {
        let day = Day { data: TEST_DATA.to_string() };
        assert_eq!(day.part_b(), Some("230".to_string()))
    }
}