use crate::common::io::read_day;
use crate::common::solution::Solution;


enum Order {
    Unchanged,
    Reversed
}

#[derive(Clone, Debug)]
struct Move {
    number: usize,
    start: usize,
    end: usize,
}

pub struct Day {
    moves: Vec<Move>,
    stacks: Vec<Vec<char>>,
}

impl Day {
    pub fn new() -> Self {
        let input = read_day(5).unwrap();
        let (stack, movements) = input.split_once("\n\n").unwrap();
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
        for line in stack.lines().rev().skip(1) {
            for ind in (0..line.len()).step_by(4) {
                if let Some(char) = line.chars().nth(ind + 1) {
                    if char.is_alphabetic() {
                        stacks[ind / 4].push(char)
                    }
                }
            }
        }

        let moves: Vec<Move> = movements
            .lines()
            .map(|line| line
                .split(' ')
                .filter_map(|num| num.parse::<usize>().ok())
            )
            .map(|mut nums| {
                Move {
                    number: nums.next().unwrap(),
                    start: nums.next().unwrap() - 1,
                    end: nums.next().unwrap() - 1,
                }
            }
            )
            .collect();

        Day {moves, stacks}
    }

    fn solve(&self, order: Order) -> String {
        let mut stacks = self.stacks.clone();
        for mov in &self.moves {
            let ind = stacks[mov.start].len() - mov.number;
            let workload: Vec<char> = stacks[mov.start].drain(ind..).collect();
            match order {
                Order::Unchanged => stacks[mov.end].extend(workload),
                Order::Reversed => stacks[mov.end].extend(workload.iter().rev())
            }
        }

        stacks.iter_mut().map(|stack| stack.pop().unwrap()).collect()
    }
}


impl Solution for Day {
    fn name(&self) -> &'static str { "Day 5: Supply Stacks" }

    fn part_a(&self) -> Option<String> {
        Some(self.solve(Order::Reversed))  // RLFNRTNFB
    }

    fn part_b(&self) -> Option<String> {
        Some(self.solve(Order::Unchanged))  // MHQTLJRLB
    }
}

