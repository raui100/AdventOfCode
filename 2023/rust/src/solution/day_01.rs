use crate::common::{solution::Solution, parse::{parse_digit, parse_english_number}};

pub struct Day {
    input: String,
}

impl Day {
    pub fn new(input: String) -> Self {
        Self {input}
    }

    pub fn parse(&self, func: Vec<fn(&str) -> Option<u32>>) -> u32 {
        let mut sum = 0;
        let mut sub: String;  // reusing buffer
        for line in self.input.lines() {
            let len = line.len();
            let (mut start, mut end) = (None, None);
            'outer: for i in 1..=len {
                sub = line.chars().take(i).collect();  // first i chars
                for f in &func {
                    if let Some(n) = f(&sub) {
                        start = Some(n);
                        break 'outer;
                    }
                }

            }
            'outer: for i in (0..len).rev() {
                sub = line.chars().skip(i).collect();  // last i chars
                for f in &func {
                    if let Some(n) = f(&sub) {
                        end = Some(n);
                        break 'outer;
                    }
                }

            }
            if let (Some(start), Some(end)) = (start, end) {
                sum += start * 10 + end;
            } else {
                panic!("{line}")
            }
        };
        sum
    }

    
}


impl Solution for Day {
    fn name(&self) -> &'static str { 
        "--- Day 1: Trebuchet?! ---"
     }

    fn part_a(&self) -> Option<String> {
        let sum = self.parse(vec![parse_digit]);
        Some(sum.to_string())
    }

    fn part_b(&self) -> Option<String> {
        let sum = self.parse(vec![parse_digit, parse_english_number]);
        Some(sum.to_string())
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
    #[test]
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