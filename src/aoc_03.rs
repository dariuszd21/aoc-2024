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
        res += first * second;
    }

    res
}

fn load_operations_part2(filepath: &str) -> Vec<(Operation, u64, u64)> {
    let lines = utils::read_file(filepath);
    let mut operations = Vec::new();

    let mut enabled = true;
    for line in lines {
        let mut curr_idx = 0;
        let mut curr_buff = line.clone();
        while curr_idx <= line.len() {
            curr_buff = curr_buff[curr_idx..].to_string();
            if enabled {
                let dont = curr_buff.find("don't()");
                match dont {
                    Some(idx) => {
                        if enabled {
                            let val = &curr_buff[..idx];
                            println!("{}", val);
                            for (first, second) in find_values(&curr_buff[..idx]) {
                                operations.push((Operation::Multiply, first, second));
                            }
                        }
                        enabled = false;
                        curr_idx = idx + 1;
                        continue;
                    }
                    None => {
                        if enabled {
                            for (first, second) in find_values(&curr_buff) {
                                operations.push((Operation::Multiply, first, second));
                            }
                            break;
                        }
                    }
                }
            } else {
                let do_match = curr_buff.find("do()");
                match do_match {
                    Some(idx) => {
                        let val = &curr_buff[idx..];
                        println!("{}", val);
                        enabled = true;
                        curr_idx = idx + 1;
                    }
                    None => break,
                }
            }
        }
    }

    operations
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let operations = load_operations_part2(filepath);
    let mut res = 0;

    for (_op, first, second) in operations {
        println!("{} * {}", first, second);
        res += first * second;
    }

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
