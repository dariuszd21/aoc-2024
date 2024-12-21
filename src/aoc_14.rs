#[path = "utils.rs"]
mod utils;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Velocity {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Robot {
    x: i64,
    y: i64,
    v: Velocity,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Quadrants {
    q1: u64,
    q2: u64,
    q3: u64,
    q4: u64,
}

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn parse_value(value: &str) -> (i64, i64) {
    let (mut x, mut y) = (0, 0);

    let splitted_value: Vec<_> = value.split(",").collect();
    if splitted_value.len() == 2 {
        let (x_str, y_str) = (splitted_value[0].trim(), splitted_value[1].trim());
        x = x_str.parse().unwrap();
        y = y_str.parse().unwrap();
    }

    (x, y)
}

fn load_robots(lines: Vec<String>) -> Vec<Robot> {
    let mut robots = Vec::new();

    for line in lines {
        if !line.is_empty() {
            let splitted_line: Vec<_> = line.split(" ").collect();
            if splitted_line.len() == 2 {
                let robot_pos = parse_value(&splitted_line[0][2..splitted_line[0].len()]);
                let robot_v = parse_value(&splitted_line[1][2..splitted_line[1].len()]);
                robots.push(Robot {
                    x: robot_pos.0,
                    y: robot_pos.1,
                    v: Velocity {
                        x: robot_v.0,
                        y: robot_v.1,
                    },
                });
            }
        }
    }

    robots
}

fn print_robots(robots: &Vec<Robot>, map_height: i64, map_width: i64) {
    let mut robots_map: HashMap<(i64, i64), u64> = HashMap::new();

    for i in 0..map_width {
        for j in 0..map_height {
            robots_map.insert((i, j), 0);
        }
    }
    for robot in robots {
        match robots_map.get(&(robot.x, robot.y)) {
            Some(amount) => robots_map.insert((robot.x, robot.y), amount + 1),
            None => robots_map.insert((robot.x, robot.y), 1),
        };
    }

    for j in 0..map_height {
        for i in 0..map_width {
            match robots_map.get(&(i, j)) {
                Some(val) => {
                    if *val == 0 {
                        print!(".");
                    } else {
                        print!("{}", val);
                    }
                }
                None => (),
            }
        }
        println!("");
    }
}

fn run_simulation(robots: &Vec<Robot>, map_height: i64, map_width: i64) -> Vec<Robot> {
    let mut new_robots_positions = Vec::new();

    for robot in robots {
        let mut new_robot = robot.clone();
        new_robot.x += new_robot.v.x;
        new_robot.y += new_robot.v.y;
        if new_robot.x < 0 {
            new_robot.x += map_width;
        }
        if new_robot.x >= map_width {
            new_robot.x -= map_width;
        }
        if new_robot.y < 0 {
            new_robot.y += map_height;
        }
        if new_robot.y >= map_height {
            new_robot.y -= map_height;
        }
        new_robots_positions.push(new_robot);
    }

    new_robots_positions
}

fn robots_per_quardant(robots: &Vec<Robot>, map_height: i64, map_width: i64) -> Quadrants {
    let mut quadrants = Quadrants {
        q1: 0,
        q2: 0,
        q3: 0,
        q4: 0,
    };

    let half_height = map_height / 2;
    let half_width = map_width / 2;

    for robot in robots {
        let robot_x = robot.x;
        let robot_y = robot.y;
        if robot_x < half_width && robot_y < half_height {
            quadrants.q1 += 1;
        }
        if robot_x < half_width && robot_y > half_height {
            quadrants.q2 += 1;
        }
        if robot_y < half_height && robot_x > half_width {
            quadrants.q3 += 1;
        }
        if robot_x > half_width && robot_y > half_height {
            quadrants.q4 += 1;
        }
    }

    quadrants
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let map_height = 103;
    let map_width = 101;

    let mut robots = load_robots(lines);

    for _ in 0..100 {
        robots = run_simulation(&robots, map_height, map_width);
    }

    let quadrants = robots_per_quardant(&robots, map_height, map_width);

    quadrants.q1 * quadrants.q2 * quadrants.q3 * quadrants.q4
}

fn find_christmas_tree(robots: &Vec<Robot>) -> bool {
    let mut robots_map: HashMap<(i64, i64), u64> = HashMap::new();

    for robot in robots {
        match robots_map.get(&(robot.x, robot.y)) {
            Some(amount) => robots_map.insert((robot.x, robot.y), amount + 1),
            None => robots_map.insert((robot.x, robot.y), 1),
        };
    }

    *robots_map.values().max().unwrap() == 1
}

fn count_cycles(robot: &Robot, map_height: i64, map_width: i64) -> u64 {
    let mut cycles = 0;

    let (mut x, mut y) = (robot.x, robot.y);
    loop {
        cycles += 1;
        x += robot.v.x;
        y += robot.v.y;

        if x < 0 {
            x += map_width;
        }
        if x >= map_width {
            x -= map_width;
        }
        if y < 0 {
            y += map_height;
        }
        if y >= map_height {
            y -= map_height;
        }

        if (x == robot.x) && (y == robot.y) {
            break;
        }
    }

    cycles
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let map_height = 103;
    let map_width = 101;

    let mut robots = load_robots(lines);

    // finding how many positions every robot can have (to know what's the max)
    // let cycles: HashMap<(i64, i64), u64> = robots
    //     .iter()
    //     .map(|r| ((r.x, r.y), count_cycles(r, map_height, map_width)))
    //     .collect();
    //
    // println!("Cycles {:?}", cycles);
    // for (start_pos, c) in &cycles {
    //     if *c != 10403 {
    //         println!("{:?}, {}", start_pos, c);
    //     }
    // }

    while !find_christmas_tree(&robots) {
        res += 1;
        robots = run_simulation(&robots, map_height, map_width);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let lines = load_lines("input_14_test");
        let map_height = 7;
        let map_width = 11;

        let mut robots = load_robots(lines);

        for _ in 0..100 {
            robots = run_simulation(&robots, map_height, map_width);
        }

        println!("{:?}", robots);
        print_robots(&robots, map_height, map_width);
        let quadrants = robots_per_quardant(&robots, map_height, map_width);

        let res = quadrants.q1 * quadrants.q2 * quadrants.q3 * quadrants.q4;
        assert_eq!(res, 12);
    }

    #[test]
    fn test_example_part2() {
        let lines = load_lines("input_14_test_2");
        let mut res = 0;

        let map_height = 7;
        let map_width = 11;

        let mut robots = load_robots(lines);
        println!("Number of robots: {}", robots.len());

        print_robots(&robots, map_height, map_width);
        let target = find_christmas_tree(&robots);

        assert!(target);
    }
}
