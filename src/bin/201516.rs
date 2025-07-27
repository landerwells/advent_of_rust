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
        println!("{line}");

        let mut map = HashMap::new();


        if let Some(caps) = re.captures(line) {
            println!("{caps:?}");
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
    let list = "children: 3 cats: 7 samoyeds: 2 pomeranians: 3 akitas: 0 vizslas: 0 goldfish: 5 trees: 3 cars: 2 perfumes: 1".to_string();
    let list = parse(list);

    println!("{list:?}");
     
    // Parse these additional lines
    


    // for sue in input.iter() {
    //
    //     println!("{sue:?}");
    // }

    // for (i, line) in input.lines().enumerate() {
    //     let mut count = 0;
    //
    //     for item in list.lines() {
    //         if line.contains(item.trim()) {
    //             count += 1;
    //         }
    //     }
    //
    //     if count == 3 {
    //         return (i + 1) as i32; // Sue numbers are usually 1-based
    //     }
    // }

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
