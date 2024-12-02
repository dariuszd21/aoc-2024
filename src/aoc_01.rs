#[path = "utils.rs"]
mod utils;

pub fn solve_part_1(filepath: &str) -> i64 {
    let lines = utils::read_file(filepath);
    let mut left_vec: Vec<i64> = Vec::new();
    let mut right_vec: Vec<i64> = Vec::new();
    for line in lines {
        let splitted: Vec<_> = line.split("   ").collect();
        if splitted.len() == 2 {
            let (left, right) = (splitted[0], splitted[1]);
            left_vec.push(left.parse().unwrap());
            right_vec.push(right.parse().unwrap());
        }
    }
    left_vec.sort();
    right_vec.sort();

    std::iter::zip(left_vec, right_vec)
        .map(|(l, r)| (r - l).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        assert_eq!(solve_part_1("input_01_test"), 11);
    }
}
