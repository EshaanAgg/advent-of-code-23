mod d7p1;
mod d7p2;
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
        "Solution [Day 7][Part 1]: {}",
        d7p1::part1("../data/7/input.txt")
    );

    println!(
        "Solution [Day 7][Part 2]: {}",
        d7p2::part2("../data/7/input.txt")
    );
}
