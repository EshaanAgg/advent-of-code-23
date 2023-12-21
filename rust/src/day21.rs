use queues::*;
use std::{fs, str::FromStr};

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    dim: (usize, usize),
    start: (usize, usize),
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let dim = (grid.len(), grid[0].len());
        let start = grid
            .iter()
            .enumerate()
            .find_map(|(i, row)| row.iter().position(|&x| x == 'S').map(|j| (i, j)))
            .expect("Unable to find the starting point");

        Ok(Grid { grid, dim, start })
    }
}

impl Grid {
    fn get_count(&self, steps: usize) -> usize {
        let mut dist = vec![vec![usize::MAX; self.dim.1]; self.dim.0];
        let mut q = Queue::new();

        let _ = q.add(self.start);
        dist[self.start.0][self.start.1] = 0;

        while q.size() != 0 {
            let (i, j) = q.remove().unwrap();
            let d = dist[i][j];

            if d == steps {
                break;
            }

            for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);

                if ni < 0 || ni >= self.dim.0 as i32 || nj < 0 || nj >= self.dim.1 as i32 {
                    continue;
                }

                let (ni, nj) = (ni as usize, nj as usize);
                if self.grid[ni][nj] == '#' {
                    continue;
                }

                if dist[ni][nj] > d + 1 {
                    dist[ni][nj] = d + 1;
                    let _ = q.add((ni, nj));
                }
            }
        }

        dist.iter()
            .map(|row| {
                row.iter()
                    .filter(|&&x| x != usize::MAX && x % 2 == steps % 2)
                    .count()
            })
            .sum()
    }
}

pub fn part1(file_path: &str, steps: usize) -> usize {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .parse::<Grid>()
        .expect("Failed to parse a valid grid from the input")
        .get_count(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/21/test.txt", 6), 16);
    }

    // #[test]
    // fn test_2() {
    //     assert_eq!(part2("../data/13/test.txt"), 400);
    // }
}
