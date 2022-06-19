mod common;
mod solution;

use common::{Day, Solution};

fn main() {
    let args = std::env::args();
    let day =  match &args.len() {
        1 => Day(0),
        2 => {
            let number = std::env::args().nth(1).unwrap().parse::<u8>().expect("Failed parsing to int");
            Day(number)
        },
        _ => panic!("Unknown arguments")
    };

    let mut out: String = String::new();
    if day == Day(0) || day == Day(1) {
        out.push_str(&*solution::day_01::Day01::new().solution());
    }
    println!("{out}")
}
