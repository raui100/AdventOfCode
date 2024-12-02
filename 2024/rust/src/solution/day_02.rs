use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};

use crate::common::{parse::parse_input, solution::Solution};

pub fn check_ordered<I: IntoIterator<Item = u32>>(it: I) -> bool {
    let mut it = it.into_iter();
    if let Some(mut a) = it.next() {
        let mut ord: Option<std::cmp::Ordering> = None;
        while let Some(b) = it.next() {
            // Check difference
            if a.abs_diff(b) >= 4 {
                return false;
            }

            // Check ordering
            match ord {
                Some(ord) => {
                    if ord != a.cmp(&b) {
                        return false;
                    }
                }
                None if a == b => return false, // must be strictly ordered
                None => ord = Some(a.cmp(&b)),
            }

            a = b;
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
            result += check_ordered(row) as u32;
        }
        Some(result.to_string())
    }

    fn part_b(&self) -> Option<String> {
        let mut result = 0;
        for row in &self.map {
            for skip in 0..=row.len() {
                let it = row.iter().copied();
                let it = it.clone().take(skip).chain(it.skip(skip + 1));
                if check_ordered(it) {
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
