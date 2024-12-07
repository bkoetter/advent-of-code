use std::io::BufRead;

fn calc_ribbon(input: &[u8]) -> u32 {
    input
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.splitn(3, "x")
                .filter_map(|field| field.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .map(|mut list| {
            list.sort();
            (list[0] + list[1]) * 2 + list.iter().product::<u32>()
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn main() {
    println!("{:?}", calc_ribbon(include_bytes!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_dim_1() {
        assert_eq!(34, calc_ribbon(b"2x3x4"))
    }
    #[test]
    fn try_dim_2() {
        assert_eq!(14, calc_ribbon(b"1x1x10"))
    }
}