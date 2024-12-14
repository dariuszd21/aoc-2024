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

    let tries = 100;

    for game in games {
        println!("Analysing game: {:?}", game);
        match find_solution(&game, tries) {
            Some((a_tokens, b_tokens)) => {
                res += 3 * a_tokens + b_tokens;
            }
            None => (),
        }
    }

    res
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let games = load_games(lines);

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
        assert_eq!(solve_part_2("input_13_test"), 236);
    }
}
