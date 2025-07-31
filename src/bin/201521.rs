use std::cmp;

use advent_of_rust::utils::*;

struct Boss {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

struct Shop {
    weapons: Vec<Item>,
    armors: Vec<Item>, 
    rings: Vec<Item>
}

struct Player {
    weapon: Item,
    armor: Option<Item>,
    ring_1: Option<Item>,
    ring_2: Option<Item>,
}

#[derive(Debug, Clone, PartialEq)]
struct Item {
    kind: ItemKind,
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum ItemKind {
    Weapon,
    Armor,
    Ring,
}

fn main() {
    let boss_data: Boss = parse_boss(get_input(2015, 21));
    let shop_data: Shop = parse_shop();

    println!("Part 1: {}", part_one(&boss_data, &shop_data));
    println!("Part 2: {}", part_two(&boss_data, &shop_data));
}

fn parse_boss(input: String) -> Boss {
    Boss {
        hit_points: input.lines().next().unwrap().split_whitespace().nth(2).unwrap().parse().unwrap(),
        damage: input.lines().nth(1).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap(),
        armor: input.lines().nth(2).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap(),
    }
}

fn parse_shop() -> Shop {
    Shop {
        weapons: vec![
            Item { kind: ItemKind::Weapon, name: "Dagger".to_string(), cost: 8, damage: 4, armor: 0 },
            Item { kind: ItemKind::Weapon, name: "Shortsword".to_string(), cost: 10, damage: 5, armor: 0 },
            Item { kind: ItemKind::Weapon, name: "Warhammer".to_string(), cost: 25, damage: 6, armor: 0 },
            Item { kind: ItemKind::Weapon, name: "Longsword".to_string(), cost: 40, damage: 7, armor: 0 },
            Item { kind: ItemKind::Weapon, name: "Greataxe".to_string(), cost: 74, damage: 8, armor: 0 },
        ],
        armors: vec![
            Item { kind: ItemKind::Armor, name: "Leather".to_string(), cost: 13, damage: 0, armor: 1 },
            Item { kind: ItemKind::Armor, name: "Chainmail".to_string(), cost: 31, damage: 0, armor: 2 },
            Item { kind: ItemKind::Armor, name: "Splintmail".to_string(), cost: 53, damage: 0, armor: 3 },
            Item { kind: ItemKind::Armor, name: "Bandedmail".to_string(), cost: 75, damage: 0, armor: 4 },
            Item { kind: ItemKind::Armor, name: "Platemail".to_string(), cost: 102, damage: 0, armor: 5 },
        ],
        rings: vec![
            Item { kind: ItemKind::Ring, name: "Damage +1".to_string(), cost: 25, damage: 1, armor: 0 },
            Item { kind: ItemKind::Ring, name: "Damage +2".to_string(), cost: 50, damage: 2, armor: 0 },
            Item { kind: ItemKind::Ring, name: "Damage +3".to_string(), cost: 100, damage: 3, armor: 0 },
            Item { kind: ItemKind::Ring, name: "Defense +1".to_string(), cost: 20, damage: 0, armor: 1 },
            Item { kind: ItemKind::Ring, name: "Defense +2".to_string(), cost: 40, damage: 0, armor: 2 },
            Item { kind: ItemKind::Ring, name: "Defense +3".to_string(), cost: 80, damage: 0, armor: 3 },
        ],
    }
}

fn part_one(boss_data: &Boss, shop_data: &Shop) -> i32 {
    // Construct all possible players that and determine the minimum of who will win
    let mut min = 1000;

    for weapon in &shop_data.weapons {
        for armor in shop_data.armors.iter().chain(std::iter::once(&Item { kind: ItemKind::Armor, name: "None".to_string(), cost: 0, damage: 0, armor: 0 })) {
            for ring_1 in shop_data.rings.iter().chain(std::iter::once(&Item { kind: ItemKind::Ring, name: "None".to_string(), cost: 0, damage: 0, armor: 0 })) {
                for ring_2 in shop_data.rings.iter().filter(|r| r != &ring_1).chain(std::iter::once(&Item { kind: ItemKind::Ring, name: "None".to_string(), cost: 0, damage: 0, armor: 0 })) {
                    let player = Player {
                        weapon: weapon.clone(),
                        armor: Some(armor.clone()),
                        ring_1: if ring_1.name == "None" { None } else { Some(ring_1.clone()) },
                        ring_2: if ring_2.name == "None" { None } else { Some(ring_2.clone()) },
                    };

                    if simulate_battle(&player, boss_data) {
                        min = cmp::min(min, player_cost(&player))
                    }
                }
            }
        }
    }

    min
}

// I think that simulate battle can be simplified to just a few checks
fn simulate_battle(player: &Player, boss: &Boss) -> bool {
    let mut player_hit_points = 100;
    let mut boss_hit_points = boss.hit_points;

    while player_hit_points > 0 && boss_hit_points > 0 {
        // Player attacks first
        boss_hit_points -= (player.weapon.damage + player.ring_1.as_ref().map_or(0, |r| r.damage) + player.ring_2.as_ref().map_or(0, |r| r.damage)) - boss.armor;
        if boss_hit_points <= 0 {
            return true; // Player wins
        }

        // Boss attacks
        player_hit_points -= boss.damage - player.armor.as_ref().map_or(0, |a| a.armor);
    }

    false // Boss wins
}

fn player_cost(player: &Player) -> i32 {
    let mut cost = player.weapon.cost;
    if let Some(armor) = &player.armor {
        cost += armor.cost;
    }
    if let Some(ring_1) = &player.ring_1 {
        cost += ring_1.cost;
    }
    if let Some(ring_2) = &player.ring_2 {
        cost += ring_2.cost;
    }
    cost
}

fn part_two(boss_data: &Boss, shop_data: &Shop) -> i32 {
    let mut max = 0;

    let none_armor = Item { kind: ItemKind::Armor, name: "None".to_string(), cost: 0, damage: 0, armor: 0 };
    let none_ring = Item { kind: ItemKind::Ring, name: "None".to_string(), cost: 0, damage: 0, armor: 0 };

    for weapon in &shop_data.weapons {
        for armor in shop_data.armors.iter().chain(std::iter::once(&none_armor)) {
            for ring_1 in shop_data.rings.iter().chain(std::iter::once(&none_ring)) {
                for ring_2 in shop_data.rings.iter()
                    .chain(std::iter::once(&none_ring))
                    .filter(|r| r.name != ring_1.name)
                {
                    let player = Player {
                        weapon: weapon.clone(),
                        armor: if armor.name == "None" { None } else { Some(armor.clone()) },
                        ring_1: if ring_1.name == "None" { None } else { Some(ring_1.clone()) },
                        ring_2: if ring_2.name == "None" { None } else { Some(ring_2.clone()) },
                    };

                    if !simulate_battle(&player, boss_data) {
                        max = cmp::max(max, player_cost(&player));
                        println!("{}")
                    }
                }
            }
        }
    }

    max
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {}
//
//     #[test]
//     fn test_part_two() {}
// }
