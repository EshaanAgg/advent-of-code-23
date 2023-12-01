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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = part1("../data/1/test.txt");
        assert_eq!(result, 142);
    }
}
