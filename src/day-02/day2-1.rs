use std::str::FromStr;

#[derive(Debug)]
struct Showing {
    green_cubes: u8,
    red_cubes: u8,
    blue_cubes: u8,
}

impl Showing {
    fn new() -> Self {
        Showing {
            green_cubes: 0,
            red_cubes: 0,
            blue_cubes: 0,
        }
    }

    fn is_valid(&self) -> bool {
        self.green_cubes <= 13 && self.red_cubes <= 12 && self.blue_cubes <= 14
    }
}

#[derive(Debug)]
enum GameError {
    InvalidGameStr,
}

#[derive(Debug)]
struct Game {
    id: u8,
    showings: Vec<Showing>,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.showings.iter().all(|showing| showing.is_valid())
    }
}

impl FromStr for Game {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_identifier, showings) = s.split_once(":").ok_or(GameError::InvalidGameStr)?;
        let id = game_identifier
            .split_once(" ")
            .ok_or(GameError::InvalidGameStr)?
            .1
            .parse::<u8>()
            .map_err(|_| GameError::InvalidGameStr)?;

        let showings = showings
            .split(";")
            .into_iter()
            .map(|showing| {
                let showing = showing.trim();
                let mut s = Showing::new();
                for cube in showing.split(",") {
                    let (amt, color) = cube.trim().split_once(" ").unwrap();
                    match color {
                        "blue" => s.blue_cubes = amt.parse::<u8>().unwrap(),
                        "red" => s.red_cubes = amt.parse::<u8>().unwrap(),
                        "green" => s.green_cubes = amt.parse::<u8>().unwrap(),
                        _ => panic!("unknown color: {color}"),
                    }
                }
                s
            })
            .collect::<Vec<Showing>>();

        Ok(Game { id, showings })
    }
}

fn main() {
    let res: usize = include_str!("./day2.input.txt")
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| Game::from_str(line).unwrap())
        .filter(|game| game.is_valid())
        .map(|game| game.id as usize)
        .sum();

    println!("{res} is the sum of the game ids");
}
