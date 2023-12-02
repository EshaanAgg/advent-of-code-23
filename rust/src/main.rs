mod day1;
mod day2;

#[macro_use]
extern crate scan_fmt;
#[macro_use]
extern crate anyhow;

pub fn main() {
    // println!(
    //     "Solution [Day 1][Part 1]: {}",
    //     day1::part1("../data/1/input.txt")
    // );

    // println!(
    //     "Solution [Day 1][Part 2]: {}",
    //     day1::part2("../data/1/input.txt")
    // );

    println!(
        "Solution [Day 2][Part 1]: {}",
        day2::part1("../data/2/input.txt")
    );

    println!(
        "Solution [Day 2][Part 2]: {}",
        day2::part2("../data/2/input.txt")
    );
}
