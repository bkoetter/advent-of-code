fn count_chars(input: &[u8]) -> usize {
    String::from_utf8(input.to_vec()).unwrap().chars().count()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(count_chars(b""), 0)
    }
    #[test]
    fn test_2() {
        assert_eq!(count_chars(b"abc"), 3)
    }
    #[test]
    fn test_3() {
        assert_eq!(count_chars(b"aaa\"aaa"), 7)
    }
}
