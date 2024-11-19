use std::{fs, io};

fn calc_floor(input: &[u8]) -> i64 {
    input.iter().fold(0, |acc, b| match b {
        40 => acc + 1,
        41 => acc - 1,
        _ => acc,
    })
}

fn main() -> io::Result<()> {
    Ok(println!("{}", calc_floor(&fs::read("input.txt")?)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_floor() {
        assert_eq!(0, calc_floor(b"(())"));
        assert_eq!(0, calc_floor(b"()()"));
        assert_eq!(3, calc_floor(b"((("));
        assert_eq!(3, calc_floor(b"(()(()("));
        assert_eq!(3, calc_floor(b"))((((("));
        assert_eq!(-1, calc_floor(b"())"));
        assert_eq!(-3, calc_floor(b")))"));
        assert_eq!(-3, calc_floor(b")())())"));
    }
}
