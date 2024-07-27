use std::{cmp::max, fmt::Error};

use crate::helpers::read_lines;

#[derive(Debug)]
struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}
impl Draw {
    fn new(red: i32, green: i32, blue: i32) -> Draw {
        Draw { red, green, blue }
    }
    fn maximize_each_entry(&self, other: &Draw) -> Draw {
        Draw::new(
            max(self.red, other.red),
            max(self.green, other.green),
            max(self.blue, other.blue),
        )
    }
}
impl TryFrom<&str> for Draw {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let colors = value.split(",");
        for color in colors {
            let mut number_then_color = color.split(" ").skip(1);
            let number = number_then_color.next().unwrap().parse().unwrap();
            let color = number_then_color.next().unwrap();
            match color {
                "red" => {
                    red = number;
                }
                "green" => {
                    green = number;
                }
                "blue" => {
                    blue = number;
                }
                _ => panic!(),
            }
        }
        Ok(Draw { red, green, blue })
    }
}

#[derive(Debug)]
struct Game {
    number: i32,
    draws: Vec<Draw>,
}

impl TryFrom<&String> for Game {
    type Error = Error;
    // should do error handling...
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        // separate game number from data: Game 2: game data
        let mut game = value.split(":");
        let number: i32 = game
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let draws = game.next().unwrap().split(";");
        let draws = draws.map(|x| x.try_into().unwrap()).collect();
        Ok(Game { number, draws })
    }
}

fn possible_draw(occurred: &Game, compare: &Draw) -> bool {
    // check if the 'occurred' draws are all less than or equal, in every entry, to the compare draw
    occurred
        .draws
        .iter()
        .all(|x| x.blue <= compare.blue && x.red <= compare.red && x.green <= compare.green)
}

pub fn day2a() {
    let lines = read_lines("inputs/day2.txt");
    let games: Vec<Game> = lines.iter().map(|l| l.try_into().unwrap()).collect();
    let compare = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };
    let possible_games = games.iter().filter(|x| possible_draw(x, &compare));
    let sum_of_ids = possible_games.fold(0, |cm, x| cm + x.number);
    println!("DAY2A: Sum of ids of possible games {}", sum_of_ids);
}

/// Returns a draw:( but this represents a game
/// Panics if there are no draws in the game
fn minimum_game(game: Game) -> Draw {
    game.draws
        .into_iter()
        .reduce(|cm, x| cm.maximize_each_entry(&x))
        .unwrap()
}

pub fn day2b() {
    let lines = read_lines("inputs/day2.txt");
    let games: Vec<Game> = lines.iter().map(|l| l.try_into().unwrap()).collect();
    let minimum_games: Vec<Draw> = games.into_iter().map(minimum_game).collect();
    let sum_of_powers = minimum_games
        .iter()
        .fold(0, |cm, x| cm + x.blue * x.green * x.red);
    println!("DAY2B: The sum of powers is {}", sum_of_powers);
}
