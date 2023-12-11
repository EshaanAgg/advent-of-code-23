mod day11;

pub fn main() {
    println!(
        "Solution [Day 11][Part 1]: {}",
        day11::part1("../data/11/input.txt")
    );

    println!(
        "Solution [Day 11][Part 2]: {}",
        day11::part2("../data/11/input.txt", 1000000)
    );
}
