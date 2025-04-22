use advent_of_rust::utils::*;

fn main() {
    let input = get_input(2015, 1);

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .sum::<i32>()
}

// Realstically, this problem does not like funcitonal languages
fn part_two(input: &str) -> i32 {
    let mut floor = 0;
    input
        .chars()
        .enumerate()
        .map_while(|(i, c)| {
            floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            // Keep mapping until we hit -1
            if floor == -1 {
                None // stop here
            } else {
                Some(i + 1) // keep going
            }
        })
        .last()
        .map(|i| i + 1)
        .unwrap() as i32
}
