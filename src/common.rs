#[derive(Debug, PartialEq)]
pub struct Day(pub u8);

pub trait Solution {
    fn name(&self) -> &'static str;
    fn part_a(&self) -> String { "".to_string() }
    fn part_b(&self) -> String { "".to_string() }
    fn solution(&self) -> String{
        let part_a = self.part_a();
        let part_b = self.part_b();

        let mut out: String = String::new();
        if &part_a.len() + &part_b.len() > 0 {
            out.push_str(&*format!("Solution for: {}\n", self.name()));
        }

        if &part_a.len() > &0 {
            out.push_str(&*format!("Part A: {}\n", self.part_a()));
        }
        if &part_a.len() > &0 {
            out.push_str(&*format!("Part B: {}\n\n", self.part_b()));
        }

        return out
    }
}
