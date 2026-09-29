fn step(b: u8) -> i32 {
    match b {
        b'(' => 1,
        b')' => -1,
        _ => 0,
    }
}

fn calc_floor(input: &[u8]) -> i32 {
    input.iter().copied().map(step).sum()
}

fn main() {
    println!("{}", calc_floor(include_bytes!("../input.txt")));
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
        assert_eq!(-1, calc_floor(b"))("));
        assert_eq!(-3, calc_floor(b")))"));
        assert_eq!(-3, calc_floor(b")())())"));
    }
    #[test]
    fn ignores_trailing_newline() {
        assert_eq!(3, calc_floor(b"(((\n"));
    }
}
