#[path = "utils.rs"]
mod utils;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Stone {
    value: u64,
}

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn get_parsed_stones(
    cache: &mut HashMap<u64, (Stone, Stone)>,
    stone_val: u64,
) -> Option<(Stone, Stone)> {
    match cache.get(&stone_val) {
        Some(value) => Some(*value),
        None => {
            let stones_str = stone_val.to_string();
            let stones_len = stones_str.len();
            if stones_len % 2 == 0 {
                let new_stones = (
                    Stone {
                        value: stones_str[0..stones_len / 2].to_string().parse().unwrap(),
                    },
                    Stone {
                        value: stones_str[stones_len / 2..stones_len]
                            .to_string()
                            .parse()
                            .unwrap(),
                    },
                );
                cache.insert(stone_val, new_stones);
                Some(new_stones)
            } else {
                None
            }
        }
    }
}

fn process_stones(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();
    let mut cache = HashMap::new();

    for (stone, amount) in stones {
        if *stone == 0 {
            insert_stone(&mut new_stones, 1, *amount);
            continue;
        }
        if let Some(parsed_stones) = get_parsed_stones(&mut cache, *stone) {
            insert_stone(&mut new_stones, parsed_stones.0.value, *amount);
            insert_stone(&mut new_stones, parsed_stones.1.value, *amount);
            continue;
        }
        insert_stone(&mut new_stones, stone * 2024, *amount);
    }

    new_stones
}

fn load_stones_to_map(lines: &Vec<String>) -> HashMap<u64, u64> {
    let mut stones = HashMap::new();

    for line in lines {
        if !line.is_empty() {
            for stone in line.split(" ") {
                insert_stone(&mut stones, stone.parse().unwrap(), 1);
            }
        }
    }

    stones
}

fn insert_stone(stones: &mut HashMap<u64, u64>, stone_val: u64, stone_amount: u64) {
    match stones.get(&stone_val) {
        Some(amount) => {
            stones.insert(stone_val, amount + stone_amount);
        }
        None => {
            stones.insert(stone_val, stone_amount);
        }
    }
}

fn transform_stones(stones: &HashMap<u64, u64>, epochs: usize) -> HashMap<u64, u64> {
    let mut new_stones = stones.clone();
    for _ in 0..epochs {
        new_stones = process_stones(&new_stones);
    }
    new_stones
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let stones = load_stones_to_map(&lines);

    let epochs = 25;
    let result_stones = transform_stones(&stones, epochs);

    result_stones.values().sum()
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let stones = load_stones_to_map(&lines);

    let epochs = 75;
    let result_stones = transform_stones(&stones, epochs);

    result_stones.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let lines = load_lines("input_11_test");

        let mut stones = load_stones_to_map(&lines);

        stones = process_stones(&stones);

        assert_eq!(stones.values().sum::<u64>(), 7);
    }

    #[test]
    fn test_example_another_example() {
        let stones_loaded_from_file = vec!["125 17".to_string()];

        let stones = load_stones_to_map(&stones_loaded_from_file);

        let new_stones = transform_stones(&stones, 25);
        assert_eq!(new_stones.values().sum::<u64>(), 55312);
    }
}
