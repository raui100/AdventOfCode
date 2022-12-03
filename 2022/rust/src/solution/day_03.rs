use crate::common::solution::Solution;
use std::fs::File;
use std::io::{prelude::*, BufReader};


fn char_to_priority(char: char) -> u8 {
    let num = char as u8;
    if num >= 97 {
        num - 96
    } else {
        num - 38
    }
}

#[test]
fn test_char_to_priority() {
    assert_eq!(char_to_priority('a'), 1);
    assert_eq!(char_to_priority('z'), 26);
    assert_eq!(char_to_priority('A'), 27);
    assert_eq!(char_to_priority('Z'), 52);
}

pub struct Day {
    part_a: u32,
    part_b: u32,
}

impl Day {
    /// To optimize runtime performance Part A and Part B are solved simultaneously
    pub fn new() -> Self {
        let mut score_a: u32 = 0;
        let mut score_b: u32 = 0;
        // Longest String length is known apriori
        let mut strings = vec![String::with_capacity(50); 3];

        let file = File::open("./input/03.txt").unwrap();
        let reader = BufReader::new(file);

        for (ind, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            score_a += Day::part_a(&line) as u32;
            let ind_mod = ind % 3;
            strings[ind_mod] = line;
            if ind % 3 == 2 {
                score_b += Day::part_b(&strings) as u32;
            }

        }

        Day {part_a: score_a, part_b: score_b}
    }

    #[inline(always)]
    fn part_a(line: &str) -> u8 {
        debug_assert_eq!(line.len() % 2, 0);  // The len of the line is always even sized
        let half = line.len() / 2;  // Half length of the line
        for char_1 in line[..half].chars() {
            for char_2 in line[half..].chars() {
                if char_1 == char_2 {
                    return char_to_priority(char_1)
                }
            }
        }
        unreachable!("There should always be an early return in the for loop")
    }

    #[inline(always)]
    fn part_b(strings: &Vec<String>) -> u8 {
        debug_assert_eq!(strings.len(), 3);  // Hopefully this eliminates the bound checks
        for char_1 in strings[0].chars() {
            for char_2 in strings[1].chars() {
                if char_1 == char_2 {
                    for char_3 in strings[2].chars() {
                        if char_1 == char_3 {
                            return char_to_priority(char_1)
                        }
                    }
                }
            }
        }
        unreachable!("There should always be an early return in the for loop")
    }

}


impl Solution for Day {
    fn name(&self) -> &'static str { "Day 3: Rucksack Reorganization" }

    fn part_a(&self) -> Option<String> {
        Some(self.part_a.to_string())  // 7848
    }

    fn part_b(&self) -> Option<String> {
        Some(self.part_b.to_string())  // 2616
    }
}

