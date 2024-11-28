use std::io::BufRead;

fn calc_ribon(input: &[u8]) -> u32 {
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
    println!("{:?}", calc_ribon(include_bytes!("../input.txt")));
}
