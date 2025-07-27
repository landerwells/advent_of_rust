use advent_of_rust::utils::*;

fn main() {
    let input = parse(get_input(2024, 8));

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn parse(input: String) -> Vec<Vec<char>> {
    parse_grid(input)
}

fn part_one(_input: &Vec<Vec<char>>) -> i32 {
    0
}

fn part_two(_input: &Vec<Vec<char>>) -> i32 {
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
