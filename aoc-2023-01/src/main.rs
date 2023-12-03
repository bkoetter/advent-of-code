fn get_first_and_last_number(string: &str) -> u32 {
    format!(
        "{}{}",
        string.chars().find(|c| c.is_ascii_digit()).unwrap_or('0'),
        string.chars().rev().find(|c| c.is_ascii_digit()).unwrap_or('0')
    ).parse::<u32>().unwrap_or(0)
}

fn main() -> std::io::Result<()> {
    println!("{}", std::fs::read_to_string("input.txt")?.lines().map(get_first_and_last_number).sum::<u32>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_and_last_number() {
        assert_eq!(get_first_and_last_number("1abc2"), 12);
        assert_eq!(get_first_and_last_number("pqr3stu8vwx"), 38);
        assert_eq!(get_first_and_last_number("a1b2c3d4e5f6g7h8i9j"), 19);
        assert_eq!(get_first_and_last_number("treb7uchet"), 77);
        assert_eq!(get_first_and_last_number("trebuchet"), 0);
    }
}
