use std::fs;
use std::process::{Command, Stdio};

pub fn run() {
    let input = fs::read_to_string("inputs/2017/day07.txt").expect("Failed to read input file");

    part_one();
    println!("Part Two: {}", part_two(&input));
}

fn part_one() {
    // Define the command to be executed
    let command = "grep -oE '[a-z]+' inputs/2017/day07.txt | sort | uniq -u";

    // Execute the command using the shell
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    // Check if the command was successful
    if output.status.success() {
        // Convert the command output to a string
        let result = String::from_utf8_lossy(&output.stdout);
        println!("Part One: {}", result);
    } else {
        // Print the error if the command failed
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error:\n{}", error);
    }
}

fn part_two(input: &str) -> i32 {
    0
}
