use std::{cmp::max, fs};

fn get_in_dirs(c: char) -> ((i32, i32), (i32, i32)) {
    match c {
        'L' => ((-1, 0), (0, 1)),
        'J' => ((0, 1), (1, 0)),
        '7' => ((1, 0), (0, -1)),
        'F' => ((0, -1), (-1, 0)),
        '|' => ((0, -1), (0, 1)),
        '-' => ((-1, 0), (1, 0)),
        _ => ((0, 0), (0, 0)),
    }
}

fn is_connected(start: char, end: char, dx: i32, dy: i32) -> bool {
    let (d1, d2) = get_in_dirs(start);
    let (d3, d4) = get_in_dirs(end);

    let delta = (dx, dy);
    let neg_delta = (-dx, -dy);

    (d1 == neg_delta || d2 == neg_delta) && (d3 == delta || d4 == delta)
}

fn is_valid_move(grid: &Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    let xn = x + dx;
    let yn = y + dy;

    if xn < 0 || yn < 0 {
        return false;
    }

    if yn >= grid.len() as i32 || xn >= grid[0].len() as i32 {
        return false;
    }

    is_connected(
        grid[y as usize][x as usize],
        grid[yn as usize][xn as usize],
        dx,
        dy,
    )
}

fn get_length(grid: &Vec<Vec<char>>, x: i32, y: i32, sx: i32, sy: i32, dis: &mut Vec<Vec<i32>>) {
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        if is_valid_move(grid, x, y, dx, dy) {
            let xn = (x + dx) as usize;
            let yn = (y + dy) as usize;

            if (xn, yn) == (sx as usize, sy as usize) {
                dis[yn][xn] = max(dis[yn][xn], 1 + dis[y as usize][x as usize]);
                continue;
            }

            if dis[yn][xn] != -1 {
                continue;
            }

            dis[yn][xn] = dis[y as usize][x as usize] + 1;
            get_length(grid, x + dx, y + dy, sx, sy, dis);
        }
    }
}

pub fn part1(file_path: &str) -> i32 {
    let inp = fs::read_to_string(file_path).expect("There was an error in reading the file");

    let mut grid = inp
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut sx = 0;
    let mut sy = 0;
    grid.iter().enumerate().for_each(|(y, l)| {
        l.iter().enumerate().for_each(|(x, &c)| {
            if c == 'S' {
                sx = x as i32;
                sy = y as i32;
            }
        });
    });

    let mut len = 0;

    for s in ['|', '-', 'L', 'J', '7', 'F'] {
        grid[sy as usize][sx as usize] = s;

        let mut dis = vec![vec![-1; grid[0].len()]; grid.len()];
        dis[sy as usize][sx as usize] = 0;
        get_length(&grid, sx, sy, sx, sy, &mut dis);

        len = max(len, dis[sy as usize][sx as usize])
    }

    len / 2
}

fn apply_shoelace(points: &Vec<(i32, i32)>) -> i32 {
    let mut ans = 0;

    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % points.len()];

        ans += x1 * y2 - x2 * y1;
    }

    ans.abs() / 2
}

fn get_area(dis: &mut Vec<Vec<i32>>) -> i32 {
    let mut points = vec![];

    let mut sx = -1;
    let mut sy = -1;

    for y in 0..dis.len() {
        for x in 0..dis[0].len() {
            if dis[y][x] == 1 {
                sx = x as i32;
                sy = y as i32;
                break;
            }
        }
    }

    let mut d_max = -1;

    loop {
        points.push((sx, sy));

        let mut found = false;

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let xn = sx + dx;
            let yn = sy + dy;
            if xn < 0 || yn < 0 || xn == dis[0].len() as i32 || yn == dis.len() as i32 {
                continue;
            }

            let yn = yn as usize;
            let xn = xn as usize;
            if dis[yn][xn] == dis[sy as usize][sx as usize] + 1 {
                sx = xn as i32;
                sy = yn as i32;
                found = true;
                d_max = dis[yn][xn];
                break;
            }
        }

        if !found {
            break;
        }
    }

    // Pick's Theorem
    apply_shoelace(&points) + 1 - d_max / 2
}

pub fn part2(file_path: &str) -> i32 {
    let inp = fs::read_to_string(file_path).expect("There was an error in reading the file");

    let mut grid = inp
        .lines()
        .map(|l| {
            let mut chars = l.chars().collect::<Vec<char>>();
            chars.insert(0, '.');
            chars.push('.');
            chars
        })
        .collect::<Vec<_>>();

    grid.push(vec!['.'; grid[0].len()]);

    let mut sx = 0;
    let mut sy = 0;
    grid.iter().enumerate().for_each(|(y, l)| {
        l.iter().enumerate().for_each(|(x, &c)| {
            if c == 'S' {
                sx = x as i32;
                sy = y as i32;
            }
        });
    });

    for s in ['|', '-', 'L', 'J', '7', 'F'] {
        let mut count = 0;
        grid[sy as usize][sx as usize] = s;

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if is_valid_move(&grid, sx, sy, dx, dy) {
                count += 1;
            }
        }

        if count == 2 {
            break;
        }
    }

    let mut dis = vec![vec![-1; grid[0].len()]; grid.len()];
    dis[sy as usize][sx as usize] = 0;
    get_length(&grid, sx, sy, sx, sy, &mut dis);

    get_area(&mut dis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/10/test1.txt"), 4);
        assert_eq!(part1("../data/10/test2.txt"), 8);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("../data/10/test3.txt"), 4);
        assert_eq!(part2("../data/10/test4.txt"), 8);
        assert_eq!(part2("../data/10/test5.txt"), 10);
    }
}
