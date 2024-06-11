use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/2018/day04.txt").expect("Failed to read input file");

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut input: Vec<&str> = input.lines().collect();
    input.sort();

    println!("{:?}", input);
    0
}

fn part_two(input: &str) -> i32 {
    0
}
