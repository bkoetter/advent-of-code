use std::io::BufRead;

fn total_brightness(input: Vec<u8>) -> i64 {
    let mut matrix = [[0; 1000]; 1000];
    let fields = |line: String| {
        line.rsplitn(4, " ")
            .filter_map(|x| {
                x.split_once(',')
                    .map(|x| (x.0.parse::<usize>().unwrap(), x.1.parse::<usize>().unwrap()))
            })
            .collect::<Vec<(usize, usize)>>()
    };
    let mut adjust_brightnesss = |fields: Vec<(usize, usize)>, dimm: i64| {
        (fields[1].0..=fields[0].0).for_each(|x| {
            (fields[1].1..=fields[0].1).for_each(|y| {
                if !(dimm.is_negative() && matrix[x][y] == 0) {
                    matrix[x][y] += dimm
                }
            })
        })
    };
    input
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| match line {
            String { .. } if line.starts_with("toggle") => adjust_brightnesss(fields(line), 2),
            String { .. } if line.starts_with("turn on") => adjust_brightnesss(fields(line), 1),
            String { .. } if line.starts_with("turn off") => adjust_brightnesss(fields(line), -1),
            _ => eprintln!("Impossible line: {line}"),
        });
    matrix.iter().flatten().sum()
}

fn main() {
    println!(
        "{}",
        total_brightness(Vec::from(include_bytes!("../input.txt")))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_brightness_1() {
        assert_eq!(total_brightness(Vec::from(b"turn on 0,0 through 0,0")), 1)
    }
    #[test]
    fn test_total_brightness_2() {
        assert_eq!(total_brightness(Vec::from(b"turn on 4,4 through 8,8")), 25)
    }
    #[test]
    fn test_total_brightness_3() {
        assert_eq!(
            total_brightness(Vec::from(b"toggle 0,0 through 999,999")),
            2_000_000
        )
    }
}
