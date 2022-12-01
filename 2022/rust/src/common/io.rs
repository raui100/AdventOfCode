pub fn read_day(day: u8) -> Result<String, String> {
    let day: String = if day < 10 { format!("0{day}") } else {day.to_string()};
    let path = format!("./input/{day}.txt");
    match std::fs::read_to_string(&path) {
        Ok(str) => Ok(str),
        Err(_) => Err(format!("Failed reading: {path}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_day() {
        assert!(read_day(0).is_err());  // There is no day 0 to read
        assert!(read_day(1).is_ok());  // Day 1 exists

    }
}

