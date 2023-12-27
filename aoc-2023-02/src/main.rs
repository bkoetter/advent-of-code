// input file contains:
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
// compare with:
// Max: 12 red, 13 green, 14 blue
#[derive(Debug)]
struct Draw {
    red: u8,
    blue: u8,
    green: u8,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u16,
    draws: bool,
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
            draws: draws.split(';').map(|draw| draw.split(',')
                .map(|s| s.trim().split_once(' ')
                    .map(|(n, color)|
                        match color {
                            "red" => Draw { red: n.parse::<u8>().unwrap(), blue: 0, green: 0 },
                            "blue" => Draw { red: 0, blue: n.parse::<u8>().unwrap(), green: 0 },
                            "green" => Draw { red: 0, blue: 0, green: n.parse::<u8>().unwrap() },
                            _ => panic!("Unknown color: {color}"),
                        }).unwrap())
                .fold(Draw { red: 0, blue: 0, green: 0 }, |acc, draw| Draw {
                    red: acc.red + draw.red,
                    blue: acc.blue + draw.blue,
                    green: acc.green + draw.green,
                })
            )
                .all(|draw| draw.red <= MAX.red && draw.blue <= MAX.blue && draw.green <= MAX.green),
        }).filter(|game| game.draws)
}

fn main() -> std::io::Result<()> {
    println!("{}", std::fs::read_to_string("input.txt")?.lines().filter_map(parse_game).map(|game| game.id).sum::<u16>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        assert_eq!(parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), Some(Game { id: 2, draws: true }));
        assert_eq!(parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), None);
        assert_eq!(parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), None);
        assert_eq!(parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), Some(Game { id: 5, draws: true }));
        assert_eq!(parse_game("Game 1: 1 blue, 7 red, 8 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 13 blue, 8 red, 3 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 5 blue, 7 red, 14 green"), None);
        assert_eq!(parse_game("Game 1: 18 blue, 11 red, 7 green"), None);
        assert_eq!(parse_game("Game 1: 4 blue, 15 red, 17 green"), None);
        assert_eq!(parse_game("Game 1: 4 blue, 7 red, 9 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 3 blue, 11 red, 6 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 11 blue, 12 red, 11 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 20 blue, 7 red, 5 green"), None);
        assert_eq!(parse_game("Game 1: 8 blue, 5 red, 16 green"), None);
        assert_eq!(parse_game("Game 1: 6 blue, 8 red, 4 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 16 blue, 18 red, 10 green"), None);
        assert_eq!(parse_game("Game 1: 3 blue, 8 red, 5 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 10 blue, 15 red, 5 green"), None);
        assert_eq!(parse_game("Game 1: 9 blue, 17 red, 1 green"), None);
        assert_eq!(parse_game("Game 1: 13 blue, 16 red, 1 green"), None);
        assert_eq!(parse_game("Game 1: 2 blue, 2 red, 9 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 16 blue, 12 red, 11 green"), None);
        assert_eq!(parse_game("Game 1: 2 blue, 9 red, 12 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 15 blue, 15 red, 3 green"), None);
        assert_eq!(parse_game("Game 1: 8 blue, 4 red, 7 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 3 blue, 11 red, 12 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 12 blue, 8 red, 14 green"), None);
        assert_eq!(parse_game("Game 1: 8 blue, 6 red, 15 green"), None);
        assert_eq!(parse_game("Game 1: 7 blue, 5 red, 16 green"), None);
        assert_eq!(parse_game("Game 1: 14 blue, 11 red, 11 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 8 blue, 2 red, 5 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 10 blue, 15 red, 18 green"), None);
        assert_eq!(parse_game("Game 1: 17 blue, 8 red, 12 green"), None);
        assert_eq!(parse_game("Game 1: 18 blue, 19 red, 5 green"), None);
        assert_eq!(parse_game("Game 1: 12 blue, 13 red, 4 green"), None);
        assert_eq!(parse_game("Game 1: 7 blue, 17 red, 15 green"), None);
        assert_eq!(parse_game("Game 1: 14 blue, 1 red, 3 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 13 blue, 2 red, 15 green"), None);
        assert_eq!(parse_game("Game 1: 5 blue, 12 red, 1 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 7 blue, 10 red, 9 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 14 blue, 19 red, 2 green"), None);
        assert_eq!(parse_game("Game 1: 5 blue, 6 red, 15 green"), None);
        assert_eq!(parse_game("Game 1: 15 blue, 5 red, 5 green"), None);
        assert_eq!(parse_game("Game 1: 10 blue, 17 red, 9 green"), None);
        assert_eq!(parse_game("Game 1: 1 blue, 11 red, 5 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 18 blue, 12 red, 16 green"), None);
        assert_eq!(parse_game("Game 1: 3 blue, 10 red, 5 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 19 blue, 8 red, 4 green"), None);
        assert_eq!(parse_game("Game 1: 5 blue, 1 red, 8 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 8 blue, 6 red, 13 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 5 blue, 15 red, 6 green"), None);
        assert_eq!(parse_game("Game 1: 16 blue, 16 red, 1 green"), None);
        assert_eq!(parse_game("Game 1: 20 blue, 8 red, 12 green"), None);
        assert_eq!(parse_game("Game 1: 9 blue, 9 red, 9 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 2 blue, 6 red, 8 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 5 blue, 2 red, 12 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 10 blue, 8 red, 12 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 4 blue, 7 red, 5 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 13 blue, 7 red, 1 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 8 blue, 6 red, 6 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 20 blue, 8 red, 16 green"), None);
        assert_eq!(parse_game("Game 1: 5 blue, 12 red, 4 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 11 blue, 9 red, 17 green"), None);
        assert_eq!(parse_game("Game 1: 18 blue, 5 red, 15 green"), None);
        assert_eq!(parse_game("Game 1: 19 blue, 2 red, 12 green"), None);
        assert_eq!(parse_game("Game 1: 5 blue, 17 red, 17 green"), None);
        assert_eq!(parse_game("Game 1: 15 blue, 5 red, 10 green"), None);
        assert_eq!(parse_game("Game 1: 10 blue, 11 red, 5 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 3 blue, 11 red, 9 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 7 blue, 3 red, 13 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 5 blue, 14 red, 10 green"), None);
        assert_eq!(parse_game("Game 1: 6 blue, 3 red, 12 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 3 blue, 10 red, 6 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 9 blue, 1 red, 6 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 12 blue, 3 red, 1 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 19 blue, 1 red, 3 green"), None);
        assert_eq!(parse_game("Game 1: 12 blue, 10 red, 14 green"), None);
        assert_eq!(parse_game("Game 1: 17 blue, 15 red, 2 green"), None);
        assert_eq!(parse_game("Game 1: 2 blue, 14 red, 4 green"), None);
        assert_eq!(parse_game("Game 1: 7 blue, 9 red, 1 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 16 blue, 5 red, 11 green"), None);
        assert_eq!(parse_game("Game 1: 7 blue, 5 red, 1 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 8 blue, 2 red, 3 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 8 blue, 13 red, 8 green"), None);
        assert_eq!(parse_game("Game 1: 9 blue, 9 red, 11 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 11 blue, 9 red, 17 green"), None);
        assert_eq!(parse_game("Game 1: 8 blue, 8 red, 3 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 9 blue, 1 red, 2 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 17 blue, 3 red, 11 green"), None);
        assert_eq!(parse_game("Game 1: 2 blue, 15 red, 6 green"), None);
        assert_eq!(parse_game("Game 1: 17 blue, 3 red, 4 green"), None);
        assert_eq!(parse_game("Game 1: 14 blue, 10 red, 16 green"), None);
        assert_eq!(parse_game("Game 1: 17 blue, 14 red, 4 green"), None);
        assert_eq!(parse_game("Game 1: 10 blue, 10 red, 6 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 5 blue, 12 red, 18 green"), None);
        assert_eq!(parse_game("Game 1: 11 blue, 12 red, 10 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 8 blue, 1 red, 8 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 9 blue, 16 red, 2 green"), None);
        assert_eq!(parse_game("Game 1: 6 blue, 8 red, 11 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 9 blue, 11 red, 12 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 6 blue, 13 red, 9 green"), None);
        assert_eq!(parse_game("Game 1: 2 blue, 10 red, 12 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 13 blue, 6 red, 10 green"), Some(Game { id: 1, draws: true }));
        assert_eq!(parse_game("Game 1: 1 blue, 14 red, 17 green"), None);
    }
}
