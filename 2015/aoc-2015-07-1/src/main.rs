use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
enum Operator {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Type {
    Int(u16),
    Ref(&'static [u8]),
}

#[allow(dead_code)]
#[derive(Debug)]
struct Signal {
    signal: Option<Type>,
    operator: Option<Operator>,
    left_operand: Option<Type>,
    right_operand: Option<Type>,
}

fn build_data(input: &'static str) -> HashMap<&'static [u8], Signal> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter_map(|x| {
            if x.len() == 3 {
                if let Ok(int) = x[0].parse::<u16>() {
                    Some((
                        x[2].as_bytes(),
                        Signal {
                            signal: Some(Type::Int(int)),
                            operator: None,
                            left_operand: None,
                            right_operand: None,
                        },
                    ))
                } else {
                    Some((
                        x[2].as_bytes(),
                        Signal {
                            signal: Some(Type::Ref(x[0].as_bytes())),
                            operator: None,
                            left_operand: None,
                            right_operand: None,
                        },
                    ))
                }
            } else if x.len() == 4 {
                if let Ok(int) = x[1].parse::<u16>() {
                    Some((
                        x[3].as_bytes(),
                        Signal {
                            signal: None,
                            operator: Some(Operator::Not),
                            left_operand: Some(Type::Int(int)),
                            right_operand: None,
                        },
                    ))
                } else {
                    Some((
                        x[3].as_bytes(),
                        Signal {
                            signal: None,
                            operator: Some(Operator::Not),
                            left_operand: Some(Type::Ref(x[1].as_bytes())),
                            right_operand: None,
                        },
                    ))
                }
            } else if x[1] == "AND" {
                if let Ok(int) = x[0].parse::<u16>() {
                    Some((
                        x[4].as_bytes(),
                        Signal {
                            signal: None,
                            operator: Some(Operator::And),
                            left_operand: Some(Type::Int(int)),
                            right_operand: Some(Type::Ref(x[2].as_bytes())),
                        },
                    ))
                } else {
                    Some((
                        x[4].as_bytes(),
                        Signal {
                            signal: None,
                            operator: Some(Operator::And),
                            left_operand: Some(Type::Ref(x[0].as_bytes())),
                            right_operand: Some(Type::Ref(x[2].as_bytes())),
                        },
                    ))
                }
            } else if x[1] == "OR" {
                if let Ok(int) = x[0].parse::<u16>() {
                    Some((
                        x[4].as_bytes(),
                        Signal {
                            signal: None,
                            operator: Some(Operator::Or),
                            left_operand: Some(Type::Int(int)),
                            right_operand: Some(Type::Ref(x[2].as_bytes())),
                        },
                    ))
                } else {
                    Some((
                        x[4].as_bytes(),
                        Signal {
                            signal: None,
                            operator: Some(Operator::Or),
                            left_operand: Some(Type::Ref(x[0].as_bytes())),
                            right_operand: Some(Type::Ref(x[2].as_bytes())),
                        },
                    ))
                }
            } else if x[1] == "LSHIFT" {
                if let Ok(int) = x[2].parse::<u16>() {
                    Some((
                        x[4].as_bytes(),
                        Signal {
                            signal: None,
                            operator: Some(Operator::Lshift),
                            left_operand: Some(Type::Int(int)),
                            right_operand: Some(Type::Ref(x[2].as_bytes())),
                        },
                    ))
                } else {
                    println!("{:?}", x);
                    None
                }
            } else if x[1] == "RSHIFT" {
                if let Ok(int) = x[2].parse::<u16>() {
                    Some((
                        x[4].as_bytes(),
                        Signal {
                            signal: None,
                            operator: Some(Operator::Rshift),
                            left_operand: Some(Type::Int(int)),
                            right_operand: Some(Type::Ref(x[2].as_bytes())),
                        },
                    ))
                } else {
                    println!("{:?}", x);
                    None
                }
            } else {
                println!("{:?}", x);
                None
            }
        })
        .collect()
}

fn main() {
    let data = build_data(include_str!("../input.txt"));
    // for item in &data {
    //     println!("{item:?}");
    // }
    println!("{}", data.len())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_get_bitwise() {
//         assert_eq!(
//             calculate(
//                 "123 -> x
// 456 -> y
// x AND y -> d
// x OR y -> e
// x LSHIFT 2 -> f
// y RSHIFT 2 -> g
// NOT x -> h
// NOT y -> i"
//             ),
//             HashMap::from([
//                 ("h", 65412),
//                 ("g", 114),
//                 ("d", 72),
//                 ("y", 456),
//                 ("f", 492),
//                 ("e", 507),
//                 ("x", 123),
//                 ("i", 65079)
//             ])
//         )
//     }
// }
