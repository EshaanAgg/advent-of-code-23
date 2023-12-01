use std::fs;

pub fn part1(file_path: &str) -> u32 {
    fs::read_to_string(file_path)
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| {
                    c.to_digit(10)
                        .expect("Failed to convert character to digit")
                })
                .collect::<Vec<u32>>()
        })
        .map(|vec| {
            10 * vec.first().expect("Every line must have atleast one digit")
                + vec.last().expect("Every line must have atleast one digit")
        })
        .sum()
}

pub fn part2(file_path: &str) -> u32 {
    fs::read_to_string(file_path)
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|line| {
            line.to_string()
                .replace("zero", "zero0zero")
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| {
                    c.to_digit(10)
                        .expect("Failed to convert character to digit")
                })
                .collect::<Vec<u32>>()
        })
        .map(|vec| {
            10 * vec.first().expect("Every line must have atleast one digit")
                + vec.last().expect("Every line must have atleast one digit")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = part1("../data/1/test1.txt");
        assert_eq!(result, 142);
    }

    #[test]
    fn test_2() {
        let result = part2("../data/1/test2.txt");
        assert_eq!(result, 281);
    }
}
