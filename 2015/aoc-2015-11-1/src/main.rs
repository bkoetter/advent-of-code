fn get_new_password(input: &[u8]) -> Vec<u8> {
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
    fn test_get_new_password_1() {
        assert_eq!(get_new_password(b"axx"), b"axy")
    }
    #[test]
    fn test_get_new_password_2() {
        assert_eq!(get_new_password(b"axy"), b"axz")
    }
    #[test]
    fn test_get_new_password_3() {
        assert_eq!(get_new_password(b"axz"), b"aya")
    }
    #[test]
    fn test_get_new_password_5() {
        assert_eq!(get_new_password(b"aaaaaaaa"), b"aaaaaaab")
    }
    #[test]
    fn test_get_new_password_6() {
        assert_eq!(get_new_password(b"yyyyyyzz"), b"yyyyyzaa")
    }
    #[test]
    fn test_get_new_password_7() {
        assert_eq!(get_new_password(b"yzzzzzzz"), b"zaaaaaaa")
    }
    #[test]
    fn test_get_new_password_8() {
        assert_eq!(get_new_password(b"zzzzzzzy"), b"zzzzzzzz")
    }
    #[test]
    fn test_get_new_password_9() {
        assert_eq!(get_new_password(b"zzzzzzzz"), b"aaaaaaaa")
    }
    #[test]
    fn test_get_new_password_10() {
        assert_eq!(get_new_password(b"aaaazaaa"), b"aaaazaab")
    }
}
