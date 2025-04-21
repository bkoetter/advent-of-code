fn password_increment(input: &[u8]) -> Vec<u8> {
    let mut result = input.to_vec();

    for c in result.iter_mut().rev() {
        if *c == b'z' {
            *c = b'a';
        } else {
            *c += 1;
            break;
        }
    }
    result
}

fn get_new_password(input: &[u8]) -> Vec<u8> {
    let mut new_password = password_increment(input);
    while !new_password
        .windows(3)
        .any(|x| x == [x[0], x[0] + 1, x[0] + 2])
        || new_password
            .windows(2)
            .fold((None, 0), |acc, x| {
                if x == [x[0], x[0]] && acc.0.is_none_or(|b| b != x[0]) {
                    (Some(x[0]), acc.1 + 1)
                } else {
                    acc
                }
            })
            .1
            < 2
        || (new_password.contains(&b'i')
            || new_password.contains(&b'l')
            || new_password.contains(&b'o'))
    {
        new_password = password_increment(&new_password);
    }
    new_password
}

fn main() {
    let new_password = get_new_password(b"hepxcrrq");
    println!("New password: {}", String::from_utf8_lossy(&new_password));
    let new_password = get_new_password(&new_password);
    println!("New password: {}", String::from_utf8_lossy(&new_password));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_increment_1() {
        assert_eq!(password_increment(b"axx"), b"axy")
    }
    #[test]
    fn test_password_increment_2() {
        assert_eq!(password_increment(b"axy"), b"axz")
    }
    #[test]
    fn test_password_increment_3() {
        assert_eq!(password_increment(b"axz"), b"aya")
    }
    #[test]
    fn test_password_increment_5() {
        assert_eq!(password_increment(b"aaaaaaaa"), b"aaaaaaab")
    }
    #[test]
    fn test_password_increment_6() {
        assert_eq!(password_increment(b"yyyyyyzz"), b"yyyyyzaa")
    }
    #[test]
    fn test_password_increment_7() {
        assert_eq!(password_increment(b"yzzzzzzz"), b"zaaaaaaa")
    }
    #[test]
    fn test_password_increment_8() {
        assert_eq!(password_increment(b"zzzzzzzy"), b"zzzzzzzz")
    }
    #[test]
    fn test_password_increment_9() {
        assert_eq!(password_increment(b"zzzzzzzz"), b"aaaaaaaa")
    }
    #[test]
    fn test_password_increment_10() {
        assert_eq!(password_increment(b"aaaazaaa"), b"aaaazaab")
    }
    #[test]
    fn test_get_new_password_1() {
        assert_eq!(get_new_password(b"abcdefgh"), b"abcdffaa")
    }
    #[test]
    fn test_get_new_password_2() {
        assert_eq!(get_new_password(b"ghijklmn"), b"ghjaabcc")
    }
}
