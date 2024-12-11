use std::{collections::HashSet, fs};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn new_position(self, current: (usize, usize)) -> (usize, usize) {
        let offset = self.offset();
        (
            (current.0 as isize + offset.0) as usize,
            (current.1 as isize + offset.1) as usize,
        )
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

fn read_input_part1(filename: &str) -> Vec<Vec<u32>> {
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn pad_matrix_with_zeros(input: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let rows = input.len();
    let columns = input[0].len();
    let mut padded = vec![vec![0; columns + 2]; rows + 2];

    for row in 0..rows {
        for column in 0..columns {
            padded[row + 1][column + 1] = input[row][column];
        }
    }

    padded
}

fn find_trailheads(input: &Vec<Vec<u32>>) -> HashSet<(usize, usize)> {
    let mut trailheads: HashSet<(usize, usize)> = HashSet::new();

    for row in 1..input.len() - 1 {
        for column in 1..input[row].len() - 1 {
            if input[row][column] == 0 {
                trailheads.insert((row, column));
            }
        }
    }

    trailheads
}

fn dfs(
    input: &Vec<Vec<u32>>,
    visited: &mut HashSet<(usize, usize)>,
    current: &(usize, usize),
    previous_value: Option<u32>,
) -> u32 {
    let mut counter = 0;

    let already_visited = visited.contains(current);
    if already_visited {
        return counter;
    }

    let current_value = input[current.0][current.1];
    let is_an_increment = previous_value.map_or(true, |value| current_value == value + 1);
    if !is_an_increment {
        return counter;
    }

    visited.insert(*current);
    if input[current.0][current.1] == 9 {
        counter += 1;
    }
    println!("current {:?} = {} visited ", current, current_value,);

    for direction in &DIRECTIONS {
        let to_visit = direction.new_position(*current);
        counter += dfs(&input, visited, &to_visit, Some(current_value));
    }

    counter
}

pub fn part1(filename: &str) -> u32 {
    let mut counter = 0;
    let input = read_input_part1(filename);

    let input = pad_matrix_with_zeros(&input);
    let trailheads = find_trailheads(&input);
    println!("input {:?}\ntrailhead {:?}", input, trailheads);
    for trailhead in trailheads {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        counter += dfs(&input, &mut visited, &trailhead, None);
    }

    counter
}

fn dfs_part2(input: &Vec<Vec<u32>>, current: &(usize, usize), previous_value: Option<u32>) -> u32 {
    let mut counter = 0;

    let current_value = input[current.0][current.1];
    let is_an_increment = previous_value.map_or(true, |value| current_value == value + 1);
    if !is_an_increment {
        return counter;
    }

    if input[current.0][current.1] == 9 {
        counter += 1;
    }
    println!("current {:?} = {} visited ", current, current_value,);

    for direction in &DIRECTIONS {
        let to_visit = direction.new_position(*current);
        counter += dfs_part2(&input, &to_visit, Some(current_value));
    }

    counter
}
pub fn part2(filename: &str) -> u32 {
    let mut counter = 0;
    let input = read_input_part1(filename);

    let input = pad_matrix_with_zeros(&input);
    let trailheads = find_trailheads(&input);
    println!("input {:?}\ntrailhead {:?}", input, trailheads);
    for trailhead in trailheads {
        counter += dfs_part2(&input, &trailhead, None);
    }

    counter
}

#[cfg(test)]
mod test {
    use crate::day10;

    #[test]
    fn part1() {
        assert_eq!(day10::part1("./input/10-test.txt"), 36);
    }

    #[test]
    fn part2() {
        assert_eq!(day10::part2("./input/10-test.txt"), 81);
    }
}
