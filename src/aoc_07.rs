#[path = "utils.rs"]
mod utils;

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn load_equation(lines: &Vec<String>) -> Vec<(u64, Vec<u64>)> {
    let mut map = Vec::new();

    for line in lines {
        if !line.is_empty() {
            let splitted: Vec<_> = line.split(":").collect();
            if splitted.len() == 2 {
                let mut equation = Vec::new();
                let result = splitted[0].parse().unwrap();
                for val in splitted[1].split(" ") {
                    if !val.is_empty() {
                        equation.push(val.parse().unwrap());
                    }
                }
                map.push((result, equation));
            }
        }
    }

    map
}

fn is_valid(equation: &(u64, Vec<u64>)) -> bool {
    let mut res = false;
    let (eq_res, values) = equation;

    let mut possible_res = Vec::new();

    possible_res.push(values[0]);

    for i in 1..values.len() {
        let res_copy = possible_res.clone();
        possible_res.clear();

        for val in res_copy {
            possible_res.push(val * values[i]);
            possible_res.push(val + values[i]);
        }
    }
    println!("Possible results: {:?}", possible_res);

    possible_res.contains(eq_res)
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let equations_vec = load_equation(&lines);

    for equation in equations_vec {
        if is_valid(&equation) {
            println!("Equation: {:?} is valid", equation);
            res += equation.0;
        }
    }
    res
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let equations_vec = load_equation(&lines);

    for equation in equations_vec {
        if is_valid(&equation) {
            println!("Equation: {:?} is valid", equation);
            res += equation.0;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part_1("input_07_test"), 3749);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_07_test"), 6);
    }
}
