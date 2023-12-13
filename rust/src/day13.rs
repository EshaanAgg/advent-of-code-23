use std::{cmp::min, fs, str::FromStr};

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    x: usize,
    y: usize,
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
        })
    }
}

impl Grid {
    fn has_vertical_symmetry(&self, after: usize) -> bool {
        let len = min(after, self.x - after);
        (0..len).all(|i| {
            self.grid
                .iter()
                .all(|row| row[after - i - 1] == row[after + i])
        })
    }

    fn has_horizontal_symmetry(&self, after: usize) -> bool {
        let len = min(after, self.y - after);
        (0..len).all(|i| self.grid[after - i - 1] == self.grid[after + i])
    }

    fn get_vertical_score(&self) -> Option<usize> {
        (1..self.x).find(|&i| self.has_vertical_symmetry(i))
    }

    fn get_horizontal_score(&self) -> Option<usize> {
        (1..self.y)
            .find(|&i| self.has_horizontal_symmetry(i))
            .map(|i| i * 100)
    }

    fn get_score(&self) -> usize {
        match (self.get_vertical_score(), self.get_horizontal_score()) {
            (Some(v), None) => v,
            (None, Some(h)) => h,
            _ => panic!("The grid must have exactly one symmetry line"),
        }
    }
}

pub fn part1(file_path: &str) -> usize {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .split("\n\n")
        .map(|x| x.parse::<Grid>().expect("Failed to parse as a grid"))
        .map(|g| g.get_score())
        .sum()
}

pub fn part2(file_path: &str) -> usize {
    todo!("Implement part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/13/test.txt"), 405);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("../data/12/test.txt"), 400);
    }
}
