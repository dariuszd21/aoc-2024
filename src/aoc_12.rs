#[path = "utils.rs"]
mod utils;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Plant {
    kind: char,
}

fn load_lines(filepath: &str) -> Vec<String> {
    let lines = utils::read_file(filepath);
    lines
}

fn load_plants(lines: Vec<String>) -> Vec<Vec<Plant>> {
    let mut plants = Vec::new();

    for line in lines {
        if !line.is_empty() {
            let mut plants_row = Vec::new();
            for c in line.chars() {
                plants_row.push(Plant { kind: c });
            }
            plants.push(plants_row);
        }
    }

    plants
}

fn find_unvisited(visited_plants: &Vec<Vec<bool>>) -> Option<(usize, usize)> {
    for (i, plants_row) in visited_plants.iter().enumerate() {
        for (j, was_visited) in plants_row.iter().enumerate() {
            if !was_visited {
                return Some((i, j));
            }
        }
    }
    None
}

fn calculate_area(
    plants: &Vec<Vec<Plant>>,
    plant_x: usize,
    plant_y: usize,
    visited_plants: &mut Vec<Vec<bool>>,
) -> (u64, u64) {
    let perimeter = 0;
    let area = 0;

    if visited_plants[plant_x][plant_y] {
        return (perimeter, area);
    }

    let mut perimeter = 4;
    let mut area = 1;

    let current_plant = plants[plant_x][plant_y];

    visited_plants[plant_x][plant_y] = true;

    if plant_x > 0 && plants[plant_x - 1][plant_y] == current_plant {
        perimeter -= 1;
        let res = calculate_area(&plants, plant_x - 1, plant_y, visited_plants);
        perimeter += res.0;
        area += res.1;
    }
    if plant_y > 0 && plants[plant_x][plant_y - 1] == current_plant {
        perimeter -= 1;
        let res = calculate_area(&plants, plant_x, plant_y - 1, visited_plants);
        perimeter += res.0;
        area += res.1;
    }
    if plant_x < plants.len() - 1 && plants[plant_x + 1][plant_y] == current_plant {
        perimeter -= 1;
        let res = calculate_area(&plants, plant_x + 1, plant_y, visited_plants);
        perimeter += res.0;
        area += res.1;
    }
    if plant_y < plants[0].len() - 1 && plants[plant_x][plant_y + 1] == current_plant {
        perimeter -= 1;
        let res = calculate_area(&plants, plant_x, plant_y + 1, visited_plants);
        perimeter += res.0;
        area += res.1;
    }

    (perimeter, area)
}

pub fn solve_part_1(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let plants = load_plants(lines);
    let mut visited_plants: Vec<Vec<bool>> = Vec::new();

    let plants_rows = plants.len();
    let plants_columns = plants[0].len();

    for _ in 0..plants_rows {
        let mut visited_row = Vec::new();
        for _ in 0..plants_columns {
            visited_row.push(false);
        }
        visited_plants.push(visited_row);
    }

    while let Some((plant_x, plant_y)) = find_unvisited(&visited_plants) {
        let plant = plants[plant_x][plant_y];
        let (perimeter, area) = calculate_area(&plants, plant_x, plant_y, &mut visited_plants);
        res += perimeter * area;
    }

    res
}

fn get_lookup_neighbourhood(
    plants: &Vec<Vec<Plant>>,
    plant_x: usize,
    plant_y: usize,
    plant: Plant,
) -> Vec<Vec<bool>> {
    let mut neighbourhood = Vec::new();
    let num_rows = plants.len();
    let num_cols = plants[0].len();

    for i in plant_x as i64 - 1..=plant_x as i64 + 1 {
        let mut neighbourhood_row = Vec::new();
        for j in plant_y as i64 - 1..=plant_y as i64 + 1 {
            if i < 0 {
                neighbourhood_row.push(false);
                continue;
            }
            if i > TryInto::<i64>::try_into(num_rows).unwrap() - 1 {
                neighbourhood_row.push(false);
                continue;
            }
            if j < 0 {
                neighbourhood_row.push(false);
                continue;
            }
            if j > TryInto::<i64>::try_into(num_cols).unwrap() - 1 {
                neighbourhood_row.push(false);
                continue;
            }
            neighbourhood_row.push(plant == plants[i as usize][j as usize]);
        }
        neighbourhood.push(neighbourhood_row);
    }

    neighbourhood
}

fn calculate_sides(
    plants: &Vec<Vec<Plant>>,
    plant_x: usize,
    plant_y: usize,
    visited_plants: &mut Vec<Vec<bool>>,
) -> (u64, u64) {
    let area = 0;
    let vertices = 0;

    if visited_plants[plant_x][plant_y] {
        return (vertices, area);
    }

    let mut vertices = 0;
    let mut area = 1;

    let current_plant = plants[plant_x][plant_y];

    visited_plants[plant_x][plant_y] = true;

    let lookup_neighbourhood = get_lookup_neighbourhood(&plants, plant_x, plant_y, current_plant);

    // vertices for the given tile
    // something is a vertice if the is another tile at the diagonal
    // and the neighbour tiles are both different kind or the same as current
    // * -
    // - -
    // or
    // * *
    // * -
    // In both above cases the last minus has a vertice at the upper left side
    if !lookup_neighbourhood[0][0] {
        if (lookup_neighbourhood[0][1] && lookup_neighbourhood[1][0])
            || (!lookup_neighbourhood[0][1] && !lookup_neighbourhood[1][0])
        {
            vertices += 1;
        }
    }

    if !lookup_neighbourhood[0][2] {
        if (lookup_neighbourhood[0][1] && lookup_neighbourhood[1][2])
            || (!lookup_neighbourhood[0][1] && !lookup_neighbourhood[1][2])
        {
            vertices += 1;
        }
    }

    if !lookup_neighbourhood[2][0] {
        if (lookup_neighbourhood[1][0] && lookup_neighbourhood[2][1])
            || (!lookup_neighbourhood[1][0] && !lookup_neighbourhood[2][1])
        {
            vertices += 1;
        }
    }

    if !lookup_neighbourhood[2][2] {
        if (lookup_neighbourhood[1][2] && lookup_neighbourhood[2][1])
            || (!lookup_neighbourhood[1][2] && !lookup_neighbourhood[2][1])
        {
            vertices += 1;
        }
    }

    // Edge case where two regions of plants of the same kind are
    // neighbours on the diagonal
    // * -
    // - *
    if lookup_neighbourhood[0][0] {
        if !lookup_neighbourhood[0][1] && !lookup_neighbourhood[1][0] {
            vertices += 1;
        }
    }

    if lookup_neighbourhood[0][2] {
        if !lookup_neighbourhood[0][1] && !lookup_neighbourhood[1][2] {
            vertices += 1;
        }
    }

    if lookup_neighbourhood[2][0] {
        if !lookup_neighbourhood[1][0] && !lookup_neighbourhood[2][1] {
            vertices += 1;
        }
    }

    if lookup_neighbourhood[2][2] {
        if !lookup_neighbourhood[1][2] && !lookup_neighbourhood[2][1] {
            vertices += 1;
        }
    }

    if plant_x > 0 && plants[plant_x - 1][plant_y] == current_plant {
        let res = calculate_sides(&plants, plant_x - 1, plant_y, visited_plants);
        vertices += res.0;
        area += res.1;
    }
    if plant_y > 0 && plants[plant_x][plant_y - 1] == current_plant {
        let res = calculate_sides(&plants, plant_x, plant_y - 1, visited_plants);
        vertices += res.0;
        area += res.1;
    }
    if plant_x < plants.len() - 1 && plants[plant_x + 1][plant_y] == current_plant {
        let res = calculate_sides(&plants, plant_x + 1, plant_y, visited_plants);
        vertices += res.0;
        area += res.1;
    }
    if plant_y < plants[0].len() - 1 && plants[plant_x][plant_y + 1] == current_plant {
        let res = calculate_sides(&plants, plant_x, plant_y + 1, visited_plants);
        vertices += res.0;
        area += res.1;
    }

    (vertices, area)
}

pub fn solve_part_2(filepath: &str) -> u64 {
    let lines = load_lines(filepath);
    let mut res = 0;

    let plants = load_plants(lines);
    let mut visited_plants: Vec<Vec<bool>> = Vec::new();

    let plants_rows = plants.len();
    let plants_columns = plants[0].len();

    for _ in 0..plants_rows {
        let mut visited_row = Vec::new();
        for _ in 0..plants_columns {
            visited_row.push(false);
        }
        visited_plants.push(visited_row);
    }

    while let Some((plant_x, plant_y)) = find_unvisited(&visited_plants) {
        let (sides, area) = calculate_sides(&plants, plant_x, plant_y, &mut visited_plants);
        res += sides * area;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(solve_part_1("input_12_test_2"), 772);
    }

    #[test]
    fn test_bigger_example_part1() {
        assert_eq!(solve_part_1("input_12_test"), 1930);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(solve_part_2("input_12_test_3"), 236);
    }

    #[test]
    fn test_bigger_example_part2() {
        assert_eq!(solve_part_2("input_12_test"), 1206);
    }

    #[test]
    fn test_another_example_part2() {
        assert_eq!(solve_part_2("input_12_test_4"), 368);
    }
}
