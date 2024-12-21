use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn calc_floor(input: &[u8]) -> Result<i32, i32> {
    let calc = |acc: Result<i32, i32>, operand, idx| {
        if (acc.unwrap_err() + operand) != -1 {
            Continue(Err(acc.unwrap_err() + operand))
        } else {
            Done(Ok(idx as i32 + 1))
        }
    };
    input
        .iter()
        .enumerate()
        .fold_while(Err(0), |acc, b| match b.1 {
            b'(' => calc(acc, 1, b.0),
            b')' => calc(acc, -1, b.0),
            _ => Continue(acc),
        })
        .into_inner()
}

fn main() {
    match calc_floor(include_bytes!("../input.txt")) {
        Ok(floor) => println!("Reached basement at character position {floor}"),
        Err(_) => println!("Santa never reached the basement"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn try_floor_0() {
        calc_floor(b"(((((").unwrap();
    }
    #[test]
    fn try_floor_1() {
        assert_eq!(Ok(1), calc_floor(b")"));
    }
    #[test]
    fn try_floor_5() {
        assert_eq!(Ok(5), calc_floor(b"()()))"));
    }
}
