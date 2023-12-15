use std::{fs, str::FromStr};

fn get_box_id(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, ch| ((acc + ch as usize) * 17) % 256)
}
enum Query {
    Update(usize, String, usize),
    Remove(usize, String),
}

impl FromStr for Query {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.contains("-") {
            true => Ok(Query::Remove(
                get_box_id(&s[..s.len() - 1]),
                s[..s.len() - 1].to_string(),
            )),
            false => {
                let equals_index = s.find('=').expect("The update query must have a '='");
                Ok(Query::Update(
                    get_box_id(&s[..equals_index]),
                    s[..equals_index].to_string(),
                    s[equals_index + 1..]
                        .parse()
                        .expect("Failed to convert the power of the lens to a number"),
                ))
            }
        }
    }
}

pub fn part1(file_path: &str) -> usize {
    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .split(",")
        .map(|l| get_box_id(l))
        .sum()
}

pub fn part2(file_path: &str) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![Vec::new(); 256];

    fs::read_to_string(file_path)
        .expect("There was an error in reading the file")
        .split(",")
        .map(|l| {
            l.parse::<Query>()
                .expect("Unable to pass the given part as a valid query")
        })
        .for_each(|q| match q {
            Query::Update(id, name, focal_length) => {
                match boxes[id].iter().position(|(n, _)| n == &name) {
                    Some(index) => boxes[id][index].1 = focal_length,
                    None => boxes[id].push((name, focal_length)),
                }
            }
            Query::Remove(id, name) => {
                boxes[id].retain(|(n, _)| n != &name);
            }
        });

    boxes
        .iter()
        .enumerate()
        .map(|(i, v)| -> usize {
            (i + 1)
                * v.iter()
                    .enumerate()
                    .map(|(j, r)| (j + 1) * r.1)
                    .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("../data/15/test.txt"), 1320);
    }

    #[test]
    fn test_2() {
        assert_eq!(part2("../data/15/test.txt"), 145);
    }
}
