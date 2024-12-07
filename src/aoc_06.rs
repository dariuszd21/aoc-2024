#[path = "utils.rs"]
mod utils;

use std::collections::HashSet;

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Clone)]
enum Tile {
    Visited(Vec<Direction>),
    Empty,
    Obstacle,
    Guard,
}

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn load_map(lines: &Vec<String>) -> Vec<Vec<Tile>> {
    let mut map = Vec::new();

    for line in lines {
        if !line.is_empty() {
            let mut tile_row = Vec::new();

            for char in line.chars() {
                match char {
                    '#' => tile_row.push(Tile::Obstacle),
                    '.' => tile_row.push(Tile::Empty),
                    '^' => tile_row.push(Tile::Guard),
                    _ => (),
                };
            }

            map.push(tile_row);
        }
    }

    map
}

fn find_guard(map: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
    for (i, tile_row) in map.iter().enumerate() {
        for (j, tile) in tile_row.iter().enumerate() {
            match tile {
                Tile::Guard => {
                    return Some((i, j));
                }
                _ => (),
            }
        }
    }

    None
}

fn can_move(
    guard_i: usize,
    guard_j: usize,
    map_x: usize,
    map_y: usize,
    guard_direction: &Direction,
) -> bool {
    match guard_direction {
        Direction::North => guard_i > 0,
        Direction::West => guard_j > 0,
        Direction::East => guard_j < map_y - 1,
        Direction::South => guard_i < map_x - 1,
    }
}

fn rotate_guard(guard_direction: Direction) -> Direction {
    match guard_direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
        Direction::South => Direction::West,
    }
}

fn simulate_movement(map: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut explored_map = map.clone();

    let num_rows = map.len();
    let num_cols = map[0].len();
    println!("Map size {} x {}", num_rows, num_cols);

    if let Some((guard_i, guard_j)) = find_guard(&map) {
        let mut map = map.clone();
        map[guard_i][guard_j] = Tile::Empty;
        let mut guard_direction = Direction::North;
        let (mut guard_pos_i, mut guard_pos_j) = (guard_i, guard_j);

        while can_move(
            guard_pos_i,
            guard_pos_j,
            num_rows,
            num_cols,
            &guard_direction,
        ) {
            explored_map[guard_pos_i][guard_pos_j] = Tile::Visited(vec![guard_direction.clone()]);
            match guard_direction {
                Direction::North => {
                    let new_guard_i = guard_pos_i - 1;
                    match map[new_guard_i][guard_pos_j] {
                        Tile::Empty => {
                            guard_pos_i = new_guard_i;
                        }
                        Tile::Obstacle => guard_direction = rotate_guard(guard_direction),
                        _ => (),
                    }
                }
                Direction::East => {
                    let new_guard_j = guard_pos_j + 1;
                    match map[guard_pos_i][new_guard_j] {
                        Tile::Empty => {
                            guard_pos_j = new_guard_j;
                        }
                        Tile::Obstacle => guard_direction = rotate_guard(guard_direction),
                        _ => (),
                    }
                }
                Direction::West => {
                    let new_guard_j = guard_pos_j - 1;
                    match map[guard_pos_i][new_guard_j] {
                        Tile::Empty => {
                            guard_pos_j = new_guard_j;
                        }
                        Tile::Obstacle => guard_direction = rotate_guard(guard_direction),
                        _ => (),
                    }
                }
                Direction::South => {
                    let new_guard_i = guard_pos_i + 1;
                    match map[new_guard_i][guard_pos_j] {
                        Tile::Empty => {
                            guard_pos_i = new_guard_i;
                        }
                        Tile::Obstacle => guard_direction = rotate_guard(guard_direction),
                        _ => (),
                    }
                }
            }
        }
        explored_map[guard_pos_i][guard_pos_j] = Tile::Visited(vec![guard_direction.clone()]);
    }

    explored_map
}
fn detect_loops(map: &Vec<Vec<Tile>>) -> bool {
    let mut explored_map = map.clone();

    let num_rows = map.len();
    let num_cols = map[0].len();

    if let Some((guard_i, guard_j)) = find_guard(&map) {
        explored_map[guard_i][guard_j] = Tile::Empty;
        let mut guard_direction = Direction::North;
        let (mut guard_pos_i, mut guard_pos_j) = (guard_i, guard_j);

        while can_move(
            guard_pos_i,
            guard_pos_j,
            num_rows,
            num_cols,
            &guard_direction,
        ) {
            match &mut explored_map[guard_pos_i][guard_pos_j] {
                Tile::Visited(vec) => vec.push(guard_direction.clone()),
                Tile::Empty => {
                    explored_map[guard_pos_i][guard_pos_j] =
                        Tile::Visited(vec![guard_direction.clone()]);
                }
                Tile::Obstacle => todo!(),
                Tile::Guard => todo!(),
            }
            match guard_direction {
                Direction::North => {
                    let new_guard_i = guard_pos_i - 1;
                    match &explored_map[new_guard_i][guard_pos_j] {
                        Tile::Empty => {
                            guard_pos_i = new_guard_i;
                        }
                        Tile::Obstacle => guard_direction = rotate_guard(guard_direction),
                        Tile::Visited(directions) => {
                            if directions.contains(&guard_direction) {
                                return true;
                            }
                            guard_pos_i = new_guard_i;
                        }
                        _ => (),
                    }
                }
                Direction::East => {
                    let new_guard_j = guard_pos_j + 1;
                    match &explored_map[guard_pos_i][new_guard_j] {
                        Tile::Empty => {
                            guard_pos_j = new_guard_j;
                        }
                        Tile::Obstacle => guard_direction = rotate_guard(guard_direction),
                        Tile::Visited(directions) => {
                            if directions.contains(&guard_direction) {
                                return true;
                            }
                            guard_pos_j = new_guard_j;
                        }
                        _ => (),
                    }
                }
                Direction::West => {
                    let new_guard_j = guard_pos_j - 1;
                    match &explored_map[guard_pos_i][new_guard_j] {
                        Tile::Empty => {
                            guard_pos_j = new_guard_j;
                        }
                        Tile::Obstacle => guard_direction = rotate_guard(guard_direction),
                        Tile::Visited(directions) => {
                            if directions.contains(&guard_direction) {
                                return true;
                            }
                            guard_pos_j = new_guard_j;
                        }
                        _ => (),
                    }
                }
                Direction::South => {
                    let new_guard_i = guard_pos_i + 1;
                    match &explored_map[new_guard_i][guard_pos_j] {
                        Tile::Empty => {
                            guard_pos_i = new_guard_i;
                        }
                        Tile::Obstacle => guard_direction = rotate_guard(guard_direction),
                        Tile::Visited(directions) => {
                            if directions.contains(&guard_direction) {
                                return true;
                            }
                            guard_pos_i = new_guard_i;
                        }
                        _ => (),
                    }
                }
            }

            // we are back at start
            if (guard_i == guard_pos_i)
                && (guard_j == guard_pos_j)
                && guard_direction == Direction::North
            {
                return true;
            }
        }
        explored_map[guard_pos_i][guard_pos_j] = Tile::Visited(vec![guard_direction.clone()]);
    }

    false
}

fn find_loops(map: &Vec<Vec<Tile>>) -> u64 {
    let mut loops = 0;

    let num_rows = map.len();
    let num_cols = map[0].len();
    println!("Map size {} x {}", num_rows, num_cols);

    let mut explored_map = map.clone();
    explored_map[9][7] = Tile::Obstacle;
    let res = detect_loops(&explored_map);
    println!("Result {}", res);
    // assert!(detect_loops(&explored_map));

    for i in 0..num_rows {
        for j in 0..num_cols {
            let mut explored_map = map.clone();
            match &explored_map[i][j] {
                Tile::Empty => {
                    explored_map[i][j] = Tile::Obstacle;
                    if detect_loops(&explored_map) {
                        println!("Detected loop if inserted at {} {}", i, j);
                        loops += 1;
                    }
                }

                _ => (),
            }
        }
    }
    loops
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let map = load_map(&lines);

    let explored_map = simulate_movement(&map);

    for tile_row in explored_map {
        for tile in tile_row {
            res += match tile {
                Tile::Visited(_) => 1,
                _ => 0,
            };
        }
    }

    res
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);

    let map = load_map(&lines);

    find_loops(&map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part_1("input_06_test"), 41);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_06_test"), 6);
    }
}
