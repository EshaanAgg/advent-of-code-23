use std::{fs, str::FromStr};

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    x: usize,
    y: usize,
    mapping: Vec<String>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Grid {
            grid: grid.clone(),
            x: grid.get(0).expect("The grid must not be empty").len(),
            y: grid.len(),
            mapping: Vec::from([s.to_string()]),
        })
    }
}

impl Grid {
    fn get_load(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, row)| (self.y - i) * row.iter().filter(|&x| *x == 'O').count())
            .sum()
    }

    fn tilt_y(&mut self, curr_start: usize, delta_curr: i64) {
        (0..self.x).for_each(|i| {
            let mut curr = curr_start as i64;
            let mut j = curr_start as i64;

            while j >= 0 && j < self.y as i64 {
                match self.grid[j as usize][i] {
                    '.' => (),
                    'O' => {
                        self.grid[j as usize][i] = '.';
                        self.grid[curr as usize][i] = 'O';
                        curr += delta_curr;
                    }
                    '#' => curr = j + delta_curr,
                    _ => panic!("Invalid character"),
                }

                j += delta_curr;
            }
        })
    }

    fn tilt_x(&mut self, curr_start: usize, delta_curr: i64) {
        (0..self.y).for_each(|j| {
            let mut curr = curr_start as i64;
            let mut i = curr_start as i64;

            while i >= 0 && i < self.x as i64 {
                match self.grid[j][i as usize] {
                    '.' => (),
                    'O' => {
                        self.grid[j][i as usize] = '.';
                        self.grid[j][curr as usize] = 'O';
                        curr += delta_curr;
                    }
                    '#' => curr = i + delta_curr,
                    _ => panic!("Invalid character"),
                }

                i += delta_curr;
            }
        })
    }

    fn cycle(&mut self) {
        // Rotate North -> West -> South -> East
        self.tilt_y(0, 1);
        self.tilt_x(0, 1);
        self.tilt_y(self.y - 1, -1);
        self.tilt_x(self.x - 1, -1);
    }

    fn cycle_n(&mut self, n: usize) -> usize {
        self.cycle();
        let key = self.to_string();

        match self.mapping.iter().enumerate().find(|(_, x)| *x == &key) {
            Some((index, _)) => self.mapping[index + (n - 1) % (self.mapping.len() - index)]
                .parse::<Grid>()
                .expect("Failed to parsed the cached state as a grid")
                .get_load(),
            _ => {
                self.mapping.push(self.to_string());
                self.cycle_n(n - 1)
            }
        }
    }

    fn get_north_load(&self) -> usize {
        (0..self.x)
            .map(|i| {
                self.grid
                    .iter()
                    .enumerate()
                    .fold((0, 0), |(curr, sum), (j, row)| match row[i] {
                        '.' => (curr, sum),
                        'O' => (curr + 1, sum + self.y - curr),
                        '#' => (j + 1, sum),
                        _ => panic!("Invalid character"),
                    })
            })
            .map(|(_, sum)| sum)
            .sum()
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub fn part1(file_path: &str) -> usize {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .parse::<Grid>()
        .expect("The given input can't be parsed as a grid")
        .get_north_load()
}

pub fn part2(file_path: &str) -> usize {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .parse::<Grid>()
        .expect("The given input can't be parsed as a grid")
        .cycle_n(1000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/14/test.txt"), 136);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("../data/14/test.txt"), 64);
    }
}
