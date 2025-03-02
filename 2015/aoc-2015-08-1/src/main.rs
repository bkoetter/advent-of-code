use evalexpr::*;
use regex::Regex;
use std::process::exit;

fn count_chars(input: &str) -> usize {
    let re = Regex::new(r#"\\x[0-9a-f]"#).unwrap();
    input
        .lines()
        .map(|line| (line.len(), re.replace_all(line, "")))
        .map(|line| match eval_string(&line.1) {
            Err(e) => {
                eprintln!("{e}");
                exit(1)
            }
            Ok(string) => {
                // println!("'{string}' -> {} characters", string.len());
                line.0 - string.len()
            }
        })
        .sum()
}

fn main() {
    println!("{}", count_chars(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(count_chars(r#""""#), 0)
    }
    #[test]
    fn test_2() {
        assert_eq!(count_chars(r#""abc""#), 3)
    }
    #[test]
    fn test_3() {
        assert_eq!(count_chars(r#""aaa\"aaa""#), 7)
    }
    #[test]
    fn test_4() {
        // assert_eq!(count_chars(r#""\x27""#), 1)
        assert_eq!("\x27".len(), 1)
    }
    #[test]
    fn test_5() {
        assert_eq!(count_chars(r#""\"""#), 1)
    }
}
