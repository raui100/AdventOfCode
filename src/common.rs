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
            out.push_str(&*format!("Part B: {}\n\n", output));
        }

        return out
    }
}
