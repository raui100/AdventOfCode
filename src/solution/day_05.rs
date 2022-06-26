use regex::Regex;

use crate::common::count_frequency;
use crate::Solution;

const DATA: &str = include_str!("./data/5");


#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Line(Point, Point);

impl Point {
    fn extend(&self, extend: Vec<(i32, i32)>) -> Vec<Point> {
        extend.iter()
            .map(|(x, y)| Point { x: self.x + x, y: self.y + y })
            .collect()
    }
}

impl Line {
    fn to_points_vertically(&self) -> Vec<Point> {
        let delta = self.1.x - self.0.x;
        let vec_x = match delta >= 0 {
            true => (0..=delta).collect::<Vec<i32>>(),
            false => (delta..=0).collect::<Vec<i32>>(),
        };
        let vec_y: Vec<i32> = vec![0; delta.abs() as usize + 1];
        let vec: Vec<(i32, i32)> = vec_x.iter()
            .zip(vec_y.iter())
            .map(|(x, y)| (*x, *y))
            .collect();

        self.0.extend(vec)
    }

    fn to_points_horizontally(&self) -> Vec<Point> {
        let delta = self.1.y - self.0.y;
        let vec_y = match delta >= 0 {
            true => (0..=delta).collect::<Vec<i32>>(),
            false => (delta..=0).collect::<Vec<i32>>(),
        };
        let vec_x: Vec<i32> = vec![0; delta.abs() as usize + 1];
        let vec: Vec<(i32, i32)> = vec_x.iter()
            .zip(vec_y.iter())
            .map(|(x, y)| (*x, *y))
            .collect();

        self.0.extend(vec)
    }

    fn to_points_diagonally(&self) -> Vec<Point> {
        let delta_x = self.1.x - self.0.x;
        let delta_y = self.1.y - self.0.y;
        debug_assert_eq!(delta_y.abs(), delta_x.abs());
        let vec_x = match delta_x >= 0 {
            true => (0..=delta_x).collect::<Vec<i32>>(),
            false => (delta_x..=0).rev().collect::<Vec<i32>>(),
        };
        let vec_y = match delta_y >= 0 {
            true => (0..=delta_y).collect::<Vec<i32>>(),
            false => (delta_y..=0).rev().collect::<Vec<i32>>(),
        };
        let vec: Vec<(i32, i32)> = vec_x.iter()
            .zip(vec_y.iter())
            .map(|(x, y)| (*x, *y))
            .collect();

        self.0.extend(vec)
    }

    fn extend(&self, use_diag: bool) -> Vec<Point> {
        return {
            if self.0.x != self.1.x && self.0.y == self.1.y {
                self.to_points_vertically()
            } else if self.0.x == self.1.x && self.0.y != self.1.y {
                self.to_points_horizontally()
            } else if use_diag {
                self.to_points_diagonally()
            } else {
                Vec::new()
            }
        };
    }

    fn from_str(s: &str, re: &Regex) -> Line {
        for matches in re.captures_iter(s) {
            return Line(
                Point {
                    x: matches[1].parse::<i32>().expect(&format!("Failed parsing: {}", &matches[0])),
                    y: matches[2].parse::<i32>().expect(&format!("Failed parsing: {}", &matches[0])),
                },
                Point {
                    x: matches[3].parse::<i32>().expect(&format!("Failed parsing: {}", &matches[0])),
                    y: matches[4].parse::<i32>().expect(&format!("Failed parsing: {}", &matches[0])),
                });
        }
        panic!("Failed matching: {}", s)
    }
}

pub(crate) struct Day {
    lines: Vec<Line>,
}

impl Day {
    fn parse_data(s: &str) -> Day {
        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        let lines: Vec<Line> = s.lines().map(|row| Line::from_str(row, &re)).collect();

        Day { lines }
    }

    fn count_overlapping_points(&self, use_diag: bool) -> usize {

        let points: Vec<Point> = self.lines.iter().map(|l| {
            let p = l.extend(use_diag);
            // dbg!(&p);
            p
        }).flatten().collect();
        let map = count_frequency(points);
        let count = map.values().filter(|v| **v >= 2).count();

        count
    }

    pub fn new() -> Day { Day::parse_data(DATA) }
}

impl Solution for Day {
    fn name(&self) -> &'static str { "Day 5: Hydrothermal Venture" }

    fn part_a(&self) -> Option<String> {
        let count = self.count_overlapping_points(false);
        Some(format!("{}", count))
    }

    fn part_b(&self) -> Option<String> {
        let count = self.count_overlapping_points(true);
        Some(format!("{}", count))
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use crate::solution::day_05::{Day, Line, Point};

    const TEST_DATA: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";


    #[test]
    /// Parses a single row of the input
    fn test_line_from_str() {
        let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        let line = Line(Point { x: 0, y: 9 }, Point { x: 5, y: 9 });
        assert_eq!(Line::from_str(TEST_DATA.lines().next().unwrap(), &re), line);
    }

    #[test]
    /// Extends a single point
    fn test_point_extend() {
        let p = Point {x: 0, y: 0};
        // Extending vertically
        let vec_x: Vec<(i32, i32)> = vec![(0, 0), (1, 0)];
        assert_eq!(p.extend(vec_x), vec![Point {x: 0, y: 0}, Point {x: 1, y: 0}]);
        // Extending horizontally
        let vec_x: Vec<(i32, i32)> = vec![(0, 0), (0, 1)];
        assert_eq!(p.extend(vec_x), vec![Point {x: 0, y: 0}, Point {x: 0, y: 1}]);
        // Extending diagonally
        let vec_x: Vec<(i32, i32)> = vec![(0, 0), (1, 1)];
        assert_eq!(p.extend(vec_x), vec![Point {x: 0, y: 0}, Point {x: 1, y: 1}]);
    }

    #[test]
    fn test_line_to_points() {
        let l = Line(Point { x: 0, y: 0 }, Point { x: 1, y: 1 });
        // Extending vertically
        assert_eq!(l.to_points_vertically(), vec![Point { x: 0, y: 0 }, Point { x: 1, y: 0 }]);
        // Extending horizontally
        assert_eq!(l.to_points_horizontally(), vec![Point { x: 0, y: 0 }, Point { x: 0, y: 1 }]);
        // Extending diagonally
        assert_eq!(l.to_points_diagonally(), vec![Point { x: 0, y: 0 }, Point { x: 1, y: 1 }]);
    }

    #[test]
    fn test_count_overlapping_points() {
        let day = Day::parse_data(TEST_DATA);
        let count = day.count_overlapping_points(false);
        assert_eq!(count, 5);
        let day = Day::parse_data(TEST_DATA);
        let count = day.count_overlapping_points(true);
        assert_eq!(count, 12);
    }

}