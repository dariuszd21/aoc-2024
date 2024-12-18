#[path = "utils.rs"]
mod utils;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Button {
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Prize {
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Game {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct LinearFn {
    a: f64,
    b: f64,
}

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn parse_prize(prize_value: &str) -> Prize {
    let mut prize = Prize { x: 0, y: 0 };

    let splitted_value: Vec<_> = prize_value.split(",").collect();
    if splitted_value.len() == 2 {
        let (x_str, y_str) = (splitted_value[0].trim(), splitted_value[1].trim());
        prize.x = x_str[2..x_str.len()].parse().unwrap();
        prize.y = y_str[2..y_str.len()].parse().unwrap();
    }

    prize
}

fn parse_button(button_value: &str) -> Button {
    let mut button = Button { x: 0, y: 0 };

    let splitted_value: Vec<_> = button_value.split(",").collect();
    if splitted_value.len() == 2 {
        let (x_str, y_str) = (splitted_value[0].trim(), splitted_value[1].trim());
        button.x = x_str[2..x_str.len()].parse().unwrap();
        button.y = y_str[2..y_str.len()].parse().unwrap();
    }

    button
}

fn load_games(lines: Vec<String>) -> Vec<Game> {
    let mut games = Vec::new();

    let mut button_a = Button { x: 0, y: 0 };
    let mut button_b = Button { x: 0, y: 0 };
    for line in lines {
        if !line.is_empty() {
            let splitted_line: Vec<_> = line.split(":").collect();
            if splitted_line.len() == 2 {
                let (entry_name, values) = (splitted_line[0], splitted_line[1]);

                if entry_name.starts_with("Prize") {
                    let prize = parse_prize(values);
                    games.push(Game {
                        button_a,
                        button_b,
                        prize,
                    });
                    button_a = Button { x: 0, y: 0 };
                    button_b = Button { x: 0, y: 0 };
                }

                if entry_name.starts_with("Button A") {
                    button_a = parse_button(values);
                }
                if entry_name.starts_with("Button B") {
                    button_b = parse_button(values);
                }
            }
        }
    }

    games
}

fn calculate_linear_fn(button: &Button, start_x: u64, start_y: u64) -> LinearFn {
    let mut linear_fn = LinearFn { a: 0.0, b: 0.0 };

    let y_diff: f64 = button.y as f64;
    let x_diff: f64 = button.x as f64;

    let a_coef: f64 = y_diff / x_diff;
    let b_coef: f64 = start_y as f64 - a_coef * (start_x as f64);
    println!("Calculated fn y = {}x + {}", a_coef, b_coef);

    linear_fn.a = a_coef;
    linear_fn.b = b_coef;

    linear_fn
}

fn calculate_y(fn_: &LinearFn, x: f64) -> f64 {
    return fn_.a * x + fn_.b;
}

fn parse_prize_part_2(prize_value: &str) -> Prize {
    let mut prize = Prize { x: 0, y: 0 };

    let splitted_value: Vec<_> = prize_value.split(",").collect();
    if splitted_value.len() == 2 {
        let (x_str, y_str) = (splitted_value[0].trim(), splitted_value[1].trim());
        let calibration: u64 = 10_000_000_000_000;
        prize.x = calibration + &x_str[2..x_str.len()].parse().unwrap();
        prize.y = calibration + &y_str[2..y_str.len()].parse().unwrap();
    }

    prize
}

fn load_games_part2(lines: Vec<String>) -> Vec<Game> {
    let mut games = Vec::new();

    let mut button_a = Button { x: 0, y: 0 };
    let mut button_b = Button { x: 0, y: 0 };
    for line in lines {
        if !line.is_empty() {
            let splitted_line: Vec<_> = line.split(":").collect();
            if splitted_line.len() == 2 {
                let (entry_name, values) = (splitted_line[0], splitted_line[1]);

                if entry_name.starts_with("Prize") {
                    let prize = parse_prize_part_2(values);
                    games.push(Game {
                        button_a,
                        button_b,
                        prize,
                    });
                    button_a = Button { x: 0, y: 0 };
                    button_b = Button { x: 0, y: 0 };
                }

                if entry_name.starts_with("Button A") {
                    button_a = parse_button(values);
                }
                if entry_name.starts_with("Button B") {
                    button_b = parse_button(values);
                }
            }
        }
    }

    games
}

fn find_solution(game: &Game, tries: u64) -> Option<(u64, u64)> {
    for a_tokens_amount in 0..tries {
        for b_tokens_amount in 0..tries {
            let x = game.button_a.x * a_tokens_amount + game.button_b.x * b_tokens_amount;
            let y = game.button_a.y * a_tokens_amount + game.button_b.y * b_tokens_amount;
            if game.prize == (Prize { x, y }) {
                println!("Found solution: {} {}!", a_tokens_amount, b_tokens_amount);
                return Some((a_tokens_amount, b_tokens_amount));
            }
        }
    }
    None
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let games = load_games(lines);

    for game in games {
        println!("Analysing game: {:?}", game);
        match find_solution(&game, 100) {
            Some((a_tokens, b_tokens)) => {
                println!("Found solution {} {}", a_tokens, b_tokens);
            }
            None => (),
        }
        println!("Analysing game: {:?}", game);
        match find_solution_part_2(&game) {
            Some((a_tokens, b_tokens)) => {
                println!("Found solution alternative: {} {}", a_tokens, b_tokens);
                res += (3 * a_tokens) + b_tokens;
            }
            None => (),
        }
    }

    res
}

fn find_crossing_point(fn_a: &LinearFn, fn_b: &LinearFn) -> Option<(u64, u64)> {
    let x = (fn_a.b - fn_b.b) / (fn_b.a - fn_a.a);
    let y = calculate_y(&fn_a, x as f64);

    if x >= 0.0 && y >= 0.0 {
        return Some((x.round() as u64, y.round() as u64));
    }

    None
}

fn find_solution_part_2(game: &Game) -> Option<(u64, u64)> {
    let linear_f_a = calculate_linear_fn(&game.button_a, game.button_a.x, game.button_a.y);
    let linear_f_b = calculate_linear_fn(&game.button_b, game.prize.x, game.prize.y);

    if let Some((x, y)) = find_crossing_point(&linear_f_a, &linear_f_b) {
        println!("Found crossing point: ({}, {})", x, y);

        if x <= game.prize.x && y <= game.prize.y {
            let a_tokens_amount = x / game.button_a.x;
            let b_tokens_amount = (game.prize.x - x) / game.button_b.x;
            println!("Found solution: {} {}", a_tokens_amount, b_tokens_amount);
            let calculated_prize_x =
                a_tokens_amount * (game.button_a.x) + b_tokens_amount * (game.button_b.x);
            let calculated_prize_y =
                a_tokens_amount * (game.button_a.y) + b_tokens_amount * (game.button_b.y);
            if calculated_prize_x == game.prize.x && calculated_prize_y == game.prize.y {
                println!("Using solution: {} {}", a_tokens_amount, b_tokens_amount);
                return Some((a_tokens_amount, b_tokens_amount));
            }
        }
    }

    None
}
pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let games = load_games_part2(lines);
    for game in games {
        println!("Analysing game: {:?}", game);
        match find_solution_part_2(&game) {
            Some((a_tokens, b_tokens)) => {
                res += 3 * a_tokens + b_tokens;
            }
            None => (),
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part_1("input_13_test"), 480);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_13_test"), 875318608908);
    }
}
