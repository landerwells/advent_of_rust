use std::collections::HashSet;

use advent_of_rust::utils::*;

fn main() {
    let input = parse(get_input(2024, 6));

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&mut input.clone()));
}

fn parse(input: String) -> Vec<Vec<char>> {
    parse_grid(input) // Return by value
}

fn part_one(input: &Vec<Vec<char>>) -> i32 {
    let mut x_pos: i32 = 0;
    let mut y_pos: i32 = 0;

    for (i, row) in input.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            x_pos = j as i32;
            y_pos = i as i32;
            break;
        }
    }

    solve(&input, x_pos, y_pos).unwrap()
}

fn in_bounds(x: i32, y: i32, grid: &Vec<Vec<char>>) -> bool {
    y >= 0 && (y as usize) < grid.len() && x >= 0 && (x as usize) < grid[0].len()
}

fn solve(grid: &Vec<Vec<char>>, mut x_pos: i32, mut y_pos: i32) -> Option<i32> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_w_dir: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    let directions: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut direction_index = 0;

    while in_bounds(x_pos, y_pos, grid) {
        visited.insert((x_pos, y_pos));

        let (dx, dy) = directions[direction_index];

        if visited_w_dir.contains(&(x_pos, y_pos, dx, dy)) {
            return None;
        } else {
            visited_w_dir.insert((x_pos, y_pos, dx, dy));
        }

        let next_x: i32 = x_pos + dx;
        let next_y: i32 = y_pos + dy;

        if 0 <= next_x && next_x < grid[0].len() as i32 && 0 <= next_y && next_y < grid.len() as i32
        {
            if grid[(y_pos + dy) as usize][(x_pos + dx) as usize] == '#' {
                direction_index = (direction_index + 1) % 4;
            } else {
                y_pos = next_y;
                x_pos = next_x;
            }
        } else {
            break;
        }
    }

    Some(visited.len() as i32)
}

fn part_two(input: &mut Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let mut x_pos: i32 = 0;
    let mut y_pos: i32 = 0;

    for (i, row) in input.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            x_pos = j as i32;
            y_pos = i as i32;
            break;
        }
    }

    for i in 0..input.len() {
        for j in 0..input.len() {
            if input[i][j] == '.' {
                let old = input[i][j];
                input[i][j] = '#';

                if solve(&input, x_pos, y_pos).is_none() {
                    count += 1;
                }

                input[i][j] = old;
            }
        }
    }

    count
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_part_one() {}

//     #[test]
//     fn test_part_two() {}
// }
