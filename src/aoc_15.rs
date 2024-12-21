#[path = "utils.rs"]
mod utils;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Robot,
    Wall,
    Empty,
    Box,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum RobotMoves {
    Up,
    Down,
    Left,
    Right,
}

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn load_elements(lines: Vec<String>) -> (Vec<Vec<Tile>>, Vec<RobotMoves>) {
    let mut tiles = Vec::new();
    let mut robot_moves = Vec::new();

    for line in lines {
        if line.starts_with("#") {
            let mut tiles_row = Vec::new();
            for char in line.chars() {
                match char {
                    '#' => tiles_row.push(Tile::Wall),
                    '.' => tiles_row.push(Tile::Empty),
                    '@' => tiles_row.push(Tile::Robot),
                    'O' => tiles_row.push(Tile::Box),
                    _ => (),
                }
            }
            tiles.push(tiles_row);
        } else if !line.is_empty() {
            for char in line.chars() {
                match char {
                    '^' => robot_moves.push(RobotMoves::Up),
                    'v' => robot_moves.push(RobotMoves::Down),
                    '>' => robot_moves.push(RobotMoves::Right),
                    '<' => robot_moves.push(RobotMoves::Left),
                    _ => (),
                }
            }
        }
    }

    tiles.reverse();
    (tiles, robot_moves)
}

fn print_tiles(tiles: &Vec<Vec<Tile>>) {
    for row in tiles.iter().rev() {
        for tile in row {
            match tile {
                Tile::Robot => print!("@"),
                Tile::Wall => print!("#"),
                Tile::Empty => print!("."),
                Tile::Box => print!("O"),
            }
        }
        println!("");
    }
}

fn find_robot(tiles: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
    for (i, tile_row) in tiles.iter().enumerate() {
        for (j, tile) in tile_row.iter().enumerate() {
            match tile {
                Tile::Robot => {
                    return Some((i, j));
                }
                _ => (),
            }
        }
    }

    None
}

fn perform_move(tiles: &Vec<Vec<Tile>>, robot_move: RobotMoves) -> Vec<Vec<Tile>> {
    let mut new_map = tiles.clone();

    let (robot_i, robot_j) = find_robot(tiles).unwrap();
    // println!("Current robot pos: {} {}", robot_i, robot_j);
    // println!("Trying to move {:?}", robot_move);

    let map_height = tiles.len();
    let map_width = tiles[0].len();

    match robot_move {
        RobotMoves::Up => {
            for i in robot_i + 1..map_height {
                match tiles[i][robot_j] {
                    Tile::Robot => (),
                    Tile::Wall => {
                        break;
                    }
                    Tile::Empty => {
                        for new_tile_i in (robot_i + 1..=i).rev() {
                            (
                                new_map[new_tile_i][robot_j],
                                new_map[new_tile_i - 1][robot_j],
                            ) = (
                                new_map[new_tile_i - 1][robot_j],
                                new_map[new_tile_i][robot_j],
                            );
                        }
                        break;
                    }
                    Tile::Box => (),
                }
            }
        }
        RobotMoves::Down => {
            for i in (0..robot_i).rev() {
                match tiles[i][robot_j] {
                    Tile::Robot => (),
                    Tile::Wall => {
                        break;
                    }
                    Tile::Empty => {
                        for new_tile_i in i..robot_i {
                            (
                                new_map[new_tile_i][robot_j],
                                new_map[new_tile_i + 1][robot_j],
                            ) = (
                                new_map[new_tile_i + 1][robot_j],
                                new_map[new_tile_i][robot_j],
                            );
                        }
                        break;
                    }
                    Tile::Box => (),
                }
            }
        }
        RobotMoves::Left => {
            for j in (0..robot_j).rev() {
                match tiles[robot_i][j] {
                    Tile::Robot => (),
                    Tile::Wall => {
                        break;
                    }
                    Tile::Empty => {
                        for new_tile_j in j..robot_j {
                            new_map[robot_i].swap(new_tile_j, new_tile_j + 1);
                        }
                        break;
                    }
                    Tile::Box => (),
                }
            }
        }
        RobotMoves::Right => {
            for j in (robot_j + 1)..map_width {
                match tiles[robot_i][j] {
                    Tile::Robot => (),
                    Tile::Wall => {
                        break;
                    }
                    Tile::Empty => {
                        for new_tile_j in (robot_j + 1..=j).rev() {
                            new_map[robot_i].swap(new_tile_j, new_tile_j - 1);
                        }
                        break;
                    }
                    Tile::Box => (),
                }
            }
        }
    }

    new_map
}

fn calculate_gps(tiles: &Vec<Vec<Tile>>) -> u64 {
    let mut res = 0;

    for (i, tile_row) in tiles.iter().rev().enumerate() {
        for (j, tile) in tile_row.iter().enumerate() {
            match tile {
                Tile::Box => {
                    res += 100 * i + j;
                }
                _ => (),
            };
        }
    }

    res.try_into().unwrap()
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);

    let (mut tiles, robot_moves) = load_elements(lines);

    for robot_move in robot_moves {
        tiles = perform_move(&tiles, robot_move);
    }

    print_tiles(&tiles);

    calculate_gps(&tiles)
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let (tiles, robot_moves) = load_elements(lines);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part_1("input_15_test"), 2028);
    }

    #[test]
    fn test_bigger_example_part1() {
        assert_eq!(solve_part_1("input_15_test_2"), 10092);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_15_test"), 12);
    }
}
