use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/2015/day18.txt").expect("Failed to read input file");

    // This day is basically the game of life
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let mut input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    for _i in 0..100 {
        input = iterate(&mut input);
    }

    input
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&ch| ch == '#')
        .count()
}

fn iterate(input: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_state: Vec<Vec<char>> = vec![vec!['.'; input.len()]; input[0].len()];
    let mut neighbors = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            neighbors = check_cells(i as i32, j as i32, input);
            if input[i][j] == '#' {
                if neighbors == 2 || neighbors == 3 {
                    new_state[i][j] = '#'
                }
            } else if neighbors == 3 {
                new_state[i][j] = '#'
            }
        }
    }

    new_state
}

fn check_cells(i: i32, j: i32, input: &mut Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for x in -1..2 {
        for y in -1..2 {
            if i + x >= 0
                && j + y >= 0
                && ((i + x) as usize) < input.len()
                && ((j + y) as usize) < input[0].len()
                && input[(i + x) as usize][(j + y) as usize] == '#'
                && (x, y) != (0, 0)
            {
                count += 1;
            }
        }
    }

    count
}

fn part_two(input: &str) -> usize {
    let mut input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let len = input.len();
    for _i in 0..100 {
        input[0][0] = '#';
        input[0][len - 1] = '#';
        input[len - 1][0] = '#';
        input[len - 1][len - 1] = '#';
        input = iterate(&mut input);
    }

    input[0][0] = '#';
    input[0][len - 1] = '#';
    input[len - 1][0] = '#';
    input[len - 1][len - 1] = '#';

    input
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&ch| ch == '#')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterate() {
        let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####.."
            .to_string();

        let mut input: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        input = iterate(&mut input);

        assert_eq!(
            input,
            vec![
                vec!['.', '.', '#', '#', '.', '.'],
                vec!['.', '.', '#', '#', '.', '#'],
                vec!['.', '.', '.', '#', '#', '.'],
                vec!['.', '.', '.', '.', '.', '.'],
                vec!['#', '.', '.', '.', '.', '.'],
                vec!['#', '.', '#', '#', '.', '.']
            ]
        );
    }
}
