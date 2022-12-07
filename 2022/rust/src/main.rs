#![feature(is_some_and)]
#![feature(if_let_guard)]

use crate::common::solution::Solution;

mod common;
mod solution;

fn main() {
    let day: u8 = std::env::args().nth(1)
        .unwrap_or_else(|| String::from("0"))
        .parse()
        .expect("Failed parsing the day. Example usage: cargo run -- 1");

    if day == 1 || day == 0 { solution::day_01::Day::new().solution(); }
    if day == 2 || day == 0 { solution::day_02::Day::new().solution(); }
    if day == 3 || day == 0 { solution::day_03::Day::new().solution(); }
    if day == 4 || day == 0 { solution::day_04::Day::new().solution(); }
    if day == 5 || day == 0 { solution::day_05::Day::new().solution(); }
    if day == 6 || day == 0 { solution::day_06::Day::new().solution(); }
    if day == 7 || day == 0 { solution::day_07::Day::new().solution(); }

}
