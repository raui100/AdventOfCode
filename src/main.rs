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

    if day == Day(0) || day == Day(1) {
        println!("{}", &*solution::day_01::Day::new().solution());
    }
    if day == Day(0) || day == Day(2) {
        println!("{}", &*solution::day_02::Day::new().solution());
    }

    if day == Day(0) || day == Day(3) {
        println!("{}", &*solution::day_03::Day::new().solution());
    }

}
