#[path = "utils.rs"]
mod utils;

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Robot,
    Wall,
    Empty,
    Box,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BoxSide {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileWide {
    Robot,
    Wall,
    Empty,
    Box(BoxSide),
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

fn find_robot_part2(tiles: &Vec<Vec<TileWide>>) -> Option<(usize, usize)> {
    for (i, tile_row) in tiles.iter().enumerate() {
        for (j, tile) in tile_row.iter().enumerate() {
            match tile {
                TileWide::Robot => {
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

fn load_elements_part_2(lines: Vec<String>) -> (Vec<Vec<TileWide>>, Vec<RobotMoves>) {
    let mut tiles = Vec::new();
    let mut robot_moves = Vec::new();

    for line in lines {
        if line.starts_with("#") {
            let mut tiles_row = Vec::new();
            for char in line.chars() {
                match char {
                    '#' => {
                        tiles_row.push(TileWide::Wall);
                        tiles_row.push(TileWide::Wall);
                    }
                    '.' => {
                        tiles_row.push(TileWide::Empty);
                        tiles_row.push(TileWide::Empty);
                    }
                    '@' => {
                        tiles_row.push(TileWide::Robot);
                        tiles_row.push(TileWide::Empty);
                    }
                    'O' => {
                        tiles_row.push(TileWide::Box(BoxSide::Left));
                        tiles_row.push(TileWide::Box(BoxSide::Right));
                    }
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

fn print_tiles_part2(tiles: &Vec<Vec<TileWide>>) {
    for row in tiles.iter().rev() {
        for tile in row {
            match tile {
                TileWide::Robot => print!("@"),
                TileWide::Wall => print!("#"),
                TileWide::Empty => print!("."),
                TileWide::Box(side) => {
                    match side {
                        BoxSide::Left => print!("["),
                        BoxSide::Right => print!("]"),
                    };
                }
            }
        }
        println!("");
    }
}

fn calculate_move_up(
    tiles: &Vec<Vec<TileWide>>,
    start_i: usize,
    j: usize,
) -> Option<(usize, usize)> {
    let map_height = tiles.len();
    for i in start_i + 1..map_height {
        match tiles[i][j] {
            TileWide::Robot => panic!("Robot shouldn't be on way {} {}", i, j),
            TileWide::Wall => {
                return None;
            }
            TileWide::Empty => {
                return Some((i, j));
            }
            TileWide::Box(_box_side) => (),
        }
    }
    None
}

fn find_blocks_to_move_up(
    tiles: &Vec<Vec<TileWide>>,
    start_i: usize,
    start_j: usize,
) -> BTreeMap<(usize, usize), Option<(usize, usize)>> {
    let mut blocks_to_move = BTreeMap::new();

    let mut blocks_to_analyse = Vec::new();
    blocks_to_analyse.push((start_i, start_j));
    while !blocks_to_analyse.is_empty() {
        let (current_i, current_j) = blocks_to_analyse.pop().unwrap();
        match blocks_to_move.get(&(current_i, current_j)) {
            Some(_) => continue,
            None => (),
        }
        let current_tile = tiles[current_i][current_j];
        match current_tile {
            TileWide::Box(current_side) => {
                match tiles[current_i - 1][current_j] {
                    TileWide::Box(previous_side) => {
                        if current_side == previous_side {
                            continue;
                        }
                    }
                    _ => (),
                };
            }
            _ => (),
        }
        match calculate_move_up(tiles, current_i, current_j) {
            Some((end_i, end_j)) => {
                blocks_to_move.insert((current_i, current_j), Some((end_i, end_j)));
                for i in current_i + 1..end_i {
                    match tiles[i][current_j] {
                        TileWide::Box(box_side) => {
                            match box_side {
                                BoxSide::Left => {
                                    blocks_to_analyse.push((i, current_j + 1));
                                }
                                BoxSide::Right => {
                                    blocks_to_analyse.push((i, current_j - 1));
                                }
                            };
                        }
                        _ => (),
                    }
                }
            }
            None => {
                // println!("Cannot move: {} {}", current_i, current_j);
                blocks_to_move.insert((current_i, current_j), None);
                break;
            }
        }
    }

    blocks_to_move
}

fn calculate_move_down(
    tiles: &Vec<Vec<TileWide>>,
    start_i: usize,
    j: usize,
) -> Option<(usize, usize)> {
    for i in (0..start_i).rev() {
        match tiles[i][j] {
            TileWide::Robot => panic!("Robot shouldn't be on way {} {}", i, j),
            TileWide::Wall => {
                return None;
            }
            TileWide::Empty => {
                return Some((i, j));
            }
            TileWide::Box(_box_side) => (),
        }
    }
    None
}

fn find_blocks_to_move_down(
    tiles: &Vec<Vec<TileWide>>,
    start_i: usize,
    start_j: usize,
) -> BTreeMap<(usize, usize), Option<(usize, usize)>> {
    let mut blocks_to_move = BTreeMap::new();

    let mut blocks_to_analyse = Vec::new();
    blocks_to_analyse.push((start_i, start_j));
    while !blocks_to_analyse.is_empty() {
        let (current_i, current_j) = blocks_to_analyse.pop().unwrap();
        match blocks_to_move.get(&(current_i, current_j)) {
            Some(_) => continue,
            None => (),
        }
        let current_tile = tiles[current_i][current_j];
        match current_tile {
            TileWide::Box(current_side) => {
                match tiles[current_i + 1][current_j] {
                    TileWide::Box(previous_side) => {
                        if current_side == previous_side {
                            continue;
                        }
                    }
                    _ => (),
                };
            }
            _ => (),
        }
        match calculate_move_down(tiles, current_i, current_j) {
            Some((end_i, end_j)) => {
                blocks_to_move.insert((current_i, current_j), Some((end_i, end_j)));
                for i in (end_i + 1..current_i).rev() {
                    match tiles[i][current_j] {
                        TileWide::Box(box_side) => {
                            match box_side {
                                BoxSide::Left => {
                                    blocks_to_analyse.push((i, current_j + 1));
                                }
                                BoxSide::Right => {
                                    blocks_to_analyse.push((i, current_j - 1));
                                }
                            };
                        }
                        _ => (),
                    };
                }
            }
            None => {
                // println!("Cannot move: {} {}", current_i, current_j);
                blocks_to_move.insert((current_i, current_j), None);
                break;
            }
        }
    }

    blocks_to_move
}

fn filter_overlaps_up(
    blocks_to_move: &BTreeMap<(usize, usize), Option<(usize, usize)>>,
) -> BTreeMap<(usize, usize), (usize, usize)> {
    let mut filtered_map = BTreeMap::new();
    for ((i, j), block) in blocks_to_move {
        match block {
            Some((target_i, target_j)) => {
                filtered_map.insert((*i, *j), (*target_i, *target_j));
            }
            None => (),
        }
    }

    let mut block_to_delete = Vec::new();
    for ((start_i, start_j), (end_i, _)) in &filtered_map {
        for ((another_start_i, another_start_j), (another_end_i, _)) in &filtered_map {
            if start_j == another_start_j && start_i != another_start_i {
                if another_end_i >= end_i && another_start_i <= start_i {
                    block_to_delete.push((*start_i, *start_j));
                }
            }
        }
    }

    for block in block_to_delete {
        filtered_map.remove(&block);
    }

    filtered_map
}

fn filter_overlaps_down(
    blocks_to_move: &BTreeMap<(usize, usize), Option<(usize, usize)>>,
) -> BTreeMap<(usize, usize), (usize, usize)> {
    let mut filtered_map = BTreeMap::new();
    for ((i, j), block) in blocks_to_move {
        match block {
            Some((target_i, target_j)) => {
                filtered_map.insert((*i, *j), (*target_i, *target_j));
            }
            None => (),
        }
    }

    let mut block_to_delete = Vec::new();
    for ((start_i, start_j), (end_i, _)) in &filtered_map {
        for ((another_start_i, another_start_j), (another_end_i, _)) in &filtered_map {
            if start_j == another_start_j && start_i != another_start_i {
                if another_end_i <= end_i && another_start_i >= start_i {
                    block_to_delete.push((*start_i, *start_j));
                }
            }
        }
    }

    for block in block_to_delete {
        filtered_map.remove(&block).unwrap();
    }

    filtered_map
}

fn perform_move_part2(tiles: &Vec<Vec<TileWide>>, robot_move: RobotMoves) -> Vec<Vec<TileWide>> {
    let mut new_map = tiles.clone();

    let (robot_i, robot_j) = find_robot_part2(tiles).unwrap();
    // println!("Current robot pos: {} {}", robot_i, robot_j);
    // println!("Trying to move {:?}", robot_move);

    let map_width = tiles[0].len();

    match robot_move {
        RobotMoves::Up => {
            match tiles[robot_i + 1][robot_j] {
                TileWide::Robot => (),
                TileWide::Wall => (),
                TileWide::Empty => {
                    // easy case just move
                    (new_map[robot_i][robot_j], new_map[robot_i + 1][robot_j]) =
                        (new_map[robot_i + 1][robot_j], new_map[robot_i][robot_j]);
                }
                TileWide::Box(box_side) => {
                    // can move -> gather all the blocks to move
                    // if side is the same do nothing
                    // if sides are opossite calculate also
                    //  for a "new" block to be moved
                    let mut blocks_to_move = find_blocks_to_move_up(&tiles, robot_i + 1, robot_j);
                    let block_to_move_other_side = if box_side == BoxSide::Left {
                        find_blocks_to_move_up(&tiles, robot_i + 1, robot_j + 1)
                    } else {
                        find_blocks_to_move_up(&tiles, robot_i + 1, robot_j - 1)
                    };
                    blocks_to_move.extend(block_to_move_other_side);
                    for ((_block_i, _block_j), target_block) in &blocks_to_move {
                        match target_block {
                            Some((_, _)) => (),
                            None => {
                                // println!("Cannot move: {:?}", (block_i, block_j));
                                return tiles.to_vec();
                            }
                        }
                    }
                    let blocks_to_move = filter_overlaps_up(&blocks_to_move);
                    for ((block_i, block_j), (target_i, _target_j)) in &blocks_to_move {
                        for i in (*block_i + 1..=*target_i).rev() {
                            (new_map[i][*block_j], new_map[i - 1][*block_j]) =
                                (new_map[i - 1][*block_j], new_map[i][*block_j]);
                        }
                    }
                    (new_map[robot_i][robot_j], new_map[robot_i + 1][robot_j]) =
                        (new_map[robot_i + 1][robot_j], new_map[robot_i][robot_j]);
                }
            }
        }
        RobotMoves::Down => {
            match tiles[robot_i - 1][robot_j] {
                TileWide::Robot => (),
                TileWide::Wall => (),
                TileWide::Empty => {
                    (new_map[robot_i][robot_j], new_map[robot_i - 1][robot_j]) =
                        (new_map[robot_i - 1][robot_j], new_map[robot_i][robot_j]);
                }
                TileWide::Box(box_side) => {
                    // can move -> gather all the blocks to move
                    // if side is the same do nothing
                    // if sides are opossite calculate also
                    //  for a "new" block to be moved
                    let mut blocks_to_move = find_blocks_to_move_down(&tiles, robot_i - 1, robot_j);
                    let block_to_move_other_side = match box_side {
                        BoxSide::Left => find_blocks_to_move_down(&tiles, robot_i - 1, robot_j + 1),
                        BoxSide::Right => {
                            find_blocks_to_move_down(&tiles, robot_i - 1, robot_j - 1)
                        }
                    };
                    blocks_to_move.extend(block_to_move_other_side);
                    for ((_block_i, _block_j), target_block) in &blocks_to_move {
                        match target_block {
                            Some((_, _)) => (),
                            None => {
                                // println!("Cannot move: {:?}", (block_i, block_j));
                                return tiles.to_vec();
                            }
                        }
                    }
                    let blocks_to_move = filter_overlaps_down(&blocks_to_move);
                    for ((block_i, block_j), (target_i, _target_j)) in &blocks_to_move {
                        for i in (*target_i + 1)..=*block_i {
                            (new_map[i][*block_j], new_map[i - 1][*block_j]) =
                                (new_map[i - 1][*block_j], new_map[i][*block_j]);
                        }
                    }
                    (new_map[robot_i][robot_j], new_map[robot_i - 1][robot_j]) =
                        (new_map[robot_i - 1][robot_j], new_map[robot_i][robot_j]);
                }
            }
        }
        RobotMoves::Left => {
            for j in (0..robot_j).rev() {
                match tiles[robot_i][j] {
                    TileWide::Robot => (),
                    TileWide::Wall => {
                        break;
                    }
                    TileWide::Empty => {
                        for new_tile_j in j..robot_j {
                            new_map[robot_i].swap(new_tile_j, new_tile_j + 1);
                        }
                        break;
                    }
                    // don't need to be updated on l-r move
                    TileWide::Box(_) => (),
                }
            }
        }
        RobotMoves::Right => {
            for j in (robot_j + 1)..map_width {
                match tiles[robot_i][j] {
                    TileWide::Robot => (),
                    TileWide::Wall => {
                        break;
                    }
                    TileWide::Empty => {
                        for new_tile_j in (robot_j + 1..=j).rev() {
                            new_map[robot_i].swap(new_tile_j, new_tile_j - 1);
                        }
                        break;
                    }
                    // don't need to be updated on l-r move
                    TileWide::Box(_) => (),
                }
            }
        }
    }

    new_map
}

fn calculate_gps_part_2(tiles: &Vec<Vec<TileWide>>) -> u64 {
    let mut res = 0;

    for (i, tile_row) in tiles.iter().rev().enumerate() {
        for (j, tile) in tile_row.iter().enumerate() {
            match tile {
                TileWide::Box(box_side) => {
                    match box_side {
                        BoxSide::Left => {
                            res += 100 * i + j;
                        }
                        BoxSide::Right => (),
                    };
                }
                _ => (),
            };
        }
    }

    res.try_into().unwrap()
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);

    let (mut tiles, robot_moves) = load_elements_part_2(lines);

    println!("\nInitial state\n");
    print_tiles_part2(&tiles);
    for (_i, robot_move) in robot_moves.iter().enumerate() {
        tiles = perform_move_part2(&tiles, *robot_move);

        // println!("\nMove {} {:?}:\n", _i, robot_move);
        // print_tiles_part2(&tiles);
    }

    println!("\nFinal state:\n");
    print_tiles_part2(&tiles);

    calculate_gps_part_2(&tiles)
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
        assert_eq!(solve_part_2("input_15_test_3"), 618);
    }

    #[test]
    fn test_another_example_part2() {
        assert_eq!(solve_part_2("input_15_test_4"), 719);
    }

    #[test]
    fn test_bigger_example_part2() {
        assert_eq!(solve_part_2("input_15_test_2"), 9021);
    }
}
