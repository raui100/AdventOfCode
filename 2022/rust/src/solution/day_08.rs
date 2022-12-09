use std::iter;

use crate::common::io::read_day;
use crate::common::solution::Solution;

pub struct Day {
    map: Vec<Vec<u8>>,
    // map[y][x]
    dim: Dimension,
}

impl Day {
    pub fn new() -> Self {
        Self::new_with(read_day(8).unwrap())
    }

    pub fn new_with(input: String) -> Self {
        let map: Vec<Vec<u8>> = input.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();

        Self {
            dim: Dimension { x: map[0].len(), y: map.len() },
            map,
        }
    }

    fn trees_in_sight(&self, x: usize, y: usize) -> [Vec<(usize, usize)>; 4] {
        [
            // From row x to 0 in column y
            (0..x).rev().zip(iter::repeat(y)).collect(),
            // From row x + 1 to border column y
            (x + 1..self.dim.x).zip(iter::repeat(y)).collect(),
            // From row 0 to x in column y
            (0..y).rev().zip(iter::repeat(x)).map(|(a, b)| (b, a)).collect(),
            // From row 0 to x in column y
            (y + 1..self.dim.y).zip(iter::repeat(x)).map(|(a, b)| (b, a)).collect(),
        ]
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == self.dim.x - 1 || y == self.dim.y - 1 {
            true
        } else {
            let my_tree = self.map[y][x];
            'outer: for tree_line in self.trees_in_sight(x, y) {
                for pos in tree_line {
                    let tree = self.map[pos.1][pos.0];
                    if tree >= my_tree {
                        continue 'outer;
                    }
                }
                return true;
            }
            false
        }
    }

    fn visibility(&self, x: usize, y: usize) -> u32 {
        if x == 0 || y == 0 || x == self.dim.x - 1 || y == self.dim.y - 1 {
            0
        } else {
            let mut visibility = 1;
            let my_tree = self.map[y][x];
            'outer: for tree_line in self.trees_in_sight(x, y) {
                let mut tmp_vis = 0;
                for pos in tree_line {
                    tmp_vis += 1;
                    let tree = self.map[pos.1][pos.0];
                    if tree >= my_tree {
                        visibility *= tmp_vis;
                        continue 'outer;
                    }
                }
                visibility *= tmp_vis;
            }
            visibility
        }
    }
}

struct Dimension {
    x: usize,
    y: usize,
}

impl Solution for Day {
    fn name(&self) -> &'static str { "Day 8: Treetop Tree House" }

    fn part_a(&self) -> Option<String> {
        let score: usize = (0..self.dim.x)
            .map(|y| (0..self.dim.x).filter(|&x| self.is_visible(x, y)).count())
            .sum();

        Some(score.to_string())  // 1801
    }

    fn part_b(&self) -> Option<String> {
        let score: u32 = (0..self.dim.y)
            .map(|y| (0..self.dim.x).map(|x| self.visibility(x, y)).max().unwrap())
            .max()
            .unwrap();

        Some(score.to_string())  // 209880
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_trees_in_sight() {
        let day = Day::new_with(INPUT.to_string());
        assert_eq!(
            day.trees_in_sight(0, 0),
            [
                vec![],
                vec![(1, 0), (2, 0), (3, 0), (4, 0)],
                vec![],
                vec![(0, 1), (0, 2), (0, 3), (0, 4)],
            ]
        );
        assert_eq!(
            day.trees_in_sight(1, 1),
            [
                vec![(0, 1)],
                vec![(2, 1), (3, 1), (4, 1)],
                vec![(1, 0)],
                vec![(1, 2), (1, 3), (1, 4)],
            ]
        );
    }

    #[test]
    fn test_is_visible() {
        let day = Day::new_with(INPUT.to_string());
        // All at the border are visible
        for x in 0..day.dim.x {
            assert!(day.is_visible(x, 0));
        }
        assert!(day.is_visible(1, 1));  //
        assert!(!day.is_visible(3, 1));
    }

    #[test]
    fn test_part_a() {
        let day = Day::new_with(INPUT.to_string());
        assert_eq!(day.part_a(), Some("21".to_string()));
    }

    #[test]
    fn test_part_b() {
        let day = Day::new_with(INPUT.to_string());
        assert_eq!(day.part_b(), Some("8".to_string()));
    }
}