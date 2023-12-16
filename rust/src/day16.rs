use std::{fs, str::FromStr};

#[derive(Debug)]
struct Ray {
    location: (i64, i64),
    direction: (i64, i64),
}

// Returns a boolean stating whether the ray should be removed
fn move_ray(index: usize, x_max: i64, y_max: i64, grid: &mut Grid) -> bool {
    let ray = &mut grid.rays[index];

    let (x, y) = ray.location;
    let (dx, dy) = ray.direction;

    // Check to done to ensure that the ray is not an initialization ray
    if x >= 0 && x < x_max && y >= 0 && y < y_max {
        if grid.dirs_covered[y as usize][x as usize].contains(&(dx, dy)) {
            return true;
        }
        grid.dirs_covered[y as usize][x as usize].push((dx, dy));
    }

    let (x_new, y_new) = (x + dx, y + dy);
    if x_new < 0 || x_new >= x_max || y_new < 0 || y_new >= y_max {
        return true;
    }

    ray.location = (x_new, y_new);
    let x_new = x_new as usize;
    let y_new = y_new as usize;

    // Check for reflection
    match grid.grid[y_new][x_new] {
        '/' => {
            let org_dir = ray.direction;
            ray.direction = (-org_dir.1, -org_dir.0);
            false
        }
        '\\' => {
            let org_dir = ray.direction;
            ray.direction = (org_dir.1, org_dir.0);
            false
        }
        '|' => {
            if dx != 0 {
                grid.rays.push(Ray {
                    location: (x_new as i64, y_new as i64),
                    direction: (0, 1),
                });
                grid.rays.push(Ray {
                    location: (x_new as i64, y_new as i64),
                    direction: (0, -1),
                });
                return true;
            }
            false
        }
        '-' => {
            if dy != 0 {
                grid.rays.push(Ray {
                    location: (x_new as i64, y_new as i64),
                    direction: (1, 0),
                });
                grid.rays.push(Ray {
                    location: (x_new as i64, y_new as i64),
                    direction: (-1, 0),
                });
                return true;
            }
            false
        }
        '.' => false,
        _ => panic!("Invalid character found in the grid"),
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    rays: Vec<Ray>,
    // Keeps track of the directions of rays that have entered the cell of the grid
    dirs_covered: Vec<Vec<Vec<(i64, i64)>>>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let width = grid[0].len();
        let height = grid.len();

        Ok(Grid {
            grid,
            width,
            height,
            rays: Vec::from([Ray {
                location: (-1, 0),
                direction: (1, 0),
            }]),
            dirs_covered: vec![vec![Vec::new(); width]; height],
        })
    }
}

impl Grid {
    fn count_explored(&self) -> usize {
        self.dirs_covered
            .iter()
            .map(|l| l.iter().filter(|ele| ele.len() != 0).count())
            .sum()
    }

    fn run(&mut self) {
        while self.rays.len() != 0 {
            let mut to_remove = Vec::new();

            for i in 0..self.rays.len() {
                let remove = move_ray(i, self.width as i64, self.height as i64, self);
                if remove {
                    to_remove.push(i);
                }
            }

            // Remove in opposite order so as to not disturb the indices
            for i in to_remove.iter().rev() {
                self.rays.remove(*i);
            }
        }
    }

    fn reset(&mut self) {
        self.rays = Vec::new();
        self.dirs_covered = vec![vec![Vec::new(); self.width]; self.height];
    }

    fn get_run_score(&mut self, init_dir: (i64, i64), init_loc: (i64, i64)) -> usize {
        self.reset();
        self.rays.push(Ray {
            location: init_loc,
            direction: init_dir,
        });
        self.run();
        self.count_explored()
    }
}

pub fn part1(file_path: &str) -> usize {
    let mut grid = fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .parse::<Grid>()
        .expect("Failed to parse the same as a valid grid");

    grid.run();
    grid.count_explored()
}

pub fn part2(file_path: &str) -> usize {
    let mut grid = fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .parse::<Grid>()
        .expect("Failed to parse the same as a valid grid");

    let mut max_explored = 0;

    (0..grid.width).for_each(|p| {
        // From the top
        max_explored = max_explored.max(grid.get_run_score((0, 1), (p as i64, -1)));
        // From the bottom
        max_explored =
            max_explored.max(grid.get_run_score((0, -1), (p as i64, grid.height as i64)));
    });

    (0..grid.height).for_each(|p| {
        // From the left
        max_explored = max_explored.max(grid.get_run_score((1, 0), (-1, p as i64)));
        // From the right
        max_explored = max_explored.max(grid.get_run_score((-1, 0), (grid.width as i64, p as i64)));
    });

    max_explored
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/16/test.txt"), 46);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("../data/16/test.txt"), 51);
    }
}
