use std::collections::HashSet;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/2018/day01.txt").expect("Failed to read input file");

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    make_vec(input).iter().sum()
}

fn make_vec(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.to_string().replace('+', "").parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn part_two(input: &str) -> i32 {
    let mut current_frequency = 0;
    let mut previous_frequencies: HashSet<i32> = HashSet::new();
    let mut i = 0;
    let vec = make_vec(input);
    loop {
        if previous_frequencies.contains(&current_frequency) {
            return current_frequency;
        }

        previous_frequencies.insert(current_frequency);
        current_frequency += vec[i];
        i = (i + 1) % vec.len();
    }
}
