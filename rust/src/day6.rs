use std::{
    cmp::{max, min},
    fs,
};

fn get_sol_count(t: i64, d: i64) -> u64 {
    let b = -(t as f64);
    let c = d as f64;

    let dis = b * b - (4 as f64) * c;
    if dis < 0 as f64 {
        0
    } else {
        let x1 = ((-b - dis.sqrt()) / (2 as f64)) as i64;
        let x2 = ((-b + dis.sqrt()) / (2 as f64)) as i64;
        let mut mi = t;
        let mut ma = 0;
        for del in -1..3 {
            let xn1 = x1 + del;
            if xn1 * xn1 - t * xn1 + d < 0 {
                mi = min(mi, xn1);
            }

            let xn2 = x2 + del;
            if xn2 * xn2 - t * xn2 + d < 0 {
                ma = max(ma, xn2);
            }
        }

        ma as u64 - mi as u64 + 1
    }
}

pub fn part1(file_path: &str) -> u64 {
    let content = fs::read_to_string(file_path).expect("There was an error in reading the file");
    let mut parts = content.split("\n");

    parts
        .next()
        .expect("The first line should have time identifiers")
        .split(" ")
        .skip(1)
        .filter(|r| !r.is_empty())
        .map(|r| {
            r.parse::<u64>()
                .expect("The time value can't be parsed as an integer")
        })
        .zip(
            parts
                .next()
                .expect("The second line should have time identifiers")
                .split(" ")
                .skip(1)
                .filter(|r| !r.is_empty())
                .map(|r| {
                    r.parse::<u64>()
                        .expect("The distance value can't be parsed as an integer")
                }),
        )
        .map(|(t, d)| get_sol_count(t as i64, d as i64))
        .fold(1, |acc, cur| acc * cur)
}

pub fn part2(file_path: &str) -> u64 {
    let content = fs::read_to_string(file_path).expect("There was an error in reading the file");
    let mut parts = content.split("\n");

    let time = parts
        .next()
        .expect("The first line should have time identifiers")
        .split(" ")
        .skip(1)
        .filter(|r| !r.is_empty())
        .map(|r| r.chars())
        .flatten()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let dis = parts
        .next()
        .expect("The second line should have time identifiers")
        .split(" ")
        .skip(1)
        .filter(|r| !r.is_empty())
        .map(|r| r.chars())
        .flatten()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    get_sol_count(time as i64, dis as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = part1("../data/6/test.txt");
        assert_eq!(result, 288);
    }

    #[test]
    fn test_2() {
        let result = part2("../data/6/test.txt");
        assert_eq!(result, 71503);
    }
}
