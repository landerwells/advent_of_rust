use std::{fs, vec};

pub fn run() {
    let input = fs::read_to_string("inputs/2016/day02.txt").expect("Failed to read input file");

    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let input = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut code = vec![];

    for line in input {
        code.push(parse_instructions(line));
    }
    code.iter().fold(0, |acc, x| acc * 10 + x)
}

fn parse_instructions(line: Vec<char>) -> i32 {
    let mut keypad: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut index = 4 as usize;
    for c in line {
        match c {
            'U' => {
                if index > 2 {
                    index -= 3;
                }
            }
            'D' => {
                if index < 6 {
                    index += 3;
                }
            }
            'L' => {
                if index % 3 != 0 {
                    index -= 1;
                }
            }
            'R' => {
                if index % 3 != 2 {
                    index += 1;
                }
            }
            _ => {}
        }
    }
    keypad[index]
}

fn part_two(input: &str) -> i32 {
    0
}
