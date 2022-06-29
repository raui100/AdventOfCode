use crate::Solution;

const DATA: &str = include_str!("./data/7");

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
        "Day 7: The Treachery of Whales"
    }

    fn part_a(&self) -> Option<String> {
        let (_pos, cost) = self.swarm.calc_optimal_position_a();
        Some(format!("{}", cost))
    }

    fn part_b(&self) -> Option<String> {
        let (_pos, cost) = self.swarm.calc_optimal_position_b();
        Some(format!("{}", cost))
    }
}

#[derive(PartialEq, Debug)]
struct Swarm {
    crabs: Vec<u32>,
}

impl Swarm {
    /// Instantiates a new `Swarm` from a String like "3,4,3,1,2"
    fn new(s: &str) -> Swarm {
        let mut crabs: Vec<u32> = Vec::new();
        for crab in s.split(",") {
            let pos = crab.parse::<u32>().expect(&*format!("Failed parsing '{crab}'"));

            crabs.push(pos);
        }

        Swarm { crabs }
    }

    /// Determines the fuel consumption to move all crabs to position `x` (Part: A)
    fn calc_total_distance_to_x(&self, x: u32) -> u32 {
        // Playing around with functional programming.
        // This can be written more verbose but expressive with for loops
        self.crabs.iter()
            .fold(0_u32, |a, b| a + b.abs_diff(x))
    }

    /// Determines the fuel consumption to move all crabs to position `x` (Part: B)
    ///
    /// The fuel consumption isn't linear anymore. To go from x=0 to x=3 the fuel consumption=6
    /// This can be calculated with the Gauß summation
    /// ```\sum_{k=0}^{n} k = \frac{n^2 + n} {2}```
    fn calc_total_fuel_to_x(&self, x: u32) -> u32 {
        let mut cost: u32 = 0;
        for position in &self.crabs {
            let n = position.abs_diff(x);
            let sum = ((n.pow(2) + n) as f32 / 2.0) as u32;  // Always produces integer

            cost += sum
        }

        cost
    }

    /// Determines the x-position that has the minimum total distance of the swarm (Part: A)
    fn calc_optimal_position_a(&self) -> (u32, u32) {
        let mut optimum: (u32, u32) = (0, self.calc_total_distance_to_x(0));
        let max_pos: u32 = self.crabs.iter().max().unwrap().clone();
        for position in 1..=max_pos {
            let dist = self.calc_total_distance_to_x(position);
            if dist <= optimum.1 {
                optimum = (position, dist)
            } else {  // There is only one single global minimum
                break;
            }
        }

        optimum
    }

    /// Determines the x-position that has the minimum fuel consomption for the swarm (Part: B)
    fn calc_optimal_position_b(&self) -> (u32, u32) {
        let mut optimum: (u32, u32) = (0, self.calc_total_fuel_to_x(0));
        let max_pos: u32 = self.crabs.iter().max().unwrap().clone();
        for position in 1..=max_pos {
            let dist = self.calc_total_fuel_to_x(position);
            if dist <= optimum.1 {
                optimum = (position, dist)
            }
            else {  // There is only one single global minimum
                break;
            }
        }

        optimum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_swarm() {
        let swarm = Swarm::new(DATA);
        let eq = Swarm { crabs: vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14] };
        assert_eq!(swarm, eq);
    }

    #[test]
    fn calc_total_distance_to_x() {
        let swarm = Swarm::new(DATA);
        assert_eq!(swarm.calc_total_distance_to_x(2), 37);  // Information from puzzle
        assert_eq!(swarm.calc_total_distance_to_x(10), 71);  // Information from puzzle
    }

    #[test]
    fn calc_total_fuel_to_x() {
        let swarm = Swarm::new(DATA);
        assert_eq!(swarm.calc_total_fuel_to_x(2), 206);  // Information from puzzle
        assert_eq!(swarm.calc_total_fuel_to_x(5), 168);  // Information from puzzle
    }

    #[test]
    fn calc_optimal_position() {
        let swarm = Swarm::new(DATA);
        assert_eq!(swarm.calc_optimal_position_a(), (2, 37));
    }
}