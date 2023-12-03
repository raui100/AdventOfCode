use anyhow::{bail, Result};

use crate::common::{solution::Solution, parse::{parse_digit, parse_english_number}};

pub enum Color {
    Blue,
    Red,
    Green,
}

impl<'a> TryFrom<&'a str> for Color {
    type Error=anyhow::Error;
    
    fn try_from(value: &'a str) -> Result<Self> {
        match value {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            value => bail!("Invalid color: {value}"),
        }
    }

}

#[derive(Debug, Clone, Copy)]
pub struct Bag {
    blue: u32,
    red: u32,
    green: u32,
}

impl Bag {
    pub fn subset(&self, other: &Self) -> bool {
        self.blue <= other.blue
        && self.red <= other.red
        && self.green <= other.green

    }
}
pub struct Day {
    input: String,
    bag: Bag,
}

impl Day {
    pub fn new(input: String) -> Self {
        let bag = Bag {red: 12, green: 13, blue: 14};
        Self {input, bag}
    }  
}

#[derive(Debug)]
pub struct Game {
    nr: u32,
    bag: Bag,
}


impl<'a> TryFrom<&'a str> for Game {
    type Error = anyhow::Error;
    fn try_from(value: &'a str) -> Result<Self> {
        // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        // ("Game 1",  "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
        let (head, body) = value.split_once(':').unwrap();
        // "Game 1"
        // ("Game", "1")
        let (_game, nr) = head.split_once(' ').unwrap();
        let nr: u32 = nr.parse().unwrap();   // "1" -> 1
        // "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        // ("3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green")
        let split = body.split(';');
        // "3 blue, 4 red"
        let mut bag = Bag { blue: 0, red: 0, green: 0 };
        for s in split {
            // ["3 blue", "4 red"]
            for cube in s.split(',') {
                let mut cube = cube.split_whitespace();
                // "3"
                let num = cube.next().unwrap();
                let num: u32 = num.parse().unwrap();
                // "blue"
                let color = cube.next().unwrap();
                let color = Color::try_from(color).unwrap();  // Color::Blue
                let n = match color {
                    Color::Blue => &mut bag.blue,
                    Color::Red => &mut bag.red,
                    Color::Green => &mut bag.green,
                };
                *n = num.max(*n);
            }
        }
        Ok(Self {nr, bag})
    }
}

impl Solution for Day {
    fn name(&self) -> &'static str { 
        "--- Day 2: Cube Conundrum ---"
     }

    fn part_a(&self) -> Option<String> {
        let mut sum = 0;
        for line in self.input.lines() {
            let game = Game::try_from(line).unwrap();
            if game.bag.subset(&self.bag) {
                sum += game.nr;
            }
        }
        Some(sum.to_string())
    }

    fn part_b(&self) -> Option<String> {
        let mut sum = 0;
        for line in self.input.lines() {
            let game = Game::try_from(line).unwrap();
            let f = |n: u32| 1.max(n);
            let b = game.bag;
            sum += f(b.red) * f(b.blue) * f(b.green);
        }
        Some(sum.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a() {
        let day = Day::new(A.into());
        assert_eq!(day.part_a(), Some("8".to_owned()));
    }
    #[test]
    fn b() {
        let day = Day::new(A.into());
        assert_eq!(day.part_b(), Some("2286".to_owned()));
    }
    
    const A: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
}