use crate::common::io::read_day;
use crate::common::solution::Solution;

pub struct Day {
    num: Vec<(u32, u32)>,
    num2: Vec<(u32, u32)>,
}

pub fn parse_nums(input: &str) -> Vec<(u32, u32)> {
    let mut num = Vec::new();
    for line in input.lines() {
        let first = line.chars().find(|c| c.is_numeric()).map(|c| c.to_digit(10));
        let last = line.chars().rev().find(|c| c.is_numeric()).map(|c| c.to_digit(10));
        num.push((first.unwrap().unwrap(), last.unwrap().unwrap()));
    }
    num
}

impl Day {
    pub fn new(input: String) -> Self {
        let num = parse_nums(&input);
        let input = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
        let num2 = parse_nums(&input);

        Self { num, num2 }
    }
}


impl Solution for Day {
    fn name(&self) -> &'static str { 
        "--- Day 1: Trebuchet?! ---"
     }

    fn part_a(&self) -> Option<String> {
        self.num.iter().map(|(a, b)| a * 10 + b).sum::<u32>().to_string().into()
    }

    fn part_b(&self) -> Option<String> {
        self.num2.iter().map(|(a, b)| a * 10 + b).sum::<u32>().to_string().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a() {
        let day = Day::new(A.into());
        assert_eq!(day.part_a(), Some("142".to_owned()));
    }
    fn b() {
        let day = Day::new(B.into());
        assert_eq!(day.part_b(), Some("281".to_owned()));
    }
}

const A: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

const B: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";