mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

#[macro_use]
extern crate scan_fmt;
#[macro_use]
extern crate anyhow;

pub fn main() {
    println!(
        "Solution [Day 6][Part 1]: {}",
        day6::part1("../data/6/input.txt")
    );

    println!(
        "Solution [Day 6][Part 2]: {}",
        day6::part2("../data/6/input.txt")
    );
}
