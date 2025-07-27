use advent_of_rust::utils::*;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let mut input = parse(get_input(2015, 13));

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&mut input));
}

fn parse(input: String) -> HashMap<String, HashMap<String, i32>> {
    let mut people: HashMap<String, HashMap<String, i32>> = Default::default();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let person = words[0];

        if people.contains_key(person) {
            // person already exists, just add the happiness
            let happiness = words[3].parse::<i32>().unwrap();
            let neighbor = words[10].trim_end_matches('.').to_string();
            if words[2] == "lose" {
                people.get_mut(person).unwrap().insert(neighbor, -happiness);
            } else {
                people.get_mut(person).unwrap().insert(neighbor, happiness);
            }
        } else {
            // new person, create a new entry
            let happiness = words[3].parse::<i32>().unwrap();
            let neighbor = words[10].trim_end_matches('.').to_string();
            let mut map: HashMap<String, i32> = HashMap::new();
            if words[2] == "lose" {
                map.insert(neighbor, -happiness);
            } else {
                map.insert(neighbor, happiness);
            }
            people.insert(person.to_string(), map);
        }
    }
    
    people
}

fn part_one(people: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut max_happiness = i32::MIN;
    
    for perm in people.keys().permutations(people.len()) {
        let mut total_happiness = 0;

        for i in 0..perm.len() {
            let person = perm[i];
            let next_person = perm[(i + 1) % perm.len()]; // wrap around to the first person
            
            let next_person_happiness: i32 = *people.get(person).unwrap().get(next_person).unwrap();
            let person_happiness: i32 = *people.get(next_person).unwrap().get(person).unwrap();
        
            total_happiness += next_person_happiness + person_happiness;

        }

        if total_happiness > max_happiness {
            max_happiness = total_happiness;
        }
    }

    max_happiness
}

fn part_two(people: &mut HashMap<String, HashMap<String, i32>>) -> i32 {
    // Need to do two things for part two to calculate correctly. First is adding
    // an additional person, YOU in the people list with all zeros for the others
    
    for person in people.values_mut() {
        person.insert("You".to_string(), 0);
    }
    
    let mut map: HashMap<String, i32> = HashMap::new();
    for person in people.clone().into_keys() {
        map.insert(person, 0);
    }
    people.insert("You".to_string(), map);


    println!("{:?}", people);

    let mut max_happiness = i32::MIN;
    
    for perm in people.keys().permutations(people.len()) {
        let mut total_happiness = 0;

        for i in 0..perm.len() {
            let person = perm[i];
            let next_person = perm[(i + 1) % perm.len()]; // wrap around to the first person
            
            let next_person_happiness: i32 = *people.get(person).unwrap().get(next_person).unwrap();
            let person_happiness: i32 = *people.get(next_person).unwrap().get(person).unwrap();
        
            total_happiness += next_person_happiness + person_happiness;

        }

        if total_happiness > max_happiness {
            max_happiness = total_happiness;
        }
    }

    max_happiness
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
            Alice would lose 79 happiness units by sitting next to Carol.
            Alice would lose 2 happiness units by sitting next to David.
            Bob would gain 83 happiness units by sitting next to Alice.
            Bob would lose 7 happiness units by sitting next to Carol.
            Bob would lose 63 happiness units by sitting next to David.
            Carol would lose 62 happiness units by sitting next to Alice.
            Carol would gain 60 happiness units by sitting next to Bob.
            Carol would gain 55 happiness units by sitting next to David.
            David would gain 46 happiness units by sitting next to Alice.
            David would lose 7 happiness units by sitting next to Bob.
            David would gain 41 happiness units by sitting next to Carol."#;

        let input = parse(input.to_string());
        assert_eq!(part_one(&input), 330);
    }

    // #[test]
    // fn test_part_two() {}
}
