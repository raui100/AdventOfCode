use crate::common::solution::Solution;

const DAY_1: &str = include_str!("./../../input/day_01");
const DAY_2: &str = include_str!("./../../input/day_02");
const DAY_3: &str = include_str!("./../../input/day_03");

mod common;
mod solution;

fn main() {
    let day: u8 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("0"))
        .parse()
        .expect("Failed parsing the day. Example usage: cargo run -- 1");

    match day {
        0 => {
            solution::day_01::Day1::new(DAY_1).solution();
            solution::day_02::Day2::new(DAY_2).solution();
            solution::day_03::Day3::new(DAY_3).solution();
        }
        1 => solution::day_01::Day1::new(DAY_1).solution(),
        2 => solution::day_02::Day2::new(DAY_2).solution(),
        3 => solution::day_03::Day3::new(DAY_3).solution(),
        _ => todo!("no solution implemented for day {day}"),
    }
}
