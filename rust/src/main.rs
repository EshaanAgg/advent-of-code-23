mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

#[macro_use]
extern crate scan_fmt;
#[macro_use]
extern crate anyhow;

pub fn main() {
    println!(
        "Solution [Day 5][Part 1]: {}",
        day5::part1("../data/5/input.txt")
    );

    println!(
        "Solution [Day 5][Part 2]: {}",
        day5::part2("../data/5/input.txt")
    );
}
