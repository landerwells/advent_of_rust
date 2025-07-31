use advent_of_rust::utils::*;
use regex::Regex;
use std::collections::HashMap;


fn main() {
    let input = parse(get_input(2015, 16));

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn parse(input: String) -> Vec<HashMap<String, i32>> {
    let mut vec: Vec<HashMap<String, i32>> = Vec::new();
    let re = Regex::new(r"(\w+): (\d+)").unwrap();

    for line in input.lines() {
        // println!("{line}");

        let mut map = HashMap::new();


        if let Some(caps) = re.captures(line) {
            // println!("{caps:?}");
            for i in (1..caps.len()).step_by(2) {
                let item = caps[i].to_string();
                let amount: i32 = caps[i + 1].parse().unwrap();
                map.insert(item, amount);
            }
        } else {
            println!("No match");
        }

        vec.push(map);
    }

    vec
}

fn part_one(input: &[HashMap<String, i32>]) -> i32 {
    let list: HashMap<String, i32> = HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizlas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);

    for (i, sue) in input.iter().enumerate() {
        let mut count = 0;

        for (key, value) in &list {
            if sue.get(key) == Some(value) {
                count += 1;
                println!("Sue {}: {key} = {value}", i + 1);
            }
        }
        // println!("Sue {}: {count}", i + 1);

        if count == 3 {
            return (i + 1) as i32; // Sue numbers are usually 1-based
        }
    }

    -1 // Not found
}

fn part_two(_input: &[HashMap<String, i32>]) -> i32 {
    // looking for greater than cats and trees, and less than pomeranians and goldfish
    //
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
