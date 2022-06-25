#[derive(Debug, PartialEq)]
pub struct Day(pub u8);

pub trait Solution {
    fn name(&self) -> &'static str;
    fn part_a(&self) -> Option<String> { None }
    fn part_b(&self) -> Option<String> { None }
    fn solution(&self) -> String{
        let part_a = self.part_a();
        let part_b = self.part_b();

        let mut out: String = String::new();
        if part_a.is_some() || part_b.is_some() {
            out.push_str(&*format!("Solution for: {}\n", self.name()));
        }

        if let Some(output) = part_a {
            out.push_str(&*format!("Part A: {}\n", output));
        }
        if let Some(output) = part_b {
            out.push_str(&*format!("Part B: {}\n", output));
        }

        return out
    }
}

/// Casts iterator over integers (eg: Vec<u32>
pub fn bin_to_int(vec: Vec<u8>) -> u32 {
    // Asserts that the entries of `vec` are all either 0 or 1
    debug_assert!(vec.clone().iter().all(|n| *n == 0 || *n == 1));
    // Computes the binary number to an u32 by using bit shifting
    vec.into_iter().fold(0_u32, |sum, n| (sum << 1) + n as u32 )
}
