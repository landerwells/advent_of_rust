use std::fs::{File, create_dir_all};
use std::io::{self, Read, Write};
use std::path::Path;
// use std::{fs, result};

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

// I would like a command for generating a new day, and potentially getting the
// input as well. I suppose that getting the input should occur when I am
// creating a new day as I can run it right away

// I should start by coming up with a template for the files that I want created

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

fn download_input(year: i32, day: i32) {
    println!("Downloading input from interwebs")
}

pub fn generate_day(year: i32, day: i32) {
    // This function needs to create a file in bin/YEAR/DAY format
    let path_str = format!("./src/bin/{}{:02}.rs", year, day);

    let contents = r#"use crate::utils;

fn main() {
    // get input

    // parse
    let input = parse(input);

    // solve part 1 or 2
    // need to implement some logic here
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn parse(input: String) -> String {
    input
}

fn part_one(input: String) -> i32 {
    0
}

fn part_two(input: String) -> i32 {
    0
}
"#;

    let _ = create_file_with_dirs(&path_str, contents);

    // I am quite sure we could achieve a high functioning system
    // With really good errors
    println!("Generating day");
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
