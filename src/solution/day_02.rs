use std::str::FromStr;
use crate::common::Solution;

const DATA: &str = include_str!("./data/2");

pub struct Day {
    data: String
}
impl Day {
    pub fn new() -> Day {
        Day {data: DATA.to_string()}
    }
}
enum Command {
    Forward(u8),
    Down(u8),
    Up(u8)
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command: Vec<&str> = s.split_ascii_whitespace().collect();
        assert_eq!(&command.len(), &2, "Expected a command similar to this: 'forward 5'");
        let steps: u8 = command[1].parse().unwrap();
        match command[0] {
            "forward" => Ok(Command::Forward(steps)),
            "down" => Ok(Command::Down(steps)),
            "up" => Ok(Command::Up(steps)),
            _ => Err(())
        }
    }
}

impl Solution for Day {
    fn name(&self) -> &'static str { "Dive!" }

    fn part_a(&self) -> Option<String> {
        let commands = self.data.lines().map(|l| l.parse::<Command>().ok().unwrap()).collect::<Vec<Command>>();
        let mut horizontal = 0_u32;
        let mut depth = 0_u32;
        for command in commands.iter() {
            match command {
                Command::Forward(n) => horizontal += *n as u32,
                Command::Down(n) => depth += *n as u32,
                Command::Up(n) => depth -= *n as u32,

            }
        }
        Some((horizontal * depth).to_string())
    }

    fn part_b(&self) -> Option<String> {
        let commands = self.data.lines().map(|l| l.parse::<Command>().ok().unwrap()).collect::<Vec<Command>>();
        let mut horizontal = 0_u32;
        let mut depth = 0_u32;
        let mut aim = 0_i32;
        for command in commands.iter() {
            match command {
                Command::Forward(n) => {
                    horizontal += *n as u32;
                    depth += (*n as i32 * aim) as u32;
                },
                Command::Down(n) => aim += *n as i32,
                Command::Up(n) => aim -= *n as i32,

            }
        }
        Some((horizontal * depth).to_string())
    }

}