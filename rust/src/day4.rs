use anyhow::Error;
use std::{cmp::min, fs, str::FromStr, vec};

#[derive(Debug)]
struct Game {
    winners: Vec<u32>,
    tickets: Vec<u32>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ticks = s
            .split(":")
            .filter(|l| !l.is_empty())
            .nth(1)
            .expect("There should be a list of tickets after the : in each line")
            .split("|");

        Ok(Game {
            winners: ticks
                .next()
                .expect("There should be a list of winners")
                .trim()
                .split(" ")
                .filter(|w| !w.is_empty())
                .map(|w| {
                    w.parse::<u32>()
                        .expect("The ticket could not be parsed as winner")
                })
                .collect(),
            tickets: ticks
                .next()
                .expect("There should be a list of winners")
                .trim()
                .split(" ")
                .filter(|w| !w.is_empty())
                .map(|w| {
                    w.parse::<u32>()
                        .expect("The ticket could not be parsed as winner")
                })
                .collect(),
        })
    }
}

pub fn part1(file_path: &str) -> u32 {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .lines()
        .map(|l| {
            l.parse::<Game>()
                .expect("The game could not be parsed from the provided line")
        })
        .map(|g| g.tickets.iter().filter(|t| g.winners.contains(t)).count() as u32)
        .map(|s| match s {
            0 => 0,
            t => 2_i32.pow(t - 1) as u32,
        })
        .sum()
}

fn increment(freq: &mut Vec<u32>, index: usize, count: u32) {
    for i in index + 1..min(index + count as usize + 1, freq.len()) {
        freq[i] += freq[index];
    }
}

pub fn part2(file_path: &str) -> u32 {
    let win_count: Vec<u32> = fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .lines()
        .map(|l| {
            l.parse::<Game>()
                .expect("The game could not be parsed from the provided line")
        })
        .map(|g| g.tickets.iter().filter(|t| g.winners.contains(t)).count() as u32)
        .collect();

    let mut freq = vec![1; win_count.len()];
    win_count
        .iter()
        .enumerate()
        .for_each(|(i, &c)| increment(&mut freq, i, c));

    freq.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = part1("../data/4/test.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn test_2() {
        let result = part2("../data/4/test.txt");
        assert_eq!(result, 30);
    }
}
