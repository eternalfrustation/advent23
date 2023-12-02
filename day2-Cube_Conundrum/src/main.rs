#![feature(
    return_position_impl_trait_in_trait,
    error_generic_member_access,
    provide_any
)]

use std::{error::Error, fmt::Display, str::FromStr};

fn main() {
    println!("{}", part2());
}

fn part2() -> usize {
    std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let game: Game = Game::from_str(l.as_str()).unwrap();
            let min_red = match game.red.iter().max() {
                Some(&x) => x,
                None => 0,
            };
            let min_green = match game.green.iter().max() {
                Some(&x) => x,
                None => 0,
            };
            let min_blue = match game.blue.iter().max() {
                Some(&x) => x,
                None => 0,
            };
            min_red * min_green * min_blue
        })
        .sum::<usize>()
}

fn part1() -> usize {
    std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let game: Game = Game::from_str(l.as_str()).unwrap();
            if game.is_possible() {
                game.id
            } else {
                0
            }
        })
        .sum::<usize>()
}

struct Game {
    id: usize,
    red: Vec<usize>,
    green: Vec<usize>,
    blue: Vec<usize>,
}

#[derive(Debug)]
enum ParseError {
    ColonSeparatorNotFound,
    SetsNotFound,
    UnknownColor,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::ColonSeparatorNotFound => "Colon separator (': ') is not found in the string",
            ParseError::SetsNotFound => {
                "Sets section of the line (The part after ': ') is not found"
            }
            ParseError::UnknownColor => "Unknown Color is being specified",
        })
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn provide<'a>(&'a self, demand: &mut std::any::Demand<'a>) {}
}

impl FromStr for Game {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Game, Box<(dyn std::error::Error + 'static)>> {
        let s = s.strip_prefix("Game ").unwrap();
        let mut game_line = s.split(": ");
        let id = match game_line.next() {
            Some(v) => v,
            None => return Err(Box::new(ParseError::ColonSeparatorNotFound)),
        }
        .parse::<usize>()?;
        let num: usize = 0;
        let sets = match game_line.next() {
            Some(v) => v,
            None => return Err(Box::new(ParseError::SetsNotFound)),
        }
        .split("; ");
        let (mut red, mut blue, mut green) = (Vec::new(), Vec::new(), Vec::new());
        for set in sets {
            for token in set.split(", ") {
                let mut token = token.split_whitespace();
                let num: usize = token.next().unwrap().parse().unwrap();
                match token.next().unwrap() {
                    "red" => red.push(num),
                    "blue" => blue.push(num),
                    "green" => green.push(num),
                    _ => return Err(ParseError::UnknownColor.into()),
                }
            }
        }
        Ok(Game {
            id,
            red,
            green,
            blue,
        })
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        !(self.red.iter().filter(|&&x| x > 12).next().is_some()
            || self.green.iter().filter(|&&x| x > 13).next().is_some()
            || self.blue.iter().filter(|&&x| x > 14).next().is_some())
    }
}
