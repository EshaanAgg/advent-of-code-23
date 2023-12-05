use anyhow::Error;
use std::{fs, str::FromStr};

#[derive(Debug)]
struct Mapping {
    source_start: u64,
    end_start: u64,
    length: u64,
}

impl FromStr for Mapping {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (end_start, source_start, length) = scan_fmt!(s, "{} {} {}", u64, u64, u64)?;
        Ok(Mapping {
            source_start,
            end_start,
            length,
        })
    }
}

impl Mapping {
    fn convert(&self, tar: u64) -> (u64, bool) {
        if tar >= self.source_start && tar < self.source_start + self.length {
            (tar - self.source_start + self.end_start, true)
        } else {
            (tar, false)
        }
    }
}

#[derive(Debug)]
struct Conversion {
    mappings: Vec<Mapping>,
}

impl Conversion {
    fn convert(&self, s: u64) -> u64 {
        let mut ans = s;
        let mut done = false;
        self.mappings.iter().for_each(|m| {
            if !done {
                (ans, done) = m.convert(ans);
            }
        });
        ans
    }
}

impl FromStr for Conversion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Conversion {
            mappings: s
                .lines()
                .skip(1)
                .map(|l| {
                    l.parse::<Mapping>()
                        .expect("Failed to parse the line as a valid mapping")
                })
                .collect(),
        })
    }
}

pub fn part1(file_path: &str) -> u64 {
    let content = fs::read_to_string(file_path).expect("There was an error in reading the file");
    let mut parts = content.split("\n\n");

    let mut seeds: Vec<u64> = parts
        .next()
        .expect("The input must have seeds in the first part")
        .split(" ")
        .skip(1)
        .map(|i| {
            i.parse::<u64>()
                .expect("The seed could not be parsed as a number")
        })
        .collect();

    let conversions = parts
        .map(|p| {
            p.parse::<Conversion>()
                .expect("Failed to parse as a valid conversion")
        })
        .collect::<Vec<_>>();

    *seeds
        .iter_mut()
        .map(|s| {
            let val = s;
            conversions.iter().for_each(|m| {
                *val = m.convert(*val);
            });
            val
        })
        .min()
        .expect("The vector should have alteast one element")
}

pub fn part2(file_path: &str) -> u64 {
    let content = fs::read_to_string(file_path).expect("There was an error in reading the file");
    let mut parts = content.split("\n\n");

    let mut seeds: Vec<u64> = parts
        .next()
        .expect("The input must have seeds in the first part")
        .split(" ")
        .skip(1)
        .map(|i| {
            i.parse::<u64>()
                .expect("The seed could not be parsed as a number")
        })
        .collect();

    let conversions = parts
        .map(|p| {
            p.parse::<Conversion>()
                .expect("Failed to parse as a valid conversion")
        })
        .collect::<Vec<_>>();

    let mut m = u64::MAX;
    seeds.chunks(2).for_each(|c| {
        for s in c[0]..c[0] + c[1] {
            let mut val = s;
            conversions.iter().for_each(|m| {
                val = m.convert(val);
            });
            m = m.min(val);
        }
    });

    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = part1("../data/5/test.txt");
        assert_eq!(result, 35);
    }

    #[test]
    fn test_2() {
        let result = part2("../data/5/test.txt");
        assert_eq!(result, 46);
    }
}
