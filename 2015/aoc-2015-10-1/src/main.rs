fn get_look_and_say(input: Vec<u8>, times: u8) -> usize {
    let mut orig_array = input;

    for _ in 0..times {
        let mut init_char = orig_array[0];
        let mut char_count = 0u8;
        let mut new_array = Vec::new();
        for c in orig_array {
            if init_char == c {
                char_count += 1;
            } else {
                new_array.push(char_count);
                new_array.push(init_char);
                init_char = c;
                char_count = 1;
            }
        }
        new_array.push(char_count);
        new_array.push(init_char);
        orig_array = new_array;
    }
    orig_array.len()
}

fn string_numbers_to_u8_vec(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn main() {
    let input = "1321131112";
    println!("{}", get_look_and_say(string_numbers_to_u8_vec(input), 50));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_look_and_say_1() {
        assert_eq!(get_look_and_say(string_numbers_to_u8_vec("1"), 2), 2)
    }
    #[test]
    fn test_get_look_and_say_2() {
        assert_eq!(get_look_and_say(string_numbers_to_u8_vec("11"), 2), 4)
    }
    #[test]
    fn test_get_look_and_say_3() {
        assert_eq!(get_look_and_say(string_numbers_to_u8_vec("21"), 1), 4)
    }
    #[test]
    fn test_get_look_and_say_4() {
        assert_eq!(get_look_and_say(string_numbers_to_u8_vec("1211"), 1), 6)
    }
    #[test]
    fn test_get_look_and_say_5() {
        assert_eq!(get_look_and_say(string_numbers_to_u8_vec("111221"), 1), 6)
    }
}
