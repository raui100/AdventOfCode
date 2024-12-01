use crate::common::solution::Solution;

const DAY_1: &str = include_str!("./../../input/day_01");

mod common;
mod solution;

fn main() {
    let day: u8 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("0"))
        .parse()
        .expect("Failed parsing the day. Example usage: cargo run -- 1");

    if day == 0 {
        solution::day_01::Day::new(DAY_1).solution();
    }

    match day {
        0 => (),
        1 => solution::day_01::Day::new(DAY_1).solution(),
        _ => todo!("no solution implemented for day {day}"),
    }
}
