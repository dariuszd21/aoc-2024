#[path = "utils.rs"]
mod utils;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Stone {
    value: u64,
}

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn load_stones(lines: &Vec<String>) -> Vec<Stone> {
    let mut stones = Vec::new();

    for line in lines {
        if !line.is_empty() {
            for stone in line.split(" ") {
                stones.push(Stone {
                    value: stone.parse().unwrap(),
                });
            }
        }
    }

    stones
}

fn process_stones(stones: &Vec<Stone>) -> Vec<Stone> {
    let mut new_stones = Vec::new();

    for stone in stones {
        if stone.value == 0 {
            new_stones.push(Stone { value: 1 });
            continue;
        }
        let stones_str = format!("{}", stone.value);
        let stones_len = stones_str.len();
        if stones_len % 2 == 0 {
            new_stones.push(Stone {
                value: stones_str[0..stones_len / 2].to_string().parse().unwrap(),
            });
            new_stones.push(Stone {
                value: stones_str[stones_len / 2..stones_len]
                    .to_string()
                    .parse()
                    .unwrap(),
            });
            continue;
        }

        new_stones.push(Stone {
            value: stone.value * 2024,
        });
    }

    new_stones
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);

    let mut stones = load_stones(&lines);

    for _ in 0..25 {
        stones = process_stones(&stones);
    }

    stones.len().try_into().unwrap()
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);

    let stones = load_stones(&lines);
    let mut res = 0;

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let lines = load_lines("input_11_test");

        let mut stones = load_stones(&lines);

        stones = process_stones(&stones);

        assert_eq!(stones.len(), 7);
    }

    #[test]
    fn test_example_another_example() {
        let stones_loaded_from_file = vec!["125 17".to_string()];

        let stones = load_stones(&stones_loaded_from_file);

        let mut new_stones = stones;
        for _ in 0..25 {
            new_stones = process_stones(&new_stones);
        }
        assert_eq!(new_stones.len(), 55312);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_11_test"), 81);
    }
}
