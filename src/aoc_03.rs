#[path = "utils.rs"]
mod utils;

use regex::Regex;

enum Operation {
    Multiply,
}

fn find_values(s: &str) -> Vec<(u64, u64)> {
    let mut values = Vec::new();
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for re_match in mul_re.captures_iter(s) {
        let (_, [first, second]) = re_match.extract();
        values.push((first.parse().unwrap(), second.parse().unwrap()));
    }

    values
}

fn load_operations(filepath: &str) -> Vec<(Operation, u64, u64)> {
    let lines = utils::read_file(filepath);
    let mut operations = Vec::new();

    for line in lines {
        for (first, second) in find_values(&line) {
            operations.push((Operation::Multiply, first, second));
        }
    }

    operations
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let operations = load_operations(filepath);
    let mut res = 0;

    for (_op, first, second) in operations {
        println!("{} * {}", first, second);
        res += first * second;
    }

    res
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let operations = load_operations(filepath);
    let mut res = 0;

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part_1("input_03_test"), 161);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_03_test"), 48);
    }
}
