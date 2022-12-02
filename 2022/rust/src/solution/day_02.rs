use crate::common::io::read_day;
use crate::common::solution::Solution;
use crate::solution::day_02::Xyz::{X, Y, Z};
use crate::solution::day_02::Abc::{Rock, Paper, Scissor};


enum Abc { Rock, Paper, Scissor }
enum Xyz { X, Y, Z }
pub struct Day { data: Vec<(Abc, Xyz)> }

impl Day {
    pub fn new() -> Self {
        let data: Vec<(Abc, Xyz)> = read_day(2).unwrap().lines()
            .map(|l| {
                let abc = match l.chars().nth(0).unwrap() {
                    'A' => Rock,
                    'B' => Paper,
                    'C' => Scissor,
                    c => panic!("Unexpected char: {}", c)
                };

                let xyz = match l.chars().nth(2).unwrap() {
                    'X' => X,
                    'Y' => Y,
                    'Z' => Z,
                    c => panic!("Unexpected char: {}", c),
                };

                (abc, xyz)
            })
            .collect();
        Day { data }
    }


}


impl Solution for Day {
    fn name(&self) -> &'static str { "Day 2: Rock Paper Scissors" }

    fn part_a(&self) -> Option<String> {
        let mut sum = 0;
        for (abc, xyz) in &self.data {
            sum += match xyz {
                X => 1,
                Y => 2,
                Z => 3,
            };

            sum += match (abc, xyz) {
                (Rock, X) | (Paper, Y) | (Scissor, Z) => 3, // Draw
                (Rock, Y) | (Paper, Z) | (Scissor, X) => 6, // Win
                (Rock, Z) | (Paper, X) | (Scissor, Y) => 0, // Lose
            };
        }

        Some(sum.to_string())  // 10941
    }

    fn part_b(&self) -> Option<String> {
        let mut sum = 0;
        for (abc, xyz) in &self.data {
            sum += match xyz {
                X => 0,  // Lose
                Y => 3,  // Draw
                Z => 6,  // Win
            };

            sum += match (abc, xyz) {
                (Rock, Y) | (Paper, X) | (Scissor, Z) => 1,  // Rock,
                (Rock, Z) | (Paper, Y) | (Scissor, X) => 2,  // Paper,
                (Rock, X) | (Paper, Z) | (Scissor, Y) => 3,  // Scissor
            }
        }

        Some(sum.to_string())  // 13753
    }
}

