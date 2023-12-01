use common::io::read_day;

use crate::common::solution::Solution;

mod common;
mod solution;

fn main() {
    let day: u8 = std::env::args().nth(1)
        .unwrap_or_else(|| String::from("0"))
        .parse()
        .expect("Failed parsing the day. Example usage: cargo run -- 1");

    if day == 1 || day == 0 { solution::day_01::Day::new(read_day(1)).solution(); }
}
