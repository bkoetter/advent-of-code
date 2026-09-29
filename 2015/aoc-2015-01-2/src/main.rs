fn step(b: u8) -> i32 {
    match b {
        b'(' => 1,
        b')' => -1,
        _ => 0,
    }
}

/// 1-based position of the first character that takes Santa to floor -1.
fn basement_position(input: &[u8]) -> Option<usize> {
    input
        .iter()
        .copied()
        .map(step)
        .scan(0, |floor, delta| {
            *floor += delta;
            Some(*floor)
        })
        .position(|floor| floor == -1)
        .map(|idx| idx + 1)
}

fn main() {
    match basement_position(include_bytes!("../input.txt")) {
        Some(pos) => println!("Reached basement at character position {pos}"),
        None => println!("Santa never reached the basement"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn never_reaches_basement() {
        assert_eq!(None, basement_position(b"((((("));
        assert_eq!(None, basement_position(b""));
    }
    #[test]
    fn try_floor_1() {
        assert_eq!(Some(1), basement_position(b")"));
    }
    #[test]
    fn try_floor_5() {
        assert_eq!(Some(5), basement_position(b"()())"));
        assert_eq!(Some(5), basement_position(b"()()))"));
    }
}
