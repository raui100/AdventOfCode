pub trait Solution {
    fn name(&self) -> &'static str;
    fn part_a(&self) -> Option<String> { None }
    fn part_b(&self) -> Option<String> { None }

    fn solution(&self) {
        println!("{}", self.name());
        if let Some(a) = self.part_a() { println!("{a}") }
        if let Some(b) = self.part_b() { println!("{b}") }
        println!()
    }
}

