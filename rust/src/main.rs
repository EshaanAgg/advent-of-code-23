mod d7p1;
mod d7p2;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day8;

#[macro_use]
extern crate scan_fmt;
#[macro_use]
extern crate anyhow;

pub fn main() {
    println!(
        "Solution [Day 8][Part 1]: {}",
        day8::part1("../data/8/input.txt")
    );

    println!(
        "Solution [Day 8][Part 2]: {}",
        day8::part2("../data/8/input.txt")
    );
}
