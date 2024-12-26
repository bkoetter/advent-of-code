use std::io::BufRead;

struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new((a, b): (&str, &str)) -> Self {
        Self {
            x: a.parse::<u16>().unwrap(),
            y: b.parse::<u16>().unwrap(),
        }
    }
}

fn get_lights(input: &[u8]) -> usize {
    input
        .lines()
        .map(|line| line.unwrap())
        .fold([[false; 1_000]; 1_000], |mut acc, line| {
            let fields = line.rsplitn(4, ' ').collect::<Vec<&str>>();
            let p1 = Point::new(fields[2].split_once(',').unwrap());
            let p2 = Point::new(fields[0].split_once(',').unwrap());
            if line.starts_with("turn on") {
                for y in p1.y..=p2.y {
                    for x in p1.x..=p2.x {
                        acc[x as usize][y as usize] = true;
                    }
                }
                acc
            } else if line.starts_with("turn off") {
                for y in p1.y..=p2.y {
                    for x in p1.x..=p2.x {
                        acc[x as usize][y as usize] = false;
                    }
                }
                acc
            } else if line.starts_with("toggle") {
                for y in p1.y..=p2.y {
                    for x in p1.x..=p2.x {
                        acc[x as usize][y as usize] = !acc[x as usize][y as usize];
                    }
                }
                acc
            } else {
                acc
            }
        })
        .iter()
        .flatten()
        .filter(|&i| *i)
        .count()
}

fn main() {
    println!("{:?}", get_lights(include_bytes!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lights_1() {
        assert_eq!(get_lights(b"turn on 0,0 through 999,999"), 1_000_000)
    }
    #[test]
    fn test_get_lights_2() {
        assert_eq!(get_lights(b"turn off 499,499 through 500,500"), 0)
    }
}
