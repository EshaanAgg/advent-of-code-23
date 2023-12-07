use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Hand<'a> {
    FiveOfAKind(&'a str),
    FourOfAKind(&'a str),
    FullHouse(&'a str),
    ThreeOfAKind(&'a str),
    TwoPair(&'a str),
    OnePair(&'a str),
    HighCard(&'a str),
}

impl<'a> Hand<'a> {
    fn get(cards: &str) -> Hand {
        let mut freq: HashMap<char, usize> = HashMap::new();

        let mut max_char = '0';
        let mut max_count = 0;

        freq.insert('J', 0);
        cards.chars().for_each(|c| {
            let count = freq.entry(c).or_insert(0);
            *count += 1;
            if (*count > max_count || *count == max_count) && (c != 'J') {
                max_char = c;
                max_count = *count;
            }
        });

        if max_char != '0' {
            let add = freq.get(&'J').unwrap().clone();
            let count = freq.entry(max_char).or_insert(0);
            *count += add;
            freq.remove(&'J');
        }

        let mut label_counts = freq.values().collect::<Vec<_>>();
        label_counts.sort();

        match label_counts.as_slice() {
            [5] => Hand::FiveOfAKind(cards),
            [1, 4] => Hand::FourOfAKind(cards),
            [2, 3] => Hand::FullHouse(cards),
            [1, 1, 3] => Hand::ThreeOfAKind(cards),
            [1, 2, 2] => Hand::TwoPair(cards),
            [1, 1, 1, 2] => Hand::OnePair(cards),
            _ => Hand::HighCard(cards),
        }
    }
}

fn to_priority(c: char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 11,
        'J' => 2,
        _ => c.to_digit(10).expect("Invalid character provided") as usize + 1,
    }
}

fn compare_hand_str(hand1: &str, hand2: &str) -> std::cmp::Ordering {
    let cards1: Vec<_> = hand1.chars().map(to_priority).collect();
    let cards2: Vec<_> = hand2.chars().map(to_priority).collect();

    cards1.cmp(&cards2)
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Hand::FiveOfAKind(label1), Hand::FiveOfAKind(label2)) => {
                Some(compare_hand_str(label1, label2))
            }
            (Hand::FiveOfAKind(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Hand::FiveOfAKind(_)) => Some(std::cmp::Ordering::Less),

            (Hand::FourOfAKind(label1), Hand::FourOfAKind(label2)) => {
                Some(compare_hand_str(label1, label2))
            }
            (Hand::FourOfAKind(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Hand::FourOfAKind(_)) => Some(std::cmp::Ordering::Less),

            (Hand::FullHouse(label1), Hand::FullHouse(label2)) => {
                Some(compare_hand_str(label1, label2))
            }
            (Hand::FullHouse(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Hand::FullHouse(_)) => Some(std::cmp::Ordering::Less),

            (Hand::ThreeOfAKind(label1), Hand::ThreeOfAKind(label2)) => {
                Some(compare_hand_str(label1, label2))
            }
            (Hand::ThreeOfAKind(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Hand::ThreeOfAKind(_)) => Some(std::cmp::Ordering::Less),

            (Hand::TwoPair(label1), Hand::TwoPair(label2)) => {
                Some(compare_hand_str(label1, label2))
            }
            (Hand::TwoPair(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Hand::TwoPair(_)) => Some(std::cmp::Ordering::Less),

            (Hand::OnePair(label1), Hand::OnePair(label2)) => {
                Some(compare_hand_str(label1, label2))
            }
            (Hand::OnePair(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Hand::OnePair(_)) => Some(std::cmp::Ordering::Less),

            (Hand::HighCard(label1), Hand::HighCard(label2)) => {
                Some(compare_hand_str(label1, label2))
            }
        }
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Draw<'a> {
    hand: Hand<'a>,
    bid: usize,
}

impl<'a> PartialOrd for Draw<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.hand.partial_cmp(&other.hand);
    }
}

impl<'a> Ord for Draw<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part2(file_path: &str) -> usize {
    let inp = fs::read_to_string(file_path).expect("There was an error in reading the file");

    let mut draws: Vec<Draw> = inp
        .lines()
        .map(|line| {
            let (hand, rank) = line.split_at(5);
            Draw {
                hand: Hand::get(hand),
                bid: rank
                    .trim()
                    .parse::<usize>()
                    .expect("Unable to parse the bid as a number"),
            }
        })
        .collect();

    draws.sort();
    draws.iter().enumerate().map(|(i, d)| (i + 1) * d.bid).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let result = part2("../data/7/test.txt");
        assert_eq!(result, 5905);
    }
}
