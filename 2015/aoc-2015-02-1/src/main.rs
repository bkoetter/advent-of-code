use std::io::BufRead;

#[derive(Debug)]
struct Square {
    length: u32,
    width: u32,
    height: u32,
}

fn calc_size(input: &[u8]) -> u32 {
    input
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.splitn(3, 'x').fold(
                Square {
                    length: 0,
                    width: 0,
                    height: 0,
                },
                |acc, field| match acc {
                    Square {
                        length: 0,
                        width: 0,
                        height: 0,
                    } => Square {
                        length: field.parse().unwrap(),
                        width: acc.width,
                        height: acc.height,
                    },
                    Square {
                        length: _,
                        width: 0,
                        height: 0,
                    } => Square {
                        length: acc.length,
                        width: field.parse().unwrap(),
                        height: acc.height,
                    },
                    Square {
                        length: _,
                        width: _,
                        height: 0,
                    } => Square {
                        length: acc.length,
                        width: acc.width,
                        height: field.parse().unwrap(),
                    },
                    _ => panic!("This must never happen"),
                },
            )
        })
        .fold(0, |acc, square| {
            acc + (square.length * square.width
                + square.width * square.height
                + square.height * square.length)
                * 2
                + [
                    square.length * square.width,
                    square.width * square.height,
                    square.height * square.length,
                ]
                .iter()
                .min()
                .unwrap()
        })
}

fn main() {
    println!("{}", calc_size(include_bytes!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_size_1() {
        assert_eq!(58, calc_size(b"2x3x4"));
    }
    #[test]
    fn try_size_2() {
        assert_eq!(43, calc_size(b"1x1x10"));
    }
}
