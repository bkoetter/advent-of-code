use std::io::BufRead;

#[derive(Debug)]
struct Square {
    l: u32,
    w: u32,
    h: u32,
}

fn calc_size(input: &[u8]) -> u32 {
    input
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.splitn(3, 'x')
                .fold(Square { l: 0, w: 0, h: 0 }, |acc, field| match acc {
                    Square { l: 0, w: 0, h: 0 } => Square {
                        l: field.parse().unwrap(),
                        w: acc.w,
                        h: acc.h,
                    },
                    Square { l: _, w: 0, h: 0 } => Square {
                        l: acc.l,
                        w: field.parse().unwrap(),
                        h: acc.h,
                    },
                    Square { l: _, w: _, h: 0 } => Square {
                        l: acc.l,
                        w: acc.w,
                        h: field.parse().unwrap(),
                    },
                    _ => panic!("This must never happen"),
                })
        })
        .fold(0, |acc, square| {
            acc + (square.l * square.w + square.w * square.h + square.h * square.l) * 2
            + [square.l * square.w, square.w * square.h, square.h * square.l].iter().min().unwrap()
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
