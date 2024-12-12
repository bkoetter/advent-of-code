use itertools::Itertools;

fn calc_houses(input: &[u8]) -> usize {
    input
        .iter()
        .fold(
            vec![((0i8, 0i8), (0i8, 0i8))],
            |mut acc: Vec<((i8, i8), (i8, i8))>, x| {
                if acc.len() % 2 == 0 {
                    match x {
                        60 => {
                            acc.append(
                                vec![(
                                    (acc[acc.len() - 1].0.0 + 1, acc[acc.len() - 1].0.1),
                                    (acc[acc.len() - 1].1.0, acc[acc.len() - 1].1.1),
                                )]
                                .as_mut(),
                            );
                            acc
                        }
                        62 => {
                            acc.append(
                                vec![(
                                    (acc[acc.len() - 1].0.0 - 1, acc[acc.len() - 1].0.1),
                                    (acc[acc.len() - 1].1.0, acc[acc.len() - 1].1.1),
                                )]
                                .as_mut(),
                            );
                            acc
                        }
                        94 => {
                            acc.append(
                                vec![(
                                    (acc[acc.len() - 1].0.0, acc[acc.len() - 1].0.1 + 1),
                                    (acc[acc.len() - 1].1.0, acc[acc.len() - 1].1.1),
                                )]
                                .as_mut(),
                            );
                            acc
                        }
                        118 => {
                            acc.append(
                                vec![(
                                    (acc[acc.len() - 1].0.0, acc[acc.len() - 1].0.1 - 1),
                                    (acc[acc.len() - 1].1.0, acc[acc.len() - 1].1.1),
                                )]
                                .as_mut(),
                            );
                            acc
                        }
                        _ => acc,
                    }
                } else {
                    match x {
                        60 => {
                            acc.append(
                                vec![(
                                    (acc[acc.len() - 1].0.0, acc[acc.len() - 1].0.1),
                                    (acc[acc.len() - 1].1.0 + 1, acc[acc.len() - 1].1.1),
                                )]
                                    .as_mut(),
                            );
                            acc
                        }
                        62 => {
                            acc.append(
                                vec![(
                                    (acc[acc.len() - 1].0.0, acc[acc.len() - 1].0.1),
                                    (acc[acc.len() - 1].1.0 - 1, acc[acc.len() - 1].1.1),
                                )]
                                    .as_mut(),
                            );
                            acc
                        }
                        94 => {
                            acc.append(
                                vec![(
                                    (acc[acc.len() - 1].0.0, acc[acc.len() - 1].0.1),
                                    (acc[acc.len() - 1].1.0, acc[acc.len() - 1].1.1 + 1),
                                )]
                                    .as_mut(),
                            );
                            acc
                        }
                        118 => {
                            acc.append(
                                vec![(
                                    (acc[acc.len() - 1].0.0, acc[acc.len() - 1].0.1),
                                    (acc[acc.len() - 1].1.0, acc[acc.len() - 1].1.1 - 1),
                                )]
                                    .as_mut(),
                            );
                            acc
                        }
                        _ => acc,
                    }
                }
            },
        )
        .iter()
        .fold(
            vec![(0i8, 0i8)], |mut acc, x| {
                acc.append(&mut vec![x.0]);
                acc.append(&mut vec![x.1]);
                acc
            })
        .into_iter()
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
        assert_eq!(1, calc_houses(b"1"))
    }
}
