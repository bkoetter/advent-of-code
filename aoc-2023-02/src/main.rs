use std::cmp::max;

#[derive(Debug, PartialEq)]
struct Draw {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u16,
    draw: Draw,
}

const MAX: Draw = Draw {
    red: 12,
    blue: 14,
    green: 13,
};

fn parse_game(game: &str) -> Option<Game> {
    game.split_once(':')
        .map(|(id, draws)| Game {
            id: id.trim().split_once(' ').unwrap().1.parse::<u16>().unwrap(),
            draw: draws.split(';').map(|draw| draw.split(',')
                .map(|s| s.trim().split_once(' ')
                    .map(|(n, color)|
                        match color {
                            "red" => Draw { red: n.parse::<u32>().unwrap(), blue: 0, green: 0 },
                            "blue" => Draw { red: 0, blue: n.parse::<u32>().unwrap(), green: 0 },
                            "green" => Draw { red: 0, blue: 0, green: n.parse::<u32>().unwrap() },
                            _ => panic!("Unknown color: {color}"),
                        }).unwrap())
                .fold(Draw { red: 0, blue: 0, green: 0 }, |acc, draw| Draw {
                    red: acc.red + draw.red,
                    blue: acc.blue + draw.blue,
                    green: acc.green + draw.green,
                }))
                .fold(Draw { red: 0, blue: 0, green: 0 }, |acc, draw| Draw {
                    red: max(acc.red, draw.red),
                    blue: max(acc.blue, draw.blue),
                    green: max(acc.green, draw.green),
                }),
        })
}

fn main() -> std::io::Result<()> {
    println!("{:?}", std::fs::read_to_string("input.txt")?
        .lines().filter_map(parse_game)
        .map(|game| (
            match game.draw.red <= MAX.red && game.draw.blue <= MAX.blue && game.draw.green <= MAX.green {
                true => game.id,
                false => 0,
            },
            game.draw.red * game.draw.blue * game.draw.green,
        ))
        .fold((0, 0), |acc, game| (acc.0 + game.0, acc.1 + game.1))
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        assert_eq!(parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), Some(Game { id: 1, draw: Draw { red: 4, blue: 6, green: 2 } }));
        assert_eq!(parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), Some(Game { id: 2, draw: Draw { red: 1, blue: 4, green: 3 } }));
        assert_eq!(parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), Some(Game { id: 3, draw: Draw { red: 20, blue: 6, green: 13 } }));
        assert_eq!(parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), Some(Game { id: 4, draw: Draw { red: 14, blue: 15, green: 3 } }));
        assert_eq!(parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), Some(Game { id: 5, draw: Draw { red: 6, blue: 2, green: 3 } }));
    }
}
