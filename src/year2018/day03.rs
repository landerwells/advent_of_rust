use crate::utils::Coordinate;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/2018/day03.txt").expect("Failed to read input file");

    println!("{}", input);
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str, map: &mut HashMap<Coordinate, i32>) -> i32 {
    let re: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            // Extract capture groups
            let x = caps[2].parse::<i32>().unwrap();
            let y = caps[3].parse::<i32>().unwrap();
            let width = caps[4].parse::<i32>().unwrap();
            let height = caps[5].parse::<i32>().unwrap();

            for i in 0..width {
                for j in 0..height {
                    let coord = (x + i, y + j);
                    *map.entry(coord).or_insert(0) += 1;
                }
            }
        }
    }

    map.values().filter(|&&c| c > 1).count() as i32
}

fn part_two(input: &str, map: &mut HashMap<Coordinate, i32>) -> i32 {
    let re: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            // Extract capture groups
            let id = caps[1].parse::<i32>().unwrap();
            let x = caps[2].parse::<i32>().unwrap();
            let y = caps[3].parse::<i32>().unwrap();
            let width = caps[4].parse::<i32>().unwrap();
            let height = caps[5].parse::<i32>().unwrap();
            let mut non_overlapping = true;

            for i in 0..width {
                for j in 0..height {
                    let coord = (x + i, y + j);
                    if *map.get_key_value(&coord).unwrap().1 != 1 {
                        non_overlapping = false;
                        break;
                    }
                }
            }

            if non_overlapping {
                return id;
            }
        }
    }

    0
}
