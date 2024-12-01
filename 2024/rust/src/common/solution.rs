pub trait Solution {
    fn new(input: &str) -> Self;
    fn name() -> &'static str;
    fn part_a(&self) -> Option<String> {
        None
    }
    fn part_b(&self) -> Option<String> {
        None
    }

    fn solution(&self) {
        println!("{}", Self::name());
        if let Some(a) = self.part_a() {
            println!("{a}")
        }
        if let Some(b) = self.part_b() {
            println!("{b}")
        }
        println!()
    }
}
