mod day19;

#[macro_use]
extern crate scan_fmt;

pub fn main() {
    println!(
        "Solution [Day 19][Part 1]: {}",
        day19::part1("../data/19/input.txt")
    );

    println!(
        "Solution [Day 19][Part 2]: {}",
        day19::part2("../data/19/input.txt")
    );
}
