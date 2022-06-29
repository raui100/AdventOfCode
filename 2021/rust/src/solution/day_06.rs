use crate::Solution;

const DATA: &str = include_str!("data/6");

pub struct Day {
    swarm: Swarm,
}

impl Day {
    pub fn new() -> Day {
        let swarm = Swarm::new(DATA);

        Day { swarm }
    }
}

impl Solution for Day {
    fn name(&self) -> &'static str {
        "Day 6: Lanternfish"
    }

    fn part_a(&self) -> Option<String> {
        let mut swarm = self.swarm.clone();
        swarm.next_n_days(80);
        let fish: u64 = swarm.swarm.iter().sum();

        Some(format!("{}", fish))
    }

    fn part_b(&self) -> Option<String> {
        let mut swarm = self.swarm.clone();
        swarm.next_n_days(256);
        let fish: u64 = swarm.swarm.iter().sum();

        Some(format!("{}", fish))
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Swarm {
    swarm: [u64; 9],
}

impl Swarm {
    /// Instantiates a new `Swarm` from a String like "3,4,3,1,2"
    fn new(s: &str) -> Swarm {
        let mut swarm: [u64; 9] = [0; 9];
        for fish in s.split(",") {
            let num = fish.parse::<usize>().expect(&*format!("Failed parsing '{fish}'"));
            assert!(num <= 8);
            swarm[num] += 1;
        }

        Swarm { swarm }
    }

    /// Simulates the growth of the swarm after one day
    fn next_day(&mut self) {
        let new_fish = self.swarm[0];  // This fish spawn new fish
        for ind in 0..(self.swarm.len() - 1) {
            self.swarm[ind] = self.swarm[ind + 1]
        }
        self.swarm[6] += new_fish;
        self.swarm[8] = new_fish;
    }

    /// Simulates the growth of the swarm after n days
    fn next_n_days(&mut self, n: u64) {
        for _ in 0..n {
            self.next_day();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "3,4,3,1,2";

    #[test]
    fn test_swarm() {
        let swarm = Swarm::new(DATA);
        let eq = Swarm { swarm: [0, 1, 1, 2, 1, 0, 0, 0, 0] };
        assert_eq!(swarm, eq);
    }

    #[test]
    fn test_next_day() {
        let mut swarm = Swarm::new(DATA);
        swarm.next_day();
        swarm.next_day();
        let sum: u64 = swarm.swarm.into_iter().enumerate().map(|(ind, val)| ind as u64 * val).sum();
        assert_eq!(sum, [1, 2, 1, 6, 0, 8].iter().sum())  // Information from puzzle
    }

    #[test]
    fn test_next_n_days() {
        let mut swarm = Swarm::new(DATA);
        swarm.next_n_days(18);
        assert_eq!(swarm.swarm.iter().sum::<u64>(), 26);  // Information from puzzle

        let mut swarm = Swarm::new(DATA);
        swarm.next_n_days(80);
        assert_eq!(swarm.swarm.iter().sum::<u64>(), 5934);  // Information from puzzle

        let mut swarm = Swarm::new(DATA);
        swarm.next_n_days(256);
        assert_eq!(swarm.swarm.iter().sum::<u64>(), 26984457539);  // Information from puzzle
    }
}