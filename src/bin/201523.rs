use advent_of_rust::utils::*;

fn main() {
    let input = parse(get_input(2015, 23));

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn parse(input: String) -> Vec<Vec<String>> {
    let input = input.replace(",", "");
    input.lines().map(|line| {
        line.split_whitespace().map(|s| s.to_string()).collect()
    }).collect()
}

fn part_one(input: &[Vec<String>]) -> i32 {
    let mut register_a = 1;
    let mut register_b = 0;
    // print number of lines
    // println!("Number of instructions: {}", input.len());
    
    let mut current_instruction: usize = 0;
    while current_instruction < input.len() {

        println!("Current instruction: {}", current_instruction);
        println!("Register A: {}, Register B: {}", register_a, register_b);
        match input[current_instruction][0].as_str() {
            "hlf" => {
                // Half the value of register a or b
                if input[current_instruction][1] == "a" {
                    hlf(&mut register_a);
                } else {
                    hlf(&mut register_b);
                }
                current_instruction += 1;
            },
            "tpl" => {
                println!("Tripling register {}", input[current_instruction][1]);
                if input[current_instruction][1] == "a" {
                    register_a *= 3;
                } else {
                    register_b *= 3;
                }
                current_instruction += 1;
            },
            "inc" => {
                println!("Incrementing register {}", input[current_instruction][1]);
                if input[current_instruction][1] == "a" {
                    register_a += 1;
                } else {
                    register_b += 1;
                }
                current_instruction += 1;
            },
            "jmp" => {
                println!("Jumping to instruction {}", input[current_instruction][1]);
                let mut jump = input[current_instruction][1].parse::<i32>().unwrap();
                if jump < 0 {
                    jump = jump.abs();
                    current_instruction = (current_instruction as i32 - jump) as usize;
                } else {
                    current_instruction += jump as usize;
                }
            },
            "jie" => {
                println!("Jump if even instruction: {}", input[current_instruction][1]);
                if input[current_instruction][1] == "a" && register_a % 2 == 0 {
                    current_instruction += input[current_instruction][2].parse::<usize>().unwrap();
                } else if input[current_instruction][1] == "b" && register_b % 2 == 0 {
                    current_instruction += input[current_instruction][2].parse::<usize>().unwrap();
                } else {
                    current_instruction += 1;
                }

            }
            "jio" => {
                println!("Jump if one instruction: {}", input[current_instruction][1]);
                if input[current_instruction][1] == "a" && register_a == 1 {
                    current_instruction += input[current_instruction][2].parse::<usize>().unwrap();
                } else if input[current_instruction][1] == "b" && register_b == 1 {
                    current_instruction += input[current_instruction][2].parse::<usize>().unwrap();
                } else {
                    current_instruction += 1;
                }
            }
            _ => {
                println!("Unknown instruction: {}", input[current_instruction][0]);
                break;
            }
        }
    }

    register_b
}

fn hlf(register: &mut i32) {
    *register /= 2;
}

fn part_two(_input: &Vec<Vec<String>>) -> i32 {
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
