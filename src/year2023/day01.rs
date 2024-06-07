use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/2023/day01.txt").expect("Failed to read input file");

    // println!("{}", input);
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut sum = 0;
    let lines = input.lines();
    for line in lines {
        let vec = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        sum += vec.first().unwrap() * 10 + vec.last().unwrap();
    }
    sum as i32
}

fn part_two(input: &str) -> i32 {
    0
}
