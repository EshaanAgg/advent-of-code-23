use std::{fs, str::FromStr};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn get_direction(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug)]
struct Move {
    dir: Direction,
    len: i64,
}

impl FromStr for Move {
    type Err = ();

    // For part 1
    // fn from_str(s: &str) -> Result<Self, Self::Err> {
    //     let parts: Vec<&str> = s.split(" ").collect();

    //     Ok(Move {
    //         dir: parts[0].parse().expect("Failed to parse a valid direction"),
    //         len: parts[1].parse().expect("Failed to parse a valid length"),
    //     })
    // }

    // For part 2
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s
            .split(" ")
            .skip(2)
            .next()
            .expect("Failed to get the hex string")
            .to_string();

        Ok(Move {
            dir: match hex
                .chars()
                .nth(7)
                .expect("Failed to get the direction character")
            {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("Invalid direction"),
            },
            len: i64::from_str_radix(&hex[2..7], 16)
                .expect("Failed to parse the length as a valid hexadecimal number"),
        })
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn next_point(&self, movement: &Move) -> Point {
        let (dx, dy) = movement.dir.get_direction();

        Point {
            x: self.x + dx * movement.len,
            y: self.y + dy * movement.len,
        }
    }

    fn get_manhattan_distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn get_enclosed_area(points: &Vec<Point>) -> i64 {
    points
        .windows(2)
        .fold(0, |acc, ele| {
            acc + ele[0].x * ele[1].y - ele[1].x * ele[0].y
        })
        .abs()
        / 2
}

fn get_points_on_boundary(points: &Vec<Point>) -> i64 {
    points
        .windows(2)
        .fold(0, |acc, ele| acc + ele[0].get_manhattan_distance(&ele[1]))
}

fn get_area(points: &Vec<Point>) -> i64 {
    // Pick's Theorem
    get_enclosed_area(points) + 1 + get_points_on_boundary(points) / 2
}

pub fn solve(file_path: &str) -> i64 {
    get_area(
        &fs::read_to_string(file_path)
            .expect("Failed to read the file")
            .lines()
            .map(|line| {
                line.parse::<Move>()
                    .expect("Failed to parse the line as a valid move")
            })
            .fold(vec![Point { x: 0, y: 0 }], |mut acc, ele| {
                acc.push(
                    acc.last()
                        .expect("The accumulator vector must have atleast one element")
                        .next_point(&ele),
                );
                acc
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve("../data/18/test.txt"), 62);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve("../data/18/test.txt"), 952408144115);
    }
}
