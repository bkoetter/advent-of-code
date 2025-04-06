use itertools::{Itertools, sorted};
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    let distances_between_cities = get_distances_between_cities(input);
    let cities = get_cities(distances_between_cities.clone());
    for (i, city) in distances_between_cities.keys().enumerate() {
        println!("{}: {} {}", i + 1, city.0, city.1);
    }
    for (i, city) in cities.iter().enumerate() {
        println!("{}: {city}", i + 1)
    }
    for list in cities.iter().permutations(cities.len()).collect::<Vec<_>>() {
        println!("{:?}", list)
    }
    println!(
        "{}",
        cities
            .iter()
            .permutations(cities.len())
            .collect::<Vec<_>>()
            .len()
    )
}

type City = String;
type Distance = u32;
type CityDistances = HashMap<(City, City), Distance>;

fn get_cities(city_distances: CityDistances) -> Vec<City> {
    city_distances
        .into_keys()
        .flat_map(|cities| [cities.0, cities.1])
        .unique()
        .collect()
}

fn get_distances_between_cities(input: &str) -> CityDistances {
    input
        .lines()
        .map(|line| line.parse::<CityDistance>().unwrap())
        .map(|cd| (cd.cities, cd.distance))
        .collect::<CityDistances>()
}

struct CityDistance {
    cities: (City, City),
    distance: Distance,
}

impl FromStr for CityDistance {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        Ok(CityDistance {
            cities: sorted([parts[0].to_string(), parts[2].to_string()])
                .collect_tuple()
                .unwrap(),
            distance: parts[4].parse::<u32>().unwrap(),
        })
    }
}
