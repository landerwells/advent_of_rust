use std::vec;

use advent_of_rust::utils::*;

fn main() {
    let input = parse(get_input(2015, 2));

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn parse(input: String) -> Vec<Vec<i32>> {
    input
        .split_whitespace()
        .map(|line| {
            line.split('x')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn part_one(input: &Vec<Vec<i32>>) -> i32 {
    let mut ans: i32 = 0;

    for line in input {
        let length = line[0];
        let width = line[1];
        let height = line[2];

        let surface_area: Vec<i32> = vec![length * width, width * height, height * length];
        let min_side: i32 = *surface_area.iter().min().unwrap();

        ans = ans + min_side + surface_area.iter().map(|x| x * 2).sum::<i32>();
    }

    ans
}

fn part_two(_input: &Vec<Vec<i32>>) -> i32 {
    0
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_part_one() {}

//     #[test]
//     fn test_part_two() {}
// }
