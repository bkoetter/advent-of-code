use std::io::BufRead;

fn calc_size(input: &[u8]) -> u64 {
    _ = input
        .lines()
        .map(|s| s.unwrap().splitn(3,'x')
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
        ).for_each(|s| println!("{:?}", s));
    5
}

fn main() {
    calc_size(include_bytes!("../input.txt"));
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_size() {
        assert_eq!(58, calc_size(b"6x12x8"));
    }
}
