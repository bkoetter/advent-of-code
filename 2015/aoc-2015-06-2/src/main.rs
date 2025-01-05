use std::io::BufRead;

fn total_brightness(input: &[u8]) -> usize {
    let fields = |line: String| {
        line.rsplitn(4, " ")
            .filter_map(|x| {
                x.split_once(',')
                    .map(|x| (x.0.parse::<i64>().unwrap(), x.1.parse::<i64>().unwrap()))
            })
            .collect::<Vec<_>>()
    };
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| match line {
            String { .. } if line.starts_with("toggle") => fields(line),
            String { .. } if line.starts_with("turn on") => fields(line),
            String { .. } if line.starts_with("turn off") => fields(line),
            _ => panic!("Impossible line: {line}"),
        })
        .for_each(|x| println!("{x:?}"));
    1
}

fn main() {
    println!("{}", total_brightness(include_bytes!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_brightness_1() {
        assert_eq!(total_brightness(b"turn on 0,0 through 0,0"), 1)
    }
}
