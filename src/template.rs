use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/YEAR/dayDAY.txt")
        .expect("Failed to read input file");

    println!("{}", input);
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(_input: &str) -> i32 {
    0
}

fn part_two(_input: &str) -> i32 {
    0
}
