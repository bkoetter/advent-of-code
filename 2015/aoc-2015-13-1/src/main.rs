use itertools::Itertools;
use std::num::ParseIntError;
use std::ops::Neg;
use std::str::FromStr;

type Name = String;
type Happiness = i32;
type Data = std::collections::HashMap<(Name, Name), Happiness>;

#[derive(Debug, PartialEq)]
struct Record {
    persons: (Name, Name),
    relation: Happiness,
}

impl FromStr for Record {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s
            .trim_end_matches('.')
            .split_ascii_whitespace()
            .collect::<Vec<_>>();
        Ok(Record {
            persons: (x[0].min(x[10]).to_string(), x[0].max(x[10]).to_string()),
            relation: if x[2] == "lose" {
                x[3].parse::<Happiness>()?.neg()
            } else {
                x[3].parse::<Happiness>()?
            },
        })
    }
}

fn get_data(input: &str) -> Data {
    input
        .lines()
        .map(|line| line.parse::<Record>().unwrap())
        .fold(Data::new(), |mut acc, record| {
            acc.entry(record.persons)
                .and_modify(|relation| *relation += record.relation)
                .or_insert(record.relation);
            acc
        })
}

fn get_names(data: &Data) -> Vec<Name> {
    data.keys()
        .flat_map(|persons| [persons.0.to_owned(), persons.1.to_owned()])
        .unique()
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let data = get_data(input);
    let names = get_names(&data);
    println!(
        "Max happiness: {:?}",
        names
            .iter()
            .permutations(names.len())
            .map(|mut persons| {
                persons.push(persons[0]);
                persons
            })
            .map(|persons| persons
                .windows(2)
                .map(|persons| persons.iter().sorted().collect_tuple().unwrap())
                .map(|persons: (&&Name, &&Name)| {
                    data.get(&(persons.0.to_string(), persons.1.to_string()))
                        .unwrap()
                })
                .sum::<Happiness>())
            .max()
            .unwrap()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_data_1() {
        assert_eq!(
            "Alice would gain 54 happiness units by sitting next to Bob.".parse(),
            Ok(Record {
                persons: ("Alice".to_string(), "Bob".to_string()),
                relation: 54
            })
        )
    }
    #[test]
    fn test_get_data_2() {
        assert!(
            "Alice would gain five happiness units by sitting next to Bob."
                .parse::<Record>()
                .is_err(),
        )
    }
}
