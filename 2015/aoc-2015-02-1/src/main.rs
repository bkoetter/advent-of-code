use std::io::BufRead;

fn calc_size(input: &[u8]) -> u16 {
    input
        .lines()
        .map(|s| s.unwrap().splitn(3,'x')
            .map(|n| n.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
        )
        .map(|array| array.iter().min().unwrap() + array.iter().sum::<u16>())
        .sum()
}

fn main() {
    println!("{}", calc_size(include_bytes!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_size() {
        assert_eq!(58, calc_size(b"6x12x8"));
    }
}
