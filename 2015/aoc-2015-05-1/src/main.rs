use std::io::BufRead;

fn get_nice_stings(input: &[u8]) -> u16 {
    let mut nice_string_count = 0;
    for line in input.lines().map(|line| line.unwrap()) {
        if line
            .chars()
            .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c))
            .count()
            < 3
        {
            continue;
        }
        let mut previous_char: char = ' ';
        let mut is_char_same = false;
        for char in line.chars() {
            if char.eq(&previous_char) {
                is_char_same = true;
                break;
            }
            previous_char = char;
        }
        if !is_char_same {
            continue;
        }
        // if line.contains("ab") || line.contains("cd") || line.contains("xy") || line.contains("pq")
        if ["ab", "cd", "pq", "xy"]
            .iter()
            .find(|&x| line.contains(x))
            .iter()
            .count()
            > 0
        {
            continue;
        }
        nice_string_count += 1;
    }
    nice_string_count
}

fn main() {
    println!("{}", get_nice_stings(include_bytes!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nice_stings_1() {
        assert_eq!(get_nice_stings(b"ugknbfddgicrmopn"), 1)
    }
    #[test]
    fn test_get_nice_stings_2() {
        assert_eq!(get_nice_stings(b"aaa"), 1)
    }
    #[test]
    fn test_get_nice_stings_3() {
        assert_eq!(get_nice_stings(b"jchzalrnumimnmhp"), 0)
    }
    #[test]
    fn test_get_nice_stings_4() {
        assert_eq!(get_nice_stings(b"haegwjzuvuyypxyu"), 0)
    }
    #[test]
    fn test_get_nice_stings_5() {
        assert_eq!(get_nice_stings(b"dvszwmarrgswjxmb"), 0)
    }
}
