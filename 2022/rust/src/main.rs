use crate::common::solution::Solution;

mod common;
mod solution;

fn main() {
    let day: u8 = std::env::args().nth(1)
        .unwrap_or_else(|| String::from("0"))
        .parse()
        .expect("Failed parsing the day. Example usage: cargo run -- 1");

    if !vec![0_u8, 1, 2, 3].contains(&day) {
        panic!("Day {day} is not implemented yet")
    }

    if day == 1 || day == 0 { solution::day_01::Day::new().solution(); }
    if day == 2 || day == 0 { solution::day_02::Day::new().solution(); }
    if day == 3 || day == 0 { solution::day_03::Day::new().solution(); }

}
