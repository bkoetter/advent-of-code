use itertools::Itertools;

fn calc_houses(input: &[u8]) -> usize {
    let direction = |position: (i8, i8), direction: u8| {
        match direction {
            b'<' => (position.0 + 1, position.1),
            b'>' => (position.0 - 1, position.1),
            b'^' => (position.0, position.1 + 1),
            b'v' => (position.0, position.1 - 1),
            _ => position,
        }
    };
    input
        .iter()
        .fold(
            vec![((0i8, 0i8), (0i8, 0i8))],
            |mut acc: Vec<((i8, i8), (i8, i8))>, &x| {
                if acc.len() % 2 == 0 {
                    acc.push((
                        direction(acc[acc.len() - 1].0, x),
                        acc.last().unwrap().1,
                    ));
                    acc
                } else {
                    acc.push((
                        acc.last().unwrap().0,
                        direction(acc[acc.len() - 1].1, x),
                    ));
                    acc
                }
            },
        )
        .iter()
        .flat_map(|(santa, robo)| vec![santa, robo])
        .unique()
        .collect::<Vec<_>>()
        .len()
}

fn main() {
    println!("{}", calc_houses(include_bytes!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_houses_1() {
        assert_eq!(3, calc_houses(b"^v"))
    }
    #[test]
    fn try_houses_2() {
        assert_eq!(3, calc_houses(b"^>v<"))
    }
    #[test]
    fn try_houses_3() {
        assert_eq!(11, calc_houses(b"^v^v^v^v^v"))
    }
}
