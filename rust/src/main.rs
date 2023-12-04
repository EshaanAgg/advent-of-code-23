mod day1;
mod day2;
mod day3;
mod day4;

#[macro_use]
extern crate scan_fmt;
#[macro_use]
extern crate anyhow;

pub fn main() {
    println!(
        "Solution [Day 4][Part 1]: {}",
        day4::part1("../data/4/input.txt")
    );

    println!(
        "Solution [Day 4][Part 2]: {}",
        day4::part2("../data/4/input.txt")
    );
}
