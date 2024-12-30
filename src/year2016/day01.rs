use regex::Regex;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/2016/day01.txt").expect("Failed to read input file");

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let input: Vec<&str> = input.split(',').collect();
    let re = Regex::new(r"#(R|L)(\d*)").unwrap();

    for line in input {
        if let Some(caps) = re.captures(line) {
            let direction = caps[1].to_string();
            let distance = caps[2].parse::<i32>().unwrap();
            println!("{} {}", direction, distance);
        }
    }

    0
}

fn part_two(input: &str) -> i32 {
    0
}
