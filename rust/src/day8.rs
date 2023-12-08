use std::{collections::HashMap, fs};

pub fn part1(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("There was an error in reading the file");
    let mut inp = input.split("\n");

    let steps = inp
        .next()
        .expect("The first line should be the steps to be followed")
        .trim()
        .chars()
        .collect::<Vec<char>>();

    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    inp.skip(1).for_each(|line| {
        let (from, left, right) = scan_fmt!(line, "{} = ({}, {})", String, String, String)
            .expect("Failed to parse the line");
        graph.insert(from, (left, right));
    });

    let mut i = 0;
    let mut ans = 0;
    let mut curr = "AAA";

    while curr != "ZZZ" {
        ans += 1;
        curr = match steps[i] {
            'L' => graph
                .get(curr)
                .expect("Invalid graph entry on the left branch")
                .0
                .as_str(),
            'R' => graph
                .get(curr)
                .expect("Invalid graph entry on the right branch")
                .1
                .as_str(),
            _ => panic!("Invalid step"),
        };
        i = (i + 1) % steps.len();
    }

    ans
}

pub fn part2(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("There was an error in reading the file");
    let mut inp = input.split("\n");

    let steps = inp
        .next()
        .expect("The first line should be the steps to be followed")
        .trim()
        .chars()
        .collect::<Vec<char>>();

    let mut graph: HashMap<String, (String, String)> = HashMap::new();
    inp.skip(1).for_each(|line| {
        let (from, left, right) = scan_fmt!(line, "{} = ({}, {})", String, String, String)
            .expect("Failed to parse the line");
        graph.insert(from, (left, right));
    });

    let mut lcm = 1;

    graph
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(k, _)| k.as_str())
        .for_each(|c| {
            let mut curr = c;
            let mut i = 0;
            let mut ans = 0;

            while !curr.ends_with("Z") {
                ans += 1;

                curr = match steps[i] {
                    'L' => graph
                        .get(curr)
                        .expect("Invalid graph entry on the left branch")
                        .0
                        .as_str(),
                    'R' => graph
                        .get(curr)
                        .expect("Invalid graph entry on the right branch")
                        .1
                        .as_str(),
                    _ => panic!("Invalid step"),
                };
                i = (i + 1) % steps.len();
            }
            lcm = num::integer::lcm(lcm, ans);
        });

    lcm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/8/test1.txt"), 2);
        assert_eq!(part1("../data/8/test2.txt"), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("../data/8/test3.txt"), 6);
    }
}
