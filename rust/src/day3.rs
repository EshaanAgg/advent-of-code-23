use std::fs;

fn is_symbol(c: char) -> bool {
    return !c.is_ascii_digit() && c != '.';
}

#[derive(Debug)]
struct Number {
    start: u32,
    end: u32,
    y: u32,
    value: u32,
}

impl Number {
    fn parse_line(y: u32, line: &str) -> Vec<Number> {
        let mut result: Vec<Number> = Vec::new();
        let mut start = -1;

        line.chars().enumerate().for_each(|(index, val)| {
            if val.is_ascii_digit() {
                if start == -1 {
                    start = index as i32;
                }
            } else {
                if start != -1 {
                    result.push(Number {
                        start: start as u32,
                        end: index as u32 - 1,
                        y,
                        value: line[start as usize..index]
                            .parse()
                            .expect("Failed to parse the provided substring as a number"),
                    });
                    start = -1;
                }
            }
        });

        // Handle the case where the line ends with a number
        if start != -1 {
            result.push(Number {
                start: start as u32,
                end: line.len() as u32 - 1,
                y,
                value: line[start as usize..]
                    .parse()
                    .expect("Failed to parse the provided substring as a number"),
            });
        }

        return result;
    }

    fn has_adjacent_symbol(&self, grid: &Grid) -> bool {
        (self.start..=self.end).any(|x| grid.has_symbol_neighbour(x, self.y))
    }

    fn is_adjacent(&self, x: u32, y: u32) -> bool {
        if self.y == y {
            self.start == x + 1 || self.end == x - 1
        } else if self.y == y + 1 || self.y == y - 1 {
            self.start <= x + 1 && self.end >= x - 1
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Grid<'a> {
    x: u32,
    y: u32,
    numbers: Vec<Number>,
    grid: Vec<&'a str>,
}

impl<'a> Grid<'a> {
    fn new(input: &str) -> Grid {
        let mut numbers: Vec<Number> = Vec::new();

        input.lines().enumerate().for_each(|(y, index)| {
            numbers.append(&mut Number::parse_line(y as u32, index));
        });

        return Grid {
            x: input
                .lines()
                .next()
                .expect("The grid should not be empty")
                .len() as u32,
            y: input.lines().count() as u32,
            numbers,
            grid: input.lines().collect(),
        };
    }

    fn has_symbol_neighbour(&self, x: u32, y: u32) -> bool {
        for dx in -1..2 {
            for dy in -1..2 {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0
                    && nx < self.x as i32
                    && ny >= 0
                    && ny < self.y as i32
                    && is_symbol(
                        self.grid[ny as usize]
                            .chars()
                            .nth(nx as usize)
                            .expect("Expected to fetch a character from the grid"),
                    )
                {
                    return true;
                }
            }
        }

        false
    }

    fn get_power(&self, x: u32, y: u32) -> u32 {
        let adj: Vec<&Number> = self
            .numbers
            .iter()
            .filter(|n| n.is_adjacent(x, y))
            .collect();

        if adj.len() == 2 {
            adj[0].value * adj[1].value
        } else {
            0
        }
    }
}

pub fn part1(file_path: &str) -> u32 {
    let inp = fs::read_to_string(file_path).expect("Unable to read the input file");
    let g = Grid::new(&inp);

    g.numbers
        .iter()
        .filter(|n| n.has_adjacent_symbol(&g))
        .map(|n| n.value)
        .sum()
}

pub fn part2(file_path: &str) -> u32 {
    let inp = fs::read_to_string(file_path).expect("Unable to read the input file");
    let g = Grid::new(&inp);

    let mut sum = 0;

    for y in 0..g.y {
        for x in 0..g.x {
            if g.grid[y as usize]
                .chars()
                .nth(x as usize)
                .expect("Expected to fetch a character")
                == '*'
            {
                sum += g.get_power(x, y)
            }
        }
    }

    return sum;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = part1("../data/3/test.txt");
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_2() {
        let result = part2("../data/3/test.txt");
        assert_eq!(result, 467835);
    }
}
