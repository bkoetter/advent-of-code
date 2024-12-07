fn calc_floor(input: &[u8]) -> i64 {
    input
        .iter()
        .map(|x| match x {
            40 => 1,
            41 => -1,
            _ => 0,
        })
        .sum::<i64>()
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
        assert_eq!(-3, calc_floor(b")))"));
        assert_eq!(-3, calc_floor(b")())())"));
    }
}
