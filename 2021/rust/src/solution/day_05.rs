use crate::Solution;

const DATA: &str = include_str!("data/5");


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

    fn from_str(s: &str) -> Line {
        let numbers: Vec<i32> = s.split(",").map(|n|
            {
                n.parse::<i32>().expect(&format!("Failed parsing: {}", n))
            })
            .collect();

        debug_assert_eq!(numbers.len(), 4);
        Line(
            Point { x: numbers[0], y: numbers[1] },
            Point { x: numbers[2], y: numbers[3] },
        )
    }
}

pub(crate) struct Day {
    lines: Vec<Line>,
}

impl Day {
    fn parse_data(s: &str) -> Day {
        let data = s.replace(" -> ", ",");
        let lines: Vec<Line> = data.lines().map(|row| Line::from_str(row)).collect();

        Day { lines }
    }

    fn count_overlapping_points(&self, use_diag: bool) -> usize {
        let points: Vec<Point> = self.lines.iter()
            .map(|l| l.extend(use_diag))
            .flatten()
            .collect();

        let max_x = points.iter().fold(0, |init, item| i32::max(init, item.x));
        let max_y = points.iter().fold(0, |init, item| i32::max(init, item.y));
        let mut grid = vec![vec![0; 1 + max_x as usize]; 1 + max_y as usize];
        for point in &points {
            grid[point.y as usize][point.x as usize] += 1;
        }
        let count = grid.iter().flatten().filter(|v| **v >= 2).count();

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
    use super::*;

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
        let s = "0,9,5,9";
        let line = Line(Point { x: 0, y: 9 }, Point { x: 5, y: 9 });
        assert_eq!(Line::from_str(s), line);
    }

    #[test]
    /// Extends a single point
    fn test_point_extend() {
        let p = Point { x: 0, y: 0 };
        // Extending vertically
        let vec_x: Vec<(i32, i32)> = vec![(0, 0), (1, 0)];
        assert_eq!(p.extend(vec_x), vec![Point { x: 0, y: 0 }, Point { x: 1, y: 0 }]);
        // Extending horizontally
        let vec_x: Vec<(i32, i32)> = vec![(0, 0), (0, 1)];
        assert_eq!(p.extend(vec_x), vec![Point { x: 0, y: 0 }, Point { x: 0, y: 1 }]);
        // Extending diagonally
        let vec_x: Vec<(i32, i32)> = vec![(0, 0), (1, 1)];
        assert_eq!(p.extend(vec_x), vec![Point { x: 0, y: 0 }, Point { x: 1, y: 1 }]);
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