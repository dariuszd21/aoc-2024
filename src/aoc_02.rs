#[path = "utils.rs"]
mod utils;

fn load_levels(filepath: &str) -> Vec<Vec<u64>> {
    let lines = utils::read_file(filepath);
    let mut levels = Vec::new();
    for line in lines {
        let mut single_vec = Vec::new();
        for value in line.split(" ") {
            let num_str = value.trim();
            if !num_str.is_empty() {
                single_vec.push(num_str.parse().unwrap());
            }
        }
        if !single_vec.is_empty() {
            levels.push(single_vec);
        }
    }
    levels
}

fn check_diff(first: u64, second: u64) -> bool {
    let diff = first - second;
    return diff > 3;
}

fn is_safe(levels_vec: &Vec<u64>) -> bool {
    let num_of_elems = levels_vec.len();
    if num_of_elems > 1 {
        let (first, second) = (levels_vec[0], levels_vec[1]);

        // ascending
        if second > first {
            for i in 0..num_of_elems - 1 {
                let (curr, next) = (levels_vec[i], levels_vec[i + 1]);
                if curr >= next {
                    return false;
                }
                if check_diff(next, curr) {
                    return false;
                }
            }
        } else if second == first {
            return false;
        } else {
            for i in 0..num_of_elems - 1 {
                let (curr, next) = (levels_vec[i], levels_vec[i + 1]);
                if next >= curr {
                    return false;
                }
                if check_diff(curr, next) {
                    return false;
                }
            }
        }
    }
    true
}

pub fn solve_part_1(filepath: &str) -> usize {
    let levels = load_levels(filepath);

    levels
        .iter()
        .filter_map(|x| if is_safe(&x) { Some(x) } else { None })
        .count()
}

pub fn solve_part_2(filepath: &str) -> usize {
    let levels = load_levels(filepath);

    let mut res = 0;

    for levels_vec in levels {
        if is_safe(&levels_vec) {
            res += 1;
            continue;
        }

        for i in 0..levels_vec.len() {
            let mut permutated_leves = levels_vec.clone();
            permutated_leves.remove(i);
            if is_safe(&permutated_leves) {
                res += 1;
                break;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part_1("input_02_test"), 2);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_02_test"), 4);
    }
}
