#[path = "utils.rs"]
mod utils;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapScale {
    Height(u8),
}

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn load_map(lines: &Vec<String>) -> Vec<Vec<MapScale>> {
    let mut map = Vec::new();

    for line in lines {
        if !line.is_empty() {
            let mut map_row = Vec::new();
            for c in line.chars() {
                match c {
                    '0'..='9' => map_row.push(MapScale::Height(c.to_string().parse().unwrap())),
                    _ => (),
                }
            }
            map.push(map_row);
        }
    }

    map
}

fn find_starting_points(map: &Vec<Vec<MapScale>>) -> Vec<(usize, usize)> {
    let mut start_points: Vec<(usize, usize)> = Vec::new();

    for (i, map_row) in map.iter().enumerate() {
        for (j, map_tile) in map_row.iter().enumerate() {
            match map_tile {
                MapScale::Height(h) => {
                    if *h == 0 {
                        start_points.push((i, j));
                    }
                }
            }
        }
    }
    start_points
}

fn calculate_trailhead_score(
    map: &Vec<Vec<MapScale>>,
    x: usize,
    y: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> u64 {
    let mut score = 0;
    let current_map_item = map[x][y];
    let current_height = match current_map_item {
        MapScale::Height(h) => h,
    };

    if visited.contains(&(x, y)) {
        return score;
    }
    visited.insert((x, y));

    if current_map_item == MapScale::Height(9) {
        return 1;
    } else {
        let map_height = map.len();
        let map_width = map[0].len();
        let next_height = MapScale::Height(current_height + 1);
        if x > 0 && map[x - 1][y] == next_height {
            score += calculate_trailhead_score(map, x - 1, y, visited);
        }
        if y > 0 && map[x][y - 1] == next_height {
            score += calculate_trailhead_score(map, x, y - 1, visited);
        }
        if x < map_height - 1 && map[x + 1][y] == next_height {
            score += calculate_trailhead_score(map, x + 1, y, visited);
        }

        if y < map_width - 1 && map[x][y + 1] == next_height {
            score += calculate_trailhead_score(map, x, y + 1, visited);
        }
    }

    score
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);

    let map = load_map(&lines);
    let mut res = 0;

    let start_points = find_starting_points(&map);

    for (start_x, start_y) in start_points {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let current_score = calculate_trailhead_score(&map, start_x, start_y, &mut visited);
        res += current_score;
    }

    res
}

fn calculate_trailhead_rating(map: &Vec<Vec<MapScale>>, x: usize, y: usize) -> u64 {
    let mut score = 0;
    let current_map_item = map[x][y];
    let current_height = match current_map_item {
        MapScale::Height(h) => h,
    };

    if current_map_item == MapScale::Height(9) {
        return 1;
    } else {
        let map_height = map.len();
        let map_width = map[0].len();
        let next_height = MapScale::Height(current_height + 1);
        if x > 0 && map[x - 1][y] == next_height {
            score += calculate_trailhead_rating(map, x - 1, y);
        }
        if y > 0 && map[x][y - 1] == next_height {
            score += calculate_trailhead_rating(map, x, y - 1);
        }
        if x < map_height - 1 && map[x + 1][y] == next_height {
            score += calculate_trailhead_rating(map, x + 1, y);
        }

        if y < map_width - 1 && map[x][y + 1] == next_height {
            score += calculate_trailhead_rating(map, x, y + 1);
        }
    }

    score
}
pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);

    let map = load_map(&lines);

    let mut res = 0;
    let start_points = find_starting_points(&map);

    for (start_x, start_y) in start_points {
        let current_score = calculate_trailhead_rating(&map, start_x, start_y);
        res += current_score;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part_1("input_10_test"), 36);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_10_test"), 81);
    }
}
