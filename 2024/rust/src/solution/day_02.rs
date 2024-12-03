use std::cmp::Ordering;

use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};

use crate::common::{parse::parse_input, solution::Solution};

/// State machine to check for proper safety
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CheckSafety {
    #[default]
    Uninit,
    One(u32),
    Increase(u32, u32),
    Decrease(u32, u32),
    Unsafe,
}

impl CheckSafety {
    pub fn check_ordered(self, n: u32) -> Self {
        match self {
            Self::Uninit => Self::One(n),
            Self::One(a) => match n.cmp(&a) {
                Ordering::Less => Self::Decrease(a, n),
                Ordering::Equal => Self::Unsafe, // strictly increasing/decreasing only
                Ordering::Greater => Self::Increase(a, n),
            },
            Self::Increase(_, a) => match n.cmp(&a) {
                Ordering::Less | Ordering::Equal => Self::Unsafe,
                Ordering::Greater => Self::Increase(a, n),
            },
            Self::Decrease(_, a) => match n.cmp(&a) {
                Ordering::Less => Self::Decrease(a, n),
                Ordering::Equal | Ordering::Greater => Self::Unsafe,
            },
            Self::Unsafe => Self::Unsafe,
        }
    }

    pub fn check_distance(self, safe_distance: u32) -> Self {
        match self {
            Self::Increase(a, b) | Self::Decrease(a, b) => match a.abs_diff(b) <= safe_distance {
                true => self,
                false => Self::Unsafe,
            },
            _ => self,
        }
    }
}

pub fn safe_report<I: IntoIterator<Item = u32>>(it: I) -> bool {
    let mut check = CheckSafety::default();
    for n in it {
        check = check.check_ordered(n).check_distance(3);
        if check == CheckSafety::Unsafe {
            return false;
        }
    }
    true
}

pub struct Day2 {
    map: Vec<Vec<u32>>,
}

impl Solution for Day2 {
    fn name() -> &'static str {
        "--- Day 2: Red-Nosed Reports ---"
    }

    fn new(input: &str) -> Self {
        let (_, map) = parse_input(input, parse_row).unwrap();
        Self { map }
    }

    fn part_a(&self) -> Option<String> {
        let mut result = 0;
        for row in self.map.clone() {
            if safe_report(row) {
                result += 1;
            }
        }

        Some(result.to_string())
    }

    fn part_b(&self) -> Option<String> {
        let mut result = 0;
        for row in &self.map {
            for skip in 0..=row.len() {
                let it = row.iter().copied();
                let it = it.clone().take(skip).chain(it.skip(skip + 1));
                if safe_report(it) {
                    result += 1;
                    break;
                }
            }
        }
        Some(result.to_string())
    }
}

fn parse_row(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, complete::u32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a() {
        let day = Day2::new(A.into());
        assert_eq!(day.part_a().unwrap(), "2");
        // assert_eq!(day.part_a(), Some("".to_owned()));
    }
    #[test]
    fn b() {
        let day = Day2::new(A.into());
        assert_eq!(day.part_b().unwrap(), "4");
        // assert_eq!(day.part_b(), Some("".to_owned()));
    }

    const A: &'static str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
}
