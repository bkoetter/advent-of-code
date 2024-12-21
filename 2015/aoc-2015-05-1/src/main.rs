use std::io::BufRead;

fn get_nice_stings(input: &[u8]) -> usize {
    input
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| {
            line.chars().filter(|&c| "aeiou".contains(c)).count() > 2
                && line.as_bytes().windows(2).any(|w| w[0] == w[1])
                && !["ab", "cd", "pq", "xy"].iter().any(|&x| line.contains(x))
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
