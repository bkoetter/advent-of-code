use std::str::FromStr;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let cities = get_cities(input);
    // let list = ["Dublin", "London", "Belfast"].into_iter().permutations(3).collect::<Vec<_>>();
    println!("{:?}", cities.iter().permutations(5).collect::<Vec<_>>());
}

// struct City String;
type City = String;
type 

fn get_cities(distances: Vec<Distance>) -> Vec<City> {
    // input.lines().map(|l| l.parse().unwrap()).collect::<Vec<Distance>>()
    distances.iter().unique().collect::<Vec<_>>()
}

#[derive(Debug)]
struct Distance

impl FromStr for Distance {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        Ok(Distance {
            city1: parts[0].to_string(),
            city2: parts[2].to_string(),
            distance: parts[4].parse::<u32>().unwrap(),
        })
    }
}
