use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/2015/day07.txt").expect("Failed to read input file");
    let mut machine: HashMap<String, Vec<String>> = HashMap::new();
    let mut output: HashMap<String, u16> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split(" -> ").collect();
        let key = parts[1].to_string();
        let value = parts[0].split_whitespace().map(|s| s.to_string()).collect();
        machine.insert(key, value);
    }
    println!("{:?}", machine);

    let result_part1 = part_one(&machine, &mut output);
    println!("Part One: {}", result_part1);

    let mut machine_part2 = machine.clone();
    machine_part2.insert("b".to_string(), vec![result_part1.to_string()]);

    let result_part2 = part_two(&machine_part2, &mut output);
    println!("Part Two: {}", result_part2);
}

fn part_one(machine: &HashMap<String, Vec<String>>, output: &mut HashMap<String, u16>) -> u16 {
    foo("a", machine, output)
}

fn part_two(machine: &HashMap<String, Vec<String>>, output: &mut HashMap<String, u16>) -> u16 {
    foo("a", machine, output)
}

fn foo(
    gate: &str,
    machine: &HashMap<String, Vec<String>>,
    output: &mut HashMap<String, u16>,
) -> u16 {
    if let Ok(value) = gate.parse::<u16>() {
        return value;
    }

    if !output.contains_key(gate) {
        let operate = machine.get(gate).unwrap();
        let value = if operate.len() == 1 {
            foo(&operate[0], machine, output)
        } else {
            let z = &operate[operate.len() - 2];
            match z.as_str() {
                "RSHIFT" => foo(&operate[0], machine, output) >> foo(&operate[2], machine, output),
                "LSHIFT" => foo(&operate[0], machine, output) << foo(&operate[2], machine, output),
                "AND" => foo(&operate[0], machine, output) & foo(&operate[2], machine, output),
                "OR" => foo(&operate[0], machine, output) | foo(&operate[2], machine, output),
                "NOT" => !foo(&operate[1], machine, output),
                _ => panic!("Unknown operation"),
            }
        };
        output.insert(gate.to_string(), value);
    }
    output[gate]
}
