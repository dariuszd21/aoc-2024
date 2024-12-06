#[path = "utils.rs"]
mod utils;

use std::cmp;

enum Operation {
    Multiply,
}

fn load_lines(filepath: &str) -> Vec<Vec<char>> {
    let lines = utils::read_file(filepath);
    let mut matrix = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        matrix.push(line.chars().collect::<Vec<_>>());
    }
    matrix
}

fn get_right_diagonal(lines: &Vec<Vec<char>>, start_i: usize, start_j: usize) -> String {
    let (mut new_i, mut new_j) = (start_i, start_j);

    let mut chars_vec = Vec::new();
    while new_i < lines.len() && new_j < lines[0].len() {
        chars_vec.push(lines[new_i][new_j]);
        new_i += 1;
        new_j += 1;
    }

    String::from_iter(chars_vec)
}

fn get_left_diagonal(lines: &Vec<Vec<char>>, start_i: usize, start_j: usize) -> String {
    let (mut new_i, mut new_j) = (start_i, start_j);

    let mut chars_vec = Vec::new();
    while new_i < lines.len() && new_j < lines[0].len() {
        chars_vec.push(lines[new_i][new_j]);
        if new_j == 0 {
            break;
        }
        new_i += 1;
        new_j -= 1;
    }

    String::from_iter(chars_vec)
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let word_to_find = "XMAS".to_string();
    let rev_word_to_find = String::from_iter(word_to_find.chars().rev());

    let number_of_lines = lines.len();
    let number_of_cols = lines[0].len();

    for i in 0..number_of_lines {
        let line_to_analyse: String = String::from_iter(lines[i].clone());
        // XMAS matches
        res += line_to_analyse.match_indices(&word_to_find).count() as u64;
        // SAMX matches
        res += line_to_analyse.match_indices(&rev_word_to_find).count() as u64;
        //
        // diagonals

        if i > 0 {
            let right_diagonal = get_right_diagonal(&lines, i, 0);
            res += right_diagonal.match_indices(&word_to_find).count() as u64;
            res += right_diagonal.match_indices(&rev_word_to_find).count() as u64;

            let left_diagonal = get_left_diagonal(&lines, i, number_of_cols - 1);
            res += left_diagonal.match_indices(&word_to_find).count() as u64;
            res += left_diagonal.match_indices(&rev_word_to_find).count() as u64;
        }
    }

    for j in 0..number_of_cols {
        // let forward_vec = Vec::new();
        let mut col_to_analyse = Vec::new();
        for i in 0..number_of_lines {
            col_to_analyse.push(lines[i][j]);
        }
        let column = String::from_iter(col_to_analyse);
        // X
        // M
        // A
        // S
        res += column.match_indices(&word_to_find).count() as u64;

        // S
        // A
        // M
        // X
        // matches
        res += column.match_indices(&rev_word_to_find).count() as u64;

        // diagonals
        let right_diagonal = get_right_diagonal(&lines, 0, j);
        res += right_diagonal.match_indices(&word_to_find).count() as u64;
        res += right_diagonal.match_indices(&rev_word_to_find).count() as u64;

        let left_diagonal = get_left_diagonal(&lines, 0, j);
        res += left_diagonal.match_indices(&word_to_find).count() as u64;
        res += left_diagonal.match_indices(&rev_word_to_find).count() as u64;
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
        assert_eq!(solve_part_1("input_04_test"), 18);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_04_test"), 9);
    }
}
