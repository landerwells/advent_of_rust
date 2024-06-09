use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/2018/day02.txt").expect("Failed to read input file");

    println!("{}", input);
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    // Iterate through every line
    let mut twos: usize = 0;
    let mut threes: usize = 0;
    for line in input.lines() {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in line.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        // Count the amount of lines that have exactly two of every letter
        if map.values().filter(|&&c| c == 2).count() > 0 {
            twos += 1;
        }
        // Count the amount of lines that have exactly three of every letter
        if map.values().filter(|&&c| c == 3).count() > 0 {
            threes += 1;
        }
    }
    twos * threes
}

fn part_two(input: &str) -> String {
    // Need to go through every line and print the correct letters of the almost

    for s in input.lines() {
        for t in input.lines() {
            let tuple = almost_matching(s, t);
            if tuple.0 {
                return tuple.1;
            }
        }
    }

    "".to_string()
}

fn almost_matching(s: &str, t: &str) -> (bool, String) {
    let mut ans = "".to_string();
    for (i, j) in s.chars().zip(t.chars()) {
        if i == j {
            ans += &i.to_string();
        }
    }
    if ans.len() == s.len() - 1 {
        return (true, ans);
    }

    (false, ans)
}
