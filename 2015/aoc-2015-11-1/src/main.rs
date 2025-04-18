fn get_new_password(input: &[u8]) -> Vec<u8> {
    input.iter().rev()
        .fold(vec![], |mut acc, &x| {
            if acc.iter().all(|&c| c == b'a') {
                if x == b'z' {
                    acc.push(b'a')
                } else {
                    acc.push(x + 1)
                }
            } else {
                acc.push(x)
            }
            acc
        })
        .into_iter().rev().collect::<Vec<u8>>()
}

fn main() {
    // hepxcrrq
    println!(
        "New password: {}",
        String::from_utf8_lossy(get_new_password(b"zyyyyyya").as_slice())
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
        assert_eq!(get_new_password(b"yzzzzzzz"), b"zzzzzzzz")
    }
}
