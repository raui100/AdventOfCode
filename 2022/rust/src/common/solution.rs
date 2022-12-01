pub trait Solution {
    fn name(&self) -> &'static str;
    fn part_a(&self) -> Option<String> { None }
    fn part_b(&self) -> Option<String> { None }

    fn solution(&self) {
        match (self.part_a(), self.part_b()) {
            (None, None) => panic!("Has not been implemented: {}", self.name()),
            (a, b) => {
                println!("{}", self.name());
                if let Some(a) = a { println!("{a}") }
                if let Some(b) = b { println!("{b}") }
                println!();
            }
        };
    }
}

