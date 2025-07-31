use std::collections::{HashMap, HashSet};
use regex::Regex;

use advent_of_rust::utils::*;

fn main() {
    let (replacements, reverse_replacements, molecule) = parse(get_input(2015, 19));

    println!("Part 1: {}", part_one(&replacements, &molecule));
    println!("Part 2: {}", part_two(&reverse_replacements, &molecule));
}

fn parse(input: String) -> (HashMap<String, Vec<String>>, HashMap<String, String>, String) {
    let mut lines = input.split("\n\n");

    let replacements_section = lines.next().unwrap();
    let molecule = lines.next().unwrap().trim().to_string();

    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();

    for line in replacements_section.lines() {
        let (from, to) = line.split_once(" => ").unwrap();
        if replacements.contains_key(from) {
            replacements.get_mut(from).unwrap().push(to.to_string());
        } else {
            replacements.insert(from.to_string(), vec![to.to_string()]);
        }
    }

    let mut reverse_replacements: HashMap<String, String> = HashMap::new();

    for line in replacements_section.lines() {
        let (from, to) = line.split_once(" => ").unwrap();
        reverse_replacements.insert(to.to_string(), from.to_string());
    }

    (replacements, reverse_replacements, molecule)
}

fn part_one(replacements: &HashMap<String, Vec<String>>, molecule: &String) -> i32 {
    let mut distinct_molecules: HashSet<String> = HashSet::new();
    for (key, value) in replacements {
        for item in value.iter() {
            let pattern = format!("({key})");
            let re = Regex::new(&pattern).unwrap();
            for mat in re.find_iter(molecule) {
                let mut temp_molecule = molecule.clone();
                temp_molecule.replace_range(mat.start()..mat.end(), item);
                distinct_molecules.insert(temp_molecule.clone());
            }

        }
    }
    distinct_molecules.len() as i32
}

// This is the naive approach to part two, I should have gone from the molecule to "e" instead.
// fn part_two(replacements: &HashMap<String, Vec<String>>, molecule: &String) -> i32 {
//     // Fewest steps to get to the molecule from "e"
//     // This seems like a breadth first search problem
//
//     // Tree so don't have to worry about revisiting nodes
//     let mut q: VecDeque<(String, i32)> = VecDeque::new();
//     q.push_back((String::from("e"), 0));
//
//     while let Some((current_molecule, steps)) = q.pop_front() {
//         if &current_molecule == molecule {
//             return steps;
//         }
//
//         // For the "e" example, iterate through all key value pairs, and make replacements,
//         // putting the new value into the queue.
//         for (key, value) in replacements {
//             if current_molecule.contains(key) {
//                 for item in value.iter() {
//                     let re = Regex::new(key).unwrap();
//                     for mat in re.find_iter(&current_molecule) {
//                         let mut temp_molecule = current_molecule.clone();
//                         temp_molecule.replace_range(mat.start()..mat.end(), item);
//                         q.push_back((temp_molecule.clone(), steps + 1));
//                     }
//                 }
//
//             }
//
//         }
//     }
//     -1 // If we reach here, we didn't find the molecule
// }

fn part_two(reverse_replacements: &HashMap<String, String>, molecule: &String) -> i32 {
    // Need to try going from the molecule to "e"
    // Find the keys with the longest length and try to replace them first
    let mut keys: Vec<String> = reverse_replacements.keys().cloned().collect();
    keys.sort_by(|a, b| b.len().cmp(&a.len())); // Sort by length descending
    let mut current_molecule = molecule.clone();

    // Need to calculate the steps to get to "e", which depends on the keys 
    // and their replacements
    let mut steps: i32 = 0;
    while current_molecule != "e" {
        // println!("Current molecule: {}", current_molecule);
        let mut found = false;

        for key in &keys {
            if let Some(pos) = current_molecule.find(key) {
                steps += 1;
                current_molecule.replace_range(pos..pos + key.len(), &reverse_replacements[key]);
                found = true;
                break; // Only replace one at a time
            }
        }

        if !found {
            break; // No more replacements possible
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;
    const HOH: &str = indoc! {r#"H => HO
        H => OH
        O => HH

        HOH"#};

    const HOHOHO: &str = indoc! {r#"H => HO
        H => OH
        O => HH

        HOHOHO"#};

    const HOH_2: &str = indoc! {r#"e => H
        e => O
        H => HO
        H => OH
        O => HH

        HOH"#};

    const HOHOHO_2: &str = indoc! {r#"e => H
        e => O
        H => HO
        H => OH
        O => HH

        HOHOHO"#};

    #[test]
    fn test_part_one() {
        let (replacements, _reverse_replacements, molecule) = parse(HOH.to_string());
        assert_eq!(part_one(&replacements, &molecule), 4);

        let (replacements, _reverse_replacements, molecule) = parse(HOHOHO.to_string());
        assert_eq!(part_one(&replacements, &molecule), 7);
    }

    #[test]
    fn test_part_two() {
        let (_replacements, reverse_replacements, molecule) = parse(HOH_2.to_string());
        assert_eq!(part_two(&reverse_replacements, &molecule), 3);

        let (_replacements, reverse_replacements, molecule) = parse(HOHOHO_2.to_string());
        assert_eq!(part_two(&reverse_replacements, &molecule), 6);

    }
}
