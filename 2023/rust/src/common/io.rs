pub fn read_day(day: u8) -> String {
    let day: String = if day < 10 { format!("0{day}") } else {day.to_string()};
    let path = format!("./input/{day}.txt");
    match std::fs::read_to_string(&path) {
        Ok(str) => Ok(str),
        Err(_) => Err(format!("Failed reading: {path}"))
    }.unwrap()
}
