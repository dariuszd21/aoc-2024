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
                    println!("Rule not followed: {} before {}", prev, val);
                    return false;
                }
            }
        }
    }

    true
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
        assert_eq!(solve_part_2("input_05_test"), 9);
    }
}
