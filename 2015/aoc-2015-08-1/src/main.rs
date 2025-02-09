fn count_chars(input: &[u8]) -> usize {
    let string = String::from_utf8(input.to_vec()).unwrap();
        println!("{string}");
    2
}

fn main() {
    count_chars(r#""""#.as_ref());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(count_chars(r#""""#.as_ref()), 2)
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
