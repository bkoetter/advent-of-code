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
    password_increment(input)
}

fn main() {
    println!(
        "New password: {}",
        String::from_utf8_lossy(&get_new_password(b"hepxcrrq"))
    );
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
}
