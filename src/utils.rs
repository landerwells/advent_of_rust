use std::fs::{File, create_dir_all};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn update_cookie() -> std::io::Result<()> {
    println!("Please enter the new value of the cookie:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("ERROR: unable to read user input");

    // I could do some validation on the session cookie in the future,
    // but for now I am feeling lazy. Just enter the correct one the first time.

    let home = std::env::var("HOME").unwrap();
    let mut file = File::create(home + "/.cache/advent_of_code_inputs/cookie")?;
    file.write_all(input.as_bytes())
}

pub fn get_input(year: i32, day: i32) -> String {
    let path = format!(
        "{}/.cache/advent_of_code_inputs/{}/{}",
        std::env::var("HOME").unwrap(),
        year,
        day
    );

    let path_obj = Path::new(&path);

    if !path_obj.exists() {
        download_input(year, day);
    }

    let mut input = String::new();
    File::open(&path)
        .expect("Failed to open input file")
        .read_to_string(&mut input)
        .expect("Failed to read input file");

    input
}

fn download_input(_year: i32, _day: i32) {
    println!("Downloading input from interwebs")
    // TODO: Finish
}

pub fn generate_day(year: i32, day: i32) {
    let path_str = format!("./src/bin/{}{:02}.rs", year, day);
    let path_obj = Path::new(&path_str);

    if path_obj.exists() {
        println!("File {}{}.rs already exists!", year, day);
        return;
    }

    let contents = format!(
        r#"use advent_of_rust::utils::*;

fn main() {{
    let input = parse(get_input({}, {}));

    println!("Part 1: {{}}", part_one(&input));
    println!("Part 2: {{}}", part_two(&input));
}}

fn parse(input: String) -> String {{
    input
}}

fn part_one(_input: &str) -> i32 {{
    0
}}

fn part_two(_input: &str) -> i32 {{
    0
}}

// #[cfg(test)]
// mod tests {{
//     use super::*;

//     #[test]
//     fn test_part_one() {{}}

//     #[test]
//     fn test_part_two() {{}}
// }}
"#,
        year, day
    );

    let _ = create_file_with_dirs(&path_str, &contents);
}

fn create_file_with_dirs(path: &str, contents: &str) -> io::Result<()> {
    let path = Path::new(path);

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

#[allow(dead_code)]
pub fn parse_grid(input: String) -> Vec<Vec<char>> {
    input
        .split_whitespace()
        .map(|line| line.chars().collect())
        .collect()
}
