use std::io::BufRead;
use std::str;

fn get_nice_stings(input: &[u8]) -> usize {
    input
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| {
            line.as_bytes()
                .windows(2)
                .any(|w| line.matches(str::from_utf8(w).unwrap()).count() > 1)
                && line.as_bytes().windows(3).any(|w| w[0] == w[2])
        })
        .count()
}

fn main() {
    println!("{}", get_nice_stings(include_bytes!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_string_1() {
        assert_eq!(get_nice_stings(b"qjhvhtzxzqqjkmpb"), 1)
    }
    #[test]
    fn test_nice_string_2() {
        assert_eq!(get_nice_stings(b"xxyxx"), 1)
    }
    #[test]
    fn test_nice_string_3() {
        assert_eq!(get_nice_stings(b"uurcxstgmygtbstg"), 0)
    }
    #[test]
    fn test_nice_string_4() {
        assert_eq!(get_nice_stings(b"ieodomkazucvgmuy"), 0)
    }
    #[test]
    fn test_nice_string_5() {
        assert_eq!(get_nice_stings(b"aaa"), 0)
    }
    #[test]
    fn test_nice_string_6() {
        assert_eq!(get_nice_stings(b"aaaa"), 1)
    }
}
