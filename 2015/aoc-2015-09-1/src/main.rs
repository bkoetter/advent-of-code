use itertools::{Itertools, sorted};
use std::collections::HashMap;
use std::str::FromStr;

type City = String;
type Distance = u32;
type CityDistances = HashMap<(City, City), Distance>;

fn main() {
    let input = include_str!("../input.txt");
    let distances_between_cities = get_distances_between_cities(input);
    let cities = get_cities(&distances_between_cities);
    println!(
        "Min: {}",
        cities
            .iter()
            .permutations(cities.len())
            .map(|list| calculate_distance(list, &distances_between_cities))
            .min()
            .unwrap()
    );
    println!(
        "Max: {}",
        cities
            .iter()
            .permutations(cities.len())
            .map(|list| calculate_distance(list, &distances_between_cities))
            .max()
            .unwrap()
    )
}

fn get_cities(city_distances: &CityDistances) -> Vec<City> {
    city_distances
        .keys()
        .flat_map(|cities| [cities.0.to_string(), cities.1.to_string()])
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

fn calculate_distance(cities: Vec<&City>, distances_between_cities: &CityDistances) -> Distance {
    cities
        .windows(2)
        .map(|cities| sorted(cities).collect_tuple::<(&&City, &&City)>().unwrap())
        .map(|cities| {
            distances_between_cities
                .get(&(cities.0.to_string(), cities.1.to_string()))
                .unwrap()
        })
        .sum()
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
            cities: sorted([parts[0].to_owned(), parts[2].to_owned()])
                .collect_tuple()
                .unwrap(),
            distance: parts[4].parse::<u32>().unwrap(),
        })
    }
}
