fn get_md5(input: &str) -> Option<(String, u64)> {
    (1..=u64::MAX)
        .map(|i| {
            let digest = md5::compute(format!("{}{}", input, i));
            (format!("{:x}", digest), i)
        })
        .find(|s| s.0.starts_with("000000"))
}

fn main() {
    match get_md5("bgvyzdsv") {
        None => println!("No matching number found."),
        Some((.., magic_number)) => println!("{magic_number}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_md5() {
        assert_eq!(get_md5("abcdef").unwrap().1, 6742839);
        assert_eq!(get_md5("pqrstuv").unwrap().1, 5714438);
    }
}
