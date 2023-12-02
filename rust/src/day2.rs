use std::cmp::max;
use std::fmt::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Ball {
    Red(u32),
    Blue(u32),
    Green(u32),
}

impl FromStr for Ball {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(" ");

        let count = components
            .next()
            .expect("Each component should have a count")
            .parse::<u32>()
            .expect("The count should be a number");

        Ok(
            match components
                .next()
                .expect("Each component should have a type")
            {
                "red" => Ball::Red(count),
                "blue" => Ball::Blue(count),
                "green" => Ball::Green(count),
                _ => panic!("Unknown ball type"),
            },
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Draw {
    red: u32,
    blue: u32,
    green: u32,
}

impl Draw {
    fn is_contained(&self, other: &Draw) -> bool {
        self.red <= other.red && self.blue <= other.blue && self.green <= other.green
    }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

impl FromStr for Draw {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut draw = Draw {
            red: 0,
            blue: 0,
            green: 0,
        };

        s.trim()
            .split(",")
            .map(|ball| ball.trim().parse::<Ball>().expect("Failed to parse ball"))
            .for_each(|ball| match ball {
                Ball::Red(count) => draw.red += count,
                Ball::Blue(count) => draw.blue += count,
                Ball::Green(count) => draw.green += count,
            });

        Ok(draw)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn minimum_draw(&self) -> Draw {
        let mut draw = Draw {
            red: 0,
            blue: 0,
            green: 0,
        };

        self.draws.iter().for_each(|d| {
            draw.red = max(d.red, draw.red);
            draw.blue = max(d.blue, draw.blue);
            draw.green = max(d.green, draw.green);
        });

        draw
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(":");

        let id = components
            .next()
            .expect("Each game should have an associated ID identifier")
            .split(" ")
            .skip(1)
            .next()
            .expect("Each game must have an id")
            .parse::<u32>()
            .expect("The game id should be a number");

        let draws = components
            .next()
            .expect("Each game should have atleast one draw")
            .split(";")
            .map(|draw| {
                draw.parse::<Draw>()
                    .expect("Cannot be parsed into a valid Draw")
            })
            .collect::<Vec<Draw>>();

        Ok(Game { id, draws })
    }
}

pub fn part1(file_path: &str) -> u32 {
    let encompassing_draw = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    fs::read_to_string(file_path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            line.parse::<Game>()
                .expect("Failed to parse a valid game from line")
        })
        .filter(|game| game.minimum_draw().is_contained(&encompassing_draw))
        .map(|game| game.id)
        .sum()
}

pub fn part2(file_path: &str) -> u32 {
    fs::read_to_string(file_path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            line.parse::<Game>()
                .expect("Failed to parse a valid game from line")
        })
        .map(|game| game.minimum_draw().power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = part1("../data/2/test.txt");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_2() {
        let result = part2("../data/2/test.txt");
        assert_eq!(result, 2286);
    }
}
