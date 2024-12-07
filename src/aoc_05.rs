#[path = "utils.rs"]
mod utils;

use std::collections::{HashMap, HashSet};

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn load_rules(lines: &Vec<String>) -> HashMap<u64, HashSet<u64>> {
    let mut map: HashMap<u64, HashSet<u64>> = HashMap::new();

    for line in lines {
        if line.contains("|") {
            let splitted: Vec<&str> = line.split("|").collect();
            if splitted.len() == 2 {
                let (first, second) = (splitted[0], splitted[1]);
                let first_num: u64 = first.parse().unwrap();
                let second_num: u64 = second.parse().unwrap();

                match map.get_mut(&first_num) {
                    Some(vec) => {
                        vec.insert(second_num);
                    }
                    None => {
                        let mut hash = HashSet::new();
                        hash.insert(second_num);
                        map.insert(first_num, hash);
                    }
                };
            }
        }
    }

    map
}
fn load_print_orders(lines: &Vec<String>) -> Vec<Vec<u64>> {
    let mut prints = Vec::new();

    for line in lines {
        if line.contains(",") {
            let mut print_order = Vec::new();
            for val in line.split(",") {
                print_order.push(val.parse().unwrap());
            }
            prints.push(print_order);
        }
    }

    prints
}

fn check_print_order(print_order: &Vec<u64>, rules: &HashMap<u64, HashSet<u64>>) -> bool {
    for i in 0..print_order.len() {
        let val = print_order[i];

        for prev in 0..i {
            let prev_val = print_order[prev];
            if let Some(ruleset) = rules.get(&val) {
                if ruleset.contains(&prev_val) {
                    println!("Rule not followed: {} before {}", prev_val, val);
                    return false;
                }
            }
        }
    }

    true
}

fn fix_ordering(print_order: &Vec<u64>, rules: &HashMap<u64, HashSet<u64>>) -> Vec<u64> {
    let mut correct_vec = print_order.clone();

    while !check_print_order(&correct_vec, rules) {
        let curr_vec = correct_vec.clone();
        println!("Current vec: {:?}", correct_vec);
        for i in 0..curr_vec.len() {
            let val = curr_vec[i];
            let mut stop = false;

            for prev in 0..i {
                let prev_val = curr_vec[prev];
                if let Some(ruleset) = rules.get(&val) {
                    if ruleset.contains(&prev_val) {
                        correct_vec.remove(i);
                        correct_vec.insert(prev, val);
                        stop = true;
                        break;
                    }
                }
            }
            if stop {
                break;
            }
        }
    }

    correct_vec
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let rules = load_rules(&lines);
    let print_orders = load_print_orders(&lines);

    println!("Rules: {:?}", rules);
    println!("Print orders: {:?}", print_orders);

    for print_order in print_orders {
        if check_print_order(&print_order, &rules) {
            let val = print_order.len() / 2;
            res += print_order[val];
        }
    }

    res
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let rules = load_rules(&lines);
    let print_orders = load_print_orders(&lines);

    println!("Rules: {:?}", rules);
    println!("Print orders: {:?}", print_orders);

    for print_order in print_orders {
        if !check_print_order(&print_order, &rules) {
            let new_values = fix_ordering(&print_order, &rules);
            let val = new_values[new_values.len() / 2];
            res += val;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part_1("input_05_test"), 143);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_05_test"), 123);
    }
}
