use std::{collections::BinaryHeap, fs, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    fn move_point(
        &self,
        point: (usize, usize),
        dimensions: (usize, usize),
    ) -> Option<(usize, usize)> {
        let (x, y) = self.get_direction();
        let (x, y) = (x + point.0 as i64, y + point.1 as i64);

        if x < 0 || y < 0 || x >= dimensions.0 as i64 || y >= dimensions.1 as i64 {
            None
        } else {
            Some((x as usize, y as usize))
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Invalid index provided"),
        }
    }

    fn are_opposite(index1: usize, index2: usize) -> bool {
        match (index1, index2) {
            (0, 1) | (1, 0) | (2, 3) | (3, 2) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct QueueItem {
    point: (usize, usize),
    direction: usize,
    count: usize,
    distance: usize,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<usize>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .expect("Failed to parse the char as a number")
                            as usize
                    })
                    .collect()
            })
            .collect();

        let width = grid[0].len();
        let height = grid.len();

        Ok(Grid {
            grid,
            width,
            height,
        })
    }
}

impl Grid {
    fn get_min_path(self, min_steps: usize, max_steps: usize) -> usize {
        let mut distance =
            vec![vec![vec![vec![usize::MAX; max_steps + 1]; 4]; self.width]; self.height];
        let mut queue: BinaryHeap<QueueItem> = BinaryHeap::new();

        // Initialize the queue
        for i in 0..4 {
            queue.push(QueueItem {
                point: (0, 0),
                direction: i,
                count: 0,
                distance: 0,
            });
            distance[0][0][i][0] = 0;
        }

        // Process the queue
        while let Some(top) = queue.pop() {
            // Check from the global table if the current entry should be processed
            if top.distance != distance[top.point.1][top.point.0][top.direction][top.count] {
                continue;
            }

            if top.point == (self.width - 1, self.height - 1) && top.count >= min_steps {
                return top.distance;
            }

            for dir_index in 0..4 {
                if Direction::are_opposite(dir_index, top.direction) {
                    continue;
                }

                // Check if the new point is lying inside the grid
                if let Some(new_point) = Direction::from_index(dir_index)
                    .move_point(top.point, (self.width, self.height))
                {
                    if (dir_index == top.direction && top.count == max_steps)
                        || (dir_index != top.direction && top.count < min_steps)
                    {
                        continue;
                    }

                    let num_step = if top.direction == dir_index {
                        top.count + 1
                    } else {
                        1
                    };

                    if distance[new_point.1][new_point.0][dir_index][num_step]
                        > top.distance + self.grid[new_point.1][new_point.0]
                    {
                        distance[new_point.1][new_point.0][dir_index][num_step] =
                            top.distance + self.grid[new_point.1][new_point.0];
                        queue.push(QueueItem {
                            point: new_point,
                            direction: dir_index,
                            count: num_step,
                            distance: distance[new_point.1][new_point.0][dir_index][num_step],
                        });
                    }
                }
            }
        }

        0
    }
}

pub fn part1(file_path: &str) -> usize {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .parse::<Grid>()
        .expect("Failed to parse the same as a valid grid")
        .get_min_path(0, 3)
}

pub fn part2(file_path: &str) -> usize {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .parse::<Grid>()
        .expect("Failed to parse the same as a valid grid")
        .get_min_path(4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/17/test1.txt"), 102);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("../data/17/test1.txt"), 94);
        assert_eq!(part2("../data/17/test2.txt"), 71);
    }
}
