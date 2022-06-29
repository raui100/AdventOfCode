use crate::solution::day_04::Number::{Marked, Unmarked};
use crate::Solution;

const DATA: &str = include_str!("data/4");

#[derive(Copy, Clone, Debug, PartialEq)]
enum Number {
    Marked(u32),
    Unmarked(u32),
}

#[derive(Copy, Clone, Debug)]
struct Grid {
    matrix: [[Number; 5]; 5],
}

impl Grid {
    fn new(s: &str) -> Grid {
        let mut grid = Grid {
            matrix: [[Unmarked(0); 5]; 5],
        };
        for (ind_row, row) in s.split("\n").enumerate() {
            for (ind_num, num) in row.split_whitespace().enumerate() {
                assert!(!num.is_empty(), "An empty string slipped through");
                let n = num
                    .parse::<u32>()
                    .expect(&format!("Failed parsing: {}", num));
                grid.matrix[ind_row][ind_num] = Unmarked(n);
            }
        }

        grid
    }

    fn sum_unmarked(&self) -> u32 {
        let mut counter: u32 = 0;
        for row in self.matrix {
            for number in row {
                match number {
                    Marked(_) => (),
                    Unmarked(n) => counter += n,
                }
            }
        }

        counter
    }

    fn has_won(&self) -> bool {
        for row in self.matrix {
            let counter = row.iter().filter(|n| matches!(n, Marked(_))).count();
            if counter == 5 {
                return true;
            }
        }

        for ind in 0..self.matrix[0].len() {
            let mut counter = 0;
            for row in self.matrix {
                let number = row[ind];
                match number {
                    Marked(_) => counter += 1,
                    _ => (),
                }
            }
            if counter == 5 {
                return true;
            }
        }

        false
    }

    fn set_marked(&mut self, mark_num: u32) {
        for column in 0..self.matrix.len() {
            for row in 0..self.matrix[0].len() {
                let number = &mut self.matrix[column][row];
                if *number == Unmarked(mark_num) {
                    *number = Marked(mark_num)
                }
            }
        }
    }
}

pub struct Game {
    grids: Vec<Grid>,
    numbers: Vec<u32>,
}

impl Game {
    fn parse_input(input: &str) -> Game {
        let parsed_input = input.split("\n\n").collect::<Vec<&str>>();
        let numbers = parsed_input[0]
            .split(",")
            .map(|n| n.parse::<u32>().expect(&format!("Failed parsing: {}", n)))
            .collect::<Vec<u32>>();
        let grids = parsed_input[1..]
            .iter()
            .map(|grid| Grid::new(grid))
            .collect::<Vec<Grid>>();

        Game { grids, numbers }
    }

    pub fn new() -> Game {
        Game::parse_input(DATA)
    }

    fn play(&mut self) -> Vec<String> {
        let mut filter: Vec<usize> = Vec::new();
        let mut winnings: Vec<String> = Vec::new();
        for number in &self.numbers {
            for (ind, grid) in &mut self.grids.iter_mut().enumerate() {
                if !filter.contains(&ind) {
                    grid.set_marked(*number);
                    if grid.has_won() {
                        filter.push(ind);
                        winnings.push(format!("{}", grid.sum_unmarked() * number));
                    }
                }
            }
        }
        winnings
    }
}

impl Solution for Game {
    fn name(&self) -> &'static str {
        "Day 4: Giant Squid"
    }

    fn part_a(&self) -> Option<String> {
        let mut game = Game::new();
        let s = game.play().first().unwrap().clone();
        Some(s)
    }

    fn part_b(&self) -> Option<String> {
        let mut game = Game::new();
        let s = game.play().last().unwrap().clone();
        Some(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    macro_rules! cast {
        ($target: expr, $pat: path) => {{
            if let $pat(a) = $target {
                // #1
                a
            } else {
                panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
            }
        }};
    }

    #[test]
    fn test_parse_input() {
        let game = Game::parse_input(TEST_DATA);
        assert_eq!(
            game.numbers,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(game.grids[0].matrix[0][0], Unmarked(22));
    }

    #[test]
    fn test_count_unmarked() {
        let mut game = Game::parse_input(TEST_DATA);
        assert_eq!(game.grids[0].sum_unmarked(), 300);
        game.grids[0].matrix[0][0] = Marked(22);
        assert_eq!(game.grids[0].sum_unmarked(), 278);
    }

    #[test]
    fn test_has_won_loosing() {
        let game = Game::parse_input(TEST_DATA);
        assert_eq!(game.grids[0].has_won(), false);
    }

    #[test]
    fn test_has_won_row() {
        let mut game = Game::parse_input(TEST_DATA);

        for ind in 0..game.grids[0].matrix[0].len() {
            let number = &mut game.grids[0].matrix[0][ind];
            let n = cast!(number, Unmarked);
            *number = Marked(*n)
        }
        assert_eq!(game.grids[0].has_won(), true)
    }

    #[test]
    fn test_has_won_columns() {
        let mut game = Game::parse_input(TEST_DATA);

        for ind in 0..game.grids[0].matrix[0].len() {
            let number = &mut game.grids[0].matrix[ind][0];
            let n = cast!(number, Unmarked);
            *number = Marked(*n)
        }
        assert_eq!(game.grids[0].has_won(), true)
    }

    #[test]
    fn test_set_marked() {
        let mut game = Game::parse_input(TEST_DATA);
        assert_eq!(game.grids[0].matrix[0][0], Unmarked(22));
        game.grids[0].set_marked(22);
        assert_eq!(game.grids[0].matrix[0][0], Marked(22));
    }

    #[test]
    fn test_play() {
        let mut game = Game::parse_input(TEST_DATA);
        assert_eq!(game.play()[0], "4512".to_string())
    }
}

