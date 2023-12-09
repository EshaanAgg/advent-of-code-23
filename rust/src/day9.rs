use std::fs;

fn get_next(arr: &Vec<i64>) -> i64 {
    if arr.iter().all(|&x| x == 0) {
        0
    } else {
        get_next(&arr.windows(2).map(|x| x[1] - x[0]).collect())
            + arr
                .get(arr.len() - 1)
                .expect("The provided array has no elements")
    }
}

fn get_prev(arr: &Vec<i64>) -> i64 {
    if arr.iter().all(|&x| x == 0) {
        0
    } else {
        arr.get(0).expect("The provided array has no elements")
            - get_prev(&arr.windows(2).map(|x| x[1] - x[0]).collect())
    }
}

pub fn part1(file_path: &str) -> i64 {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .lines()
        .map(|l| {
            get_next(
                &l.split(" ")
                    .map(|x| {
                        x.parse::<i64>()
                            .expect("Failed to parse the string as a i64")
                    })
                    .collect::<Vec<i64>>(),
            )
        })
        .sum()
}

pub fn part2(file_path: &str) -> i64 {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .lines()
        .map(|l| {
            get_prev(
                &l.split(" ")
                    .map(|x| {
                        x.parse::<i64>()
                            .expect("Failed to parse the string as a i64")
                    })
                    .collect::<Vec<i64>>(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/9/test.txt"), 114);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("../data/9/test.txt"), 2);
    }
}
