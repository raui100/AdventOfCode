use crate::common::io::read_day;
use crate::common::solution::Solution;

pub struct Day {}

impl Day {
    pub fn new(input: &str) -> Self {
        Self {}
    }
}

impl Solution for Day {
    fn name() -> &'static str {
        todo!()
    }

    fn part_a(&self) -> Option<String> {
        None
    }

    fn part_b(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a() {
        let day = Day::new(A.into());
        assert_eq!(day.part_a(), None);
        // assert_eq!(day.part_a(), Some("".to_owned()));
    }
    #[test]
    fn b() {
        let day = Day::new(B.into());
        assert_eq!(day.part_b(), None);
        // assert_eq!(day.part_b(), Some("".to_owned()));
    }

    const A: &'static str = "";
    const B: &'static str = "";
}
