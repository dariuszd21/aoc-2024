#[path = "utils.rs"]
mod utils;

fn load_vectors(filepath: &str) -> (Vec<u64>, Vec<u64>) {
    let lines = utils::read_file(filepath);
    let mut left_vec: Vec<u64> = Vec::new();
    let mut right_vec: Vec<u64> = Vec::new();
    for line in lines {
        let splitted: Vec<_> = line.split("   ").collect();
        if splitted.len() == 2 {
            let (left, right) = (splitted[0], splitted[1]);
            left_vec.push(left.parse().unwrap());
            right_vec.push(right.parse().unwrap());
        }
    }
    (left_vec, right_vec)
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let (mut left_vec, mut right_vec) = load_vectors(filepath);
    left_vec.sort();
    right_vec.sort();

    std::iter::zip(left_vec, right_vec)
        // implement abs without using i64 to store
        .map(|(l, r)| if r > l { r - l } else { l - r })
        .sum()
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let (left_vec, right_vec) = load_vectors(filepath);
    let mut res = 0;

    for num in left_vec {
        res += num * (right_vec.iter().filter(|&v| *v == num).count() as u64);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part_1("input_01_test"), 11);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_01_test"), 31);
    }
}
