use crate::common::solution::Solution;

mod common;
mod solutions;

fn main() {
    let day: u8 = std::env::args().nth(1)
        .unwrap_or(String::from("0"))
        .parse()
        .expect("Failed parsing the day. Example usage: cargo run -- 1");

    if !vec![0_u8].contains(&day) {panic!("Day {day} is not implemented yet")}

    if day == 0 || day == 1 {
        solutions::day_01::Day::new().solution();
    }

}
