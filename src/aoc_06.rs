#[path = "utils.rs"]
mod utils;

enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Clone)]
enum Tile {
    Visited,
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
        Direction::East => guard_j > 0,
        Direction::West => guard_j < map_y - 1,
        Direction::South => guard_i < map_x - 1,
    }
}

fn simulate_movement(map: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut explored_map = map.clone();

    let num_rows = map.len();
    let num_cols = map[0].len();
    println!("Map size {} x {}", num_rows, num_cols);

    if let Some((guard_i, guard_j)) = find_guard(&map) {
        let mut map = map.clone();
        let mut guard_direction = Direction::North;
        let (mut guard_pos_i, mut guard_pos_j) = (guard_i, guard_j);

        map[guard_i][guard_j] = Tile::Empty;
        while can_move(
            guard_pos_i,
            guard_pos_j,
            num_rows,
            num_cols,
            &guard_direction,
        ) {
            explored_map[guard_pos_i][guard_pos_j] = Tile::Visited;
            match guard_direction {
                Direction::North => {
                    let new_guard_i = guard_pos_i - 1;
                    match map[new_guard_i][guard_pos_j] {
                        Tile::Empty => {
                            guard_pos_i = new_guard_i;
                        }
                        Tile::Obstacle => guard_direction = Direction::West,
                        _ => (),
                    }
                }
                Direction::East => {
                    let new_guard_j = guard_pos_j - 1;
                    match map[guard_pos_i][new_guard_j] {
                        Tile::Empty => {
                            guard_pos_j = new_guard_j;
                        }
                        Tile::Obstacle => guard_direction = Direction::North,
                        _ => (),
                    }
                }
                Direction::West => {
                    let new_guard_j = guard_pos_j + 1;
                    match map[guard_pos_i][new_guard_j] {
                        Tile::Empty => {
                            guard_pos_j = new_guard_j;
                        }
                        Tile::Obstacle => guard_direction = Direction::South,
                        _ => (),
                    }
                }
                Direction::South => {
                    let new_guard_i = guard_pos_i + 1;
                    match map[new_guard_i][guard_pos_j] {
                        Tile::Empty => {
                            guard_pos_i = new_guard_i;
                        }
                        Tile::Obstacle => guard_direction = Direction::East,
                        _ => (),
                    }
                }
            }
        }
        explored_map[guard_pos_i][guard_pos_j] = Tile::Visited;
    }

    explored_map
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let map = load_map(&lines);

    let explored_map = simulate_movement(&map);

    for tile_row in explored_map {
        for tile in tile_row {
            res += match tile {
                Tile::Visited => 1,
                _ => 0,
            };
        }
    }

    res
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let map = load_map(&lines);

    res
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
        assert_eq!(solve_part_2("input_06_test"), 123);
    }
}
