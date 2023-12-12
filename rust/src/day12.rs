use std::fs;

fn joined_copy<T: Clone>(vector: Vec<T>, num_copies: usize) -> Vec<T> {
    let mut ans = Vec::new();
    for _ in 0..num_copies {
        ans.extend(vector.clone());
    }

    ans
}

fn parse_line(line: &str) -> (String, Vec<usize>) {
    let parts = line.split(" ").collect::<Vec<_>>();
    (
        parts[0].to_owned() + ".",
        parts[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>(),
    )
}

fn parse_line_2(line: &str) -> (String, Vec<usize>) {
    let parts = line.split(" ").collect::<Vec<_>>();
    (
        (0..5).map(|_| parts[0]).collect::<Vec<_>>().join("?") + ".",
        joined_copy(
            parts[1]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
            5,
        ),
    )
}

fn solve(
    i: usize,
    gi: usize,
    pattern: &str,
    groups: &Vec<usize>,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    // If we reach the end of groups, check if the remaining pattern is all '.' or '?'
    if gi == groups.len() {
        match (i..pattern.len()).any(|j| pattern.chars().nth(j).unwrap() == '#') {
            true => return 0,
            false => return 1,
        }
    }

    // If we reach the end of pattern, return 0
    if i == pattern.len() {
        return 0;
    }

    if let Some(ans) = dp[i][gi] {
        return ans;
    }

    let mut ans = 0;

    // Try to fit the group at the next index
    if pattern.chars().nth(i).unwrap() != '#' {
        ans += solve(i + 1, gi, pattern, groups, dp);
    }

    // Try to fit the group at the current index
    if i + groups[gi] < pattern.len()
        && (i..i + groups[gi]).all(|j| pattern.chars().nth(j).unwrap() != '.')
        && pattern.chars().nth(i + groups[gi]).unwrap() != '#'
    {
        ans += solve(i + groups[gi] + 1, gi + 1, pattern, groups, dp);
    }

    dp[i][gi] = Some(ans);
    ans
}

pub fn part1(file_path: &str) -> usize {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .lines()
        .map(|x| parse_line(x))
        .map(|(pattern, groups)| {
            let mut dp = vec![vec![None; groups.len()]; pattern.len()];
            let ans = solve(0, 0, pattern.as_str(), &groups, &mut dp);
            ans
        })
        .sum()
}

pub fn part2(file_path: &str) -> usize {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .lines()
        .map(|x| parse_line_2(x))
        .map(|(pattern, groups)| {
            let mut dp = vec![vec![None; groups.len()]; pattern.len()];
            let ans = solve(0, 0, pattern.as_str(), &groups, &mut dp);
            ans
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/12/test.txt"), 21);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("../data/12/test.txt"), 525152);
    }
}
