use std::{cmp::max, cmp::min, fs, str::FromStr};

#[derive(Debug)]
struct Grid {
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    galaxies: Vec<(usize, usize)>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        let empty_rows = grid
            .iter()
            .enumerate()
            .filter(|(_, r)| r.iter().all(|&c| c == '.'))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let empty_cols = (0..grid[0].len())
            .filter(|&i| grid.iter().all(|r| r[i] == '.'))
            .collect::<Vec<_>>();

        let galaxies = grid.iter().enumerate().fold(Vec::new(), |mut acc, (y, r)| {
            acc.extend(
                r.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == '#')
                    .map(|(x, _)| (x, y)),
            );
            acc
        });

        Ok(Grid {
            empty_rows,
            empty_cols,
            galaxies,
        })
    }
}

impl Grid {
    fn distance(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize), exp: usize) -> usize {
        let x = (x1 as i64 - x2 as i64).abs() as usize
            + self
                .empty_rows
                .iter()
                .filter(|&&r| r > min(y1, y2) && r < max(y1, y2))
                .count()
                * (exp - 1);
        let y = (y1 as i64 - y2 as i64).abs() as usize
            + self
                .empty_cols
                .iter()
                .filter(|&&c| c > min(x1, x2) && c < max(x1, x2))
                .count()
                * (exp - 1);
        x + y
    }

    fn get_pairwise_sum(&self, exp: usize) -> usize {
        self.galaxies
            .iter()
            .enumerate()
            .map(|(i, &g1)| {
                self.galaxies
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| j > &i)
                    .map(|(_, &g2)| self.distance(g1, g2, exp))
                    .sum::<usize>()
            })
            .sum()
    }
}

pub fn part1(file_path: &str) -> usize {
    let inp = fs::read_to_string(file_path).expect("There was an error in reading the file");
    let grid = inp
        .parse::<Grid>()
        .expect("Failed to parse the input as a grid");

    grid.get_pairwise_sum(2)
}

pub fn part2(file_path: &str, exp: usize) -> usize {
    let inp = fs::read_to_string(file_path).expect("There was an error in reading the file");
    let grid = inp
        .parse::<Grid>()
        .expect("Failed to parse the input as a grid");

    grid.get_pairwise_sum(exp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/11/test.txt"), 374);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("../data/11/test.txt", 10), 1030);
        assert_eq!(part2("../data/11/test.txt", 100), 8410);
    }
}
