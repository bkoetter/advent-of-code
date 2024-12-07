use itertools::Itertools;

fn calc_houses(input: &[u8]) -> usize {
    input
        .iter()
        .fold(vec![(0, 0)], |mut acc: Vec<(i16, i16)>, x| match x {
            60 => {
                acc.append(vec![(acc[acc.len() - 1].0 + 1, acc[acc.len() - 1].1)].as_mut());
                acc
            }
            62 => {
                acc.append(vec![(acc[acc.len() - 1].0 - 1, acc[acc.len() - 1].1)].as_mut());
                acc
            }
            94 => {
                acc.append(vec![(acc[acc.len() - 1].0, acc[acc.len() - 1].1 - 1)].as_mut());
                acc
            }
            118 => {
                acc.append(vec![(acc[acc.len() - 1].0, acc[acc.len() - 1].1 + 1)].as_mut());
                acc
            }
            _ => acc,
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
    fn try_foo_1() {
        assert_eq!(2, calc_houses(b">"))
    }
    #[test]
    fn try_foo_2() {
        assert_eq!(4, calc_houses(b"^>v<"))
    }
    #[test]
    fn try_foo_3() {
        assert_eq!(2, calc_houses(b"^v^v^v^v^v"))
    }
}
