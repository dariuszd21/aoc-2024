#[path = "utils.rs"]
mod utils;

use std::collections::{HashMap, HashSet};

enum Tile {
    Antena(char),
    Empty,
}

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn load_antenas(lines: &Vec<String>) -> Vec<Vec<Tile>> {
    let mut antennas = Vec::new();

    for line in lines {
        let mut tiles_row = Vec::new();
        if line.is_empty() {
            continue;
        }
        for c in line.chars() {
            tiles_row.push(match c {
                '.' => Tile::Empty,
                _ => Tile::Antena(c),
            });
        }
        antennas.push(tiles_row);
    }

    antennas
}

fn point_in_boundaries(x: i64, y: i64, max_x: usize, max_y: usize) -> bool {
    if (x < 0) || x >= max_x.try_into().unwrap() {
        return false;
    }
    if (y < 0) || y >= max_y.try_into().unwrap() {
        return false;
    }

    true
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let antenas = load_antenas(&lines);

    let mut antenas_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, antennas_row) in antenas.iter().enumerate() {
        for (j, antena) in antennas_row.iter().enumerate() {
            match antena {
                Tile::Antena(c) => {
                    match &mut antenas_locations.get_mut(c) {
                        Some(vec) => {
                            vec.push((i, j));
                        }
                        None => {
                            antenas_locations.insert(*c, vec![(i, j)]);
                        }
                    };
                }
                Tile::Empty => (),
            }
        }
    }

    let num_rows = antenas.len();
    let num_cols = antenas[0].len();

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (antena_type, locations) in &antenas_locations {
        println!("Analysing antenas {} {:?}", antena_type, locations);
        for (i, location) in locations.iter().enumerate() {
            let (current_antena_x, current_antena_y) = location;
            for next_antena in &locations[i + 1..locations.len()] {
                let (next_antena_x, next_antena_y) = next_antena;
                println!("Analysing antenas {:?}", next_antena);

                let antena_diff_x: i64 = (*current_antena_x as i64 - *next_antena_x as i64).abs();
                let antena_diff_y: i64 = (*next_antena_y as i64 - *current_antena_y as i64).abs();
                let next_antinode_y = if next_antena_y > current_antena_y {
                    *next_antena_y as i64 + antena_diff_y
                } else {
                    *next_antena_y as i64 - antena_diff_y
                };
                if point_in_boundaries(
                    *next_antena_x as i64 + antena_diff_x,
                    next_antinode_y,
                    num_rows,
                    num_cols,
                ) {
                    println!(
                        "Analysing antinode {} {:?}",
                        *next_antena_x as i64 + antena_diff_x,
                        next_antinode_y,
                    );
                    res += 1;
                    antinodes.insert((
                        next_antena_x + antena_diff_x as usize,
                        next_antinode_y as usize,
                    ));
                }
                let current_antinode_y = if next_antena_y > current_antena_y {
                    *current_antena_y as i64 - antena_diff_y
                } else {
                    *current_antena_y as i64 + antena_diff_y
                };
                if point_in_boundaries(
                    *current_antena_x as i64 - antena_diff_x,
                    current_antinode_y,
                    num_rows,
                    num_cols,
                ) {
                    println!(
                        "Analysing antinode {} {:?}",
                        *current_antena_x as i64 - antena_diff_x,
                        current_antinode_y,
                    );
                    antinodes.insert((
                        current_antena_x - antena_diff_x as usize,
                        current_antinode_y as usize,
                    ));
                    res += 1;
                }
            }
        }
    }

    println!("{:?}", antinodes);
    println!("{}", antinodes.len());
    for i in 0..num_rows {
        for j in 0..num_cols {
            match antinodes.contains(&(i, j)) {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!("");
    }

    antinodes.len() as u64
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
        assert_eq!(solve_part_1("input_08_test"), 14);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_08_test"), 11387);
    }
}
