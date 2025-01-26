use std::collections::HashMap;
use std::process::exit;

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
#[derive(Debug, Clone)]
enum Type {
    Int(u16),
    Ref(&'static [u8]),
}

#[allow(dead_code)]
#[derive(Debug)]
struct Signal {
    sig: Option<Type>,
    op: Option<Operator>,
    l_op: Option<Type>,
    r_op: Option<Type>,
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
                            sig: Some(Type::Int(int)),
                            op: None,
                            l_op: None,
                            r_op: None,
                        },
                    ))
                } else {
                    Some((
                        x[2].as_bytes(),
                        Signal {
                            sig: Some(Type::Ref(x[0].as_bytes())),
                            op: None,
                            l_op: None,
                            r_op: None,
                        },
                    ))
                }
            } else if x.len() == 4 {
                if let Ok(int) = x[1].parse::<u16>() {
                    Some((
                        x[3].as_bytes(),
                        Signal {
                            sig: None,
                            op: Some(Operator::Not),
                            l_op: None,
                            r_op: Some(Type::Int(int)),
                        },
                    ))
                } else {
                    Some((
                        x[3].as_bytes(),
                        Signal {
                            sig: None,
                            op: Some(Operator::Not),
                            l_op: None,
                            r_op: Some(Type::Ref(x[1].as_bytes())),
                        },
                    ))
                }
            } else if x[1] == "AND" {
                if let Ok(int) = x[0].parse::<u16>() {
                    Some((
                        x[4].as_bytes(),
                        Signal {
                            sig: None,
                            op: Some(Operator::And),
                            l_op: Some(Type::Int(int)),
                            r_op: Some(Type::Ref(x[2].as_bytes())),
                        },
                    ))
                } else {
                    Some((
                        x[4].as_bytes(),
                        Signal {
                            sig: None,
                            op: Some(Operator::And),
                            l_op: Some(Type::Ref(x[0].as_bytes())),
                            r_op: Some(Type::Ref(x[2].as_bytes())),
                        },
                    ))
                }
            } else if x[1] == "OR" {
                if let Ok(int) = x[0].parse::<u16>() {
                    Some((
                        x[4].as_bytes(),
                        Signal {
                            sig: None,
                            op: Some(Operator::Or),
                            l_op: Some(Type::Int(int)),
                            r_op: Some(Type::Ref(x[2].as_bytes())),
                        },
                    ))
                } else {
                    Some((
                        x[4].as_bytes(),
                        Signal {
                            sig: None,
                            op: Some(Operator::Or),
                            l_op: Some(Type::Ref(x[0].as_bytes())),
                            r_op: Some(Type::Ref(x[2].as_bytes())),
                        },
                    ))
                }
            } else if x[1] == "LSHIFT" {
                if let Ok(int) = x[2].parse::<u16>() {
                    Some((
                        x[4].as_bytes(),
                        Signal {
                            sig: None,
                            op: Some(Operator::Lshift),
                            l_op: Some(Type::Ref(x[0].as_bytes())),
                            r_op: Some(Type::Int(int)),
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
                            sig: None,
                            op: Some(Operator::Rshift),
                            l_op: Some(Type::Ref(x[0].as_bytes())),
                            r_op: Some(Type::Int(int)),
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

fn get_signal<'a>(
    wire: &'a [u8],
    data: &HashMap<&[u8], Signal>,
    seen: &mut HashMap<&'a [u8], u16>,
) -> u16 {
    println!(
        "{wire:?}, {:?} -> {:?}",
        String::from_utf8(wire.to_vec()).unwrap(),
        data[wire]
    );
    match data[wire] {
        Signal {
            sig: Some(Type::Int(n)),
            ..
        } => seen.insert(wire, n).unwrap_or(n),
        Signal {
            sig: Some(Type::Ref(r)),
            ..
        } => seen
            .get(r)
            .copied()
            .unwrap_or_else(|| get_signal(r, data, seen)),
        Signal {
            sig: None,
            op: Some(Operator::And),
            l_op: Some(Type::Ref(lr)),
            r_op: Some(Type::Ref(rr)),
        } => {
            seen.get(lr)
                .copied()
                .unwrap_or_else(|| get_signal(lr, data, seen))
                & seen
                    .get(rr)
                    .copied()
                    .unwrap_or_else(|| get_signal(rr, data, seen))
        }
        Signal {
            sig: None,
            op: Some(Operator::And),
            l_op: Some(Type::Int(lr)),
            r_op: Some(Type::Ref(rr)),
        } => lr & seen.get(rr).copied().unwrap_or_else(|| get_signal(rr, data, seen)),
        Signal {
            sig: None,
            op: Some(Operator::Or),
            l_op: Some(Type::Ref(lr)),
            r_op: Some(Type::Ref(rr)),
        } => get_signal(lr, data, seen) | get_signal(rr, data, seen),
        Signal {
            sig: None,
            op: Some(Operator::Not),
            l_op: None,
            r_op: Some(Type::Ref(rr)),
        } => !get_signal(rr, data, seen),
        Signal {
            sig: None,
            op: Some(Operator::Lshift),
            l_op: Some(Type::Ref(lr)),
            r_op: Some(Type::Int(rr)),
        } => get_signal(lr, data, seen) << rr,
        Signal {
            sig: None,
            op: Some(Operator::Rshift),
            l_op: Some(Type::Ref(lr)),
            r_op: Some(Type::Int(rr)),
        } => get_signal(lr, data, seen) << rr,
        _ => {
            println!(
                "Wire: {}: {:?}",
                String::from_utf8(wire.to_vec()).unwrap(),
                data[wire]
            );
            exit(0)
        }
    }
}

fn main() {
    let data = build_data(include_str!("../input.txt"));
    // for item in &data {
    //     println!("{item:?}");
    // }
    println!("{}", get_signal(b"a", &data, &mut HashMap::new()));
}
