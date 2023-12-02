pub fn parse_digit(s: &str) -> Option<u32> {
    match s {
        _ if s.contains('1') => Some(1),
        _ if s.contains('2') => Some(2),
        _ if s.contains('3') => Some(3),
        _ if s.contains('4') => Some(4),
        _ if s.contains('5') => Some(5),
        _ if s.contains('6') => Some(6),
        _ if s.contains('7') => Some(7),
        _ if s.contains('8') => Some(8),
        _ if s.contains('9') => Some(9),
        _ => None,    
    }
}

pub fn parse_english_number(s: &str) -> Option<u32> {
    match s {
        _ if s.contains("one") => Some(1),
        _ if s.contains("two") => Some(2),
        _ if s.contains("three") => Some(3),
        _ if s.contains("four") => Some(4),
        _ if s.contains("five") => Some(5),
        _ if s.contains("six") => Some(6),
        _ if s.contains("seven") => Some(7),
        _ if s.contains("eight") => Some(8),
        _ if s.contains("nine") => Some(9),
        _ => None,    
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_digit() {
        let f = parse_digit;
        assert_eq!(f("1"), Some(1));
        assert_eq!(f("a1"), Some(1));
        assert_eq!(f("1b"), Some(1));
        assert_eq!(f("12"), Some(1));
        assert_eq!(f("a"), None);
    }

    #[test]
    fn test_parse_english_number() {
        let f = parse_english_number;
        assert_eq!(f("ooone"), Some(1));
        assert_eq!(f("ootwo"), Some(2));
        assert_eq!(f("oothree"), Some(3));
        assert_eq!(f("oofour"), Some(4));
        assert_eq!(f("oofive"), Some(5));
        assert_eq!(f("oosix"), Some(6));
        assert_eq!(f("ooseven"), Some(7));
        assert_eq!(f("ooeight"), Some(8));
        assert_eq!(f("oonine"), Some(9));
    }
}