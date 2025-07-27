use advent_of_rust::utils::*;

fn main() {
    let input = parse(get_input(2024, 7));

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn parse(input: String) -> Vec<Vec<i64>> {
    input
        .replace(':', "")
        .lines() // better than .split('\n')
        .filter(|line| !line.trim().is_empty()) // ⬅ filter out blank lines
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
    .collect()
}

// somehow I broke this during the part two, but they both produce correct answers??
fn part_one(input: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for line in input {
        let target = line[0];
        let array = &line[1..];

        if is_valid(target, array, false) {
            sum += target;
        }
    }

    sum
}

fn is_valid(target: i64, array: &[i64], part_two: bool) -> bool {
    if array.len() == 1 {
        return target == array[0];
    }

    let sum = array[0] + array[1];
    let product = array[0] * array[1];
    let concat = format!("{}{}", array[0], array[1])
        .parse::<i64>()
        .unwrap();

    if array.len() == 2 {
        return target == product || target == sum || (part_two && target == concat);
    }

    let add = target >= sum;
    let mut addition = false;
    if add {
        let mut result = vec![sum];
        result.extend_from_slice(&array[2..]);
        addition = is_valid(target, &result, part_two);
    }

    let multiply = target >= product;
    let mut multiplication = false;
    if multiply {
        let mut result = vec![product];
        result.extend_from_slice(&array[2..]);
        multiplication = is_valid(target, &result, part_two);
    }

    let concatenate = target >= concat;
    let mut concatenation = false;
    if concatenate && part_two {
        let mut result = vec![concat];
        result.extend_from_slice(&array[2..]);
        concatenation = is_valid(target, &result, part_two);
    }

    addition || multiplication || concatenation
}

fn part_two(input: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for line in input {
        let target = line[0];
        let array = &line[1..];

        if is_valid(target, array, true) {
            sum += target;
        }
    }

    sum
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {
//         let input = r#"189: 10 19
//             3267: 81 40 27
//             83: 17 5
//             156: 15 6
//             7290: 6 8 6 15
//             161011: 16 10 13
//             192: 17 8 14
//             21037: 9 7 18 13
//             292: 11 6 16 20
//             "#
//             .to_string();
//
//         let input = parse(input);
//
//         assert!(part_one(&input) == 3749);
//     }
//
//     #[test]
//     fn test_is_valid() {
//         assert!(is_valid(190, &[19, 10], false));
//         assert!(is_valid(19, &[9, 10], false));
//         assert!(is_valid(292, &[11, 6, 16, 20], false));
//         assert!(is_valid(292, &[17, 16, 20], false));
//         assert!(is_valid(156, &[15, 6], true));
//         assert!(!is_valid(292, &[82, 20], false));
//     }
//
//
//     #[test]
//     fn test_part_two() {
//         let input = r#"190: 10 19
//             3267: 81 40 27
//             83: 17 5
//             156: 15 6
//             7290: 6 8 6 15
//             161011: 16 10 13
//             192: 17 8 14
//             21037: 9 7 18 13
//             292: 11 6 16 20
//             "#
//             .to_string();
//
//         let input = parse(input);
//
//         assert!(part_two(&input) == 11387);
//     }
// }
