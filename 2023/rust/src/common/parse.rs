pub fn parse_digit(s: &str) -> Option<u32> {
    let mut digits = Vec::new();
    for (i, d) in ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].into_iter().enumerate() {
        if let Some(p) = s.find(d) {
            digits.push((p, 1 + i as u32));
        }
    }
    digits.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    digits.first().map(|(_, i) |*i)
}