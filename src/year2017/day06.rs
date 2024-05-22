use std::collections::HashSet;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/2017/day06.txt").expect("Failed to read input file");

    println!("{}", input);
    println!("Part One: {}", part_one(&input));
}

fn part_one(input: &str) -> i32 {
    let mut input: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut sum = 0;
    let mut set: HashSet<Vec<i32>> = HashSet::new();

    loop {
        if set.contains(&input) {
            part_two(&mut input);
            break;
        } else {
            set.insert(input.clone());
            sum += 1;
        }

        let mut max = -1;
        let mut counter = 0;

        for (i, num) in input.iter().enumerate() {
            if *num > max {
                counter = i;
                max = *num;
            }
        }

        input = distribute(&mut input, counter, max);
    }

    sum
}

fn distribute(input: &mut Vec<i32>, start: usize, amount: i32) -> Vec<i32> {
    input[start] = 0;
    let len = input.len();
    let mut current_position = start + 1;
    let mut remaining_amount = amount;

    while remaining_amount > 0 {
        input[current_position % len] += 1;
        current_position += 1;
        remaining_amount -= 1;
    }

    input.to_vec()
}

fn part_two(input: &mut Vec<i32>) {
    let mut sum = 0;
    let mut set: HashSet<Vec<i32>> = HashSet::new();

    loop {
        if set.contains(input) {
            break;
        } else {
            set.insert(input.clone());
            sum += 1;
        }

        let mut max = -1;
        let mut counter = 0;

        for (i, num) in input.iter().enumerate() {
            if *num > max {
                counter = i;
                max = *num;
            }
        }

        *input = distribute(input, counter, max);
    }
    println!("Part Two: {}", sum);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_distribute() {
        let mut input = vec![0, 2, 7, 0];
        assert_eq!(vec![2, 4, 1, 2], distribute(&mut input, 2, 7));
    }
}
