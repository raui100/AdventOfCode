mod common;
mod solution;

use common::Solution;

#[derive(PartialEq)]
struct ArgDay(u8);

fn main() {
    let args = std::env::args();
    let day =  match &args.len() {
        1 => ArgDay(0),
        2 => {
            let number = std::env::args().nth(1).unwrap().parse::<u8>().expect("Failed parsing to int");
            ArgDay(number)
        },
        _ => panic!("Unknown arguments")
    };

    // Day 1
    if day == ArgDay(0) || day == ArgDay(1) {
        println!("{}", &*solution::day_01::Day::new().solution());
    }
    // Day 2
    if day == ArgDay(0) || day == ArgDay(2) {
        println!("{}", &*solution::day_02::Day::new().solution());
    }
    // Day 3
    if day == ArgDay(0) || day == ArgDay(3) {
        println!("{}", &*solution::day_03::Day::new().solution());
    }
    // Day 4
    if day == ArgDay(0) || day == ArgDay(4) {
        println!("{}", &*solution::day_04::Game::new().solution());
    }
    // Day 5
    if day == ArgDay(0) || day == ArgDay(5) {
        println!("{}", &*solution::day_05::Day::new().solution());
    }
    // Day 6
    if day == ArgDay(0) || day == ArgDay(6) {
        println!("{}", &*solution::day_06::Day::new().solution());
    }
    // Day 7
    if day == ArgDay(0) || day == ArgDay(7) {
        println!("{}", &*solution::day_07::Day::new().solution());
    }


}
