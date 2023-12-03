mod day1;
mod day2;
mod day3;

#[macro_use]
extern crate scan_fmt;
#[macro_use]
extern crate anyhow;

pub fn main() {
    println!(
        "Solution [Day 3][Part 1]: {}",
        day3::part1("../data/3/input.txt")
    );

    println!(
        "Solution [Day 3][Part 2]: {}",
        day3::part2("../data/3/input.txt")
    );
}
