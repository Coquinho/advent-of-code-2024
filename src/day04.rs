use crate::utils::{numbers_in_line, read_lines};

pub fn part1(filename: &str) -> u32 {
    let mut counter: u32 = 0;
    let mut matrix: Vec<Vec<char>>;

    if let Ok(lines) = read_lines(filename) {
        matrix = lines
            .map(|line| line.expect("Failed to unwrap char").chars().collect())
            .collect();

        // print_matrix(&matrix);
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                counter += match matrix[y][x] {
                    'X' => read_xmas(&matrix, x, y),
                    'S' => read_samx(&matrix, x, y),
                    _ => 0,
                }
            }
        }
    }

    counter
}

pub fn print_matrix(matrix: &Vec<Vec<char>>) -> () {
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            print!("{} ", matrix[y][x]);
        }
        println!("");
    }
}

pub fn read_xmas(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut counter: u32 = 0;

    let top_left = y as i32 - 3 >= 0
        && x as i32 - 3 >= 0
        && matrix[y - 1][x - 1] == 'M'
        && matrix[y - 2][x - 2] == 'A'
        && matrix[y - 3][x - 3] == 'S';
    let top = y as i32 - 3 >= 0
        && matrix[y - 1][x] == 'M'
        && matrix[y - 2][x] == 'A'
        && matrix[y - 3][x] == 'S';
    let top_right = y as i32 - 3 >= 0
        && x + 3 < matrix[y].len()
        && matrix[y - 1][x + 1] == 'M'
        && matrix[y - 2][x + 2] == 'A'
        && matrix[y - 3][x + 3] == 'S';
    let left = x as i32 - 3 >= 0
        && matrix[y][x - 1] == 'M'
        && matrix[y][x - 2] == 'A'
        && matrix[y][x - 3] == 'S';

    let positions = [
        (top_left, "top_left"),
        (top, "top"),
        (top_right, "top_right"),
        (left, "left"),
    ];
    for position in positions {
        if position.0 {
            // println!("XMAS[{}][{}] {}", y, x, position.1);
            counter += 1;
        }
    }

    counter
}

pub fn read_samx(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut counter: u32 = 0;

    let top_left = y as i32 - 3 >= 0
        && x as i32 - 3 >= 0
        && matrix[y - 1][x - 1] == 'A'
        && matrix[y - 2][x - 2] == 'M'
        && matrix[y - 3][x - 3] == 'X';
    let top = y as i32 - 3 >= 0
        && matrix[y - 1][x] == 'A'
        && matrix[y - 2][x] == 'M'
        && matrix[y - 3][x] == 'X';
    let top_right = y as i32 - 3 >= 0
        && x + 3 < matrix[y].len()
        && matrix[y - 1][x + 1] == 'A'
        && matrix[y - 2][x + 2] == 'M'
        && matrix[y - 3][x + 3] == 'X';
    let left = x as i32 - 3 >= 0
        && matrix[y][x - 1] == 'A'
        && matrix[y][x - 2] == 'M'
        && matrix[y][x - 3] == 'X';

    let positions = [
        (top_left, "top_left"),
        (top, "top"),
        (top_right, "top_right"),
        (left, "left"),
    ];
    for position in positions {
        if position.0 {
            // println!("XMAS[{}][{}] {}", y, x, position.1);
            counter += 1;
        }
    }

    counter
}

pub fn part2(filename: &str) -> u32 {
    let mut counter: u32 = 0;
    let mut matrix: Vec<Vec<char>>;

    if let Ok(lines) = read_lines(filename) {
        matrix = lines
            .map(|line| line.expect("Failed to unwrap char").chars().collect())
            .collect();

        // print_matrix(&matrix);
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                counter += match matrix[y][x] {
                    'A' => read_mas(&matrix, x, y),
                    _ => 0,
                }
            }
        }
    }

    counter
}

pub fn read_mas(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut counter: u32 = 0;

    let in_bounds =
        y as i32 - 1 >= 0 && y + 1 < matrix.len() && x as i32 - 1 >= 0 && x + 1 < matrix[y].len();

    if !in_bounds {
        return counter;
    }

    let top_left = matrix[y - 1][x - 1] == 'M' && matrix[y + 1][x + 1] == 'S';
    let bottom_right = matrix[y - 1][x - 1] == 'S' && matrix[y + 1][x + 1] == 'M';
    let top_right = matrix[y - 1][x + 1] == 'M' && matrix[y + 1][x - 1] == 'S';
    let bottom_left = matrix[y - 1][x + 1] == 'S' && matrix[y + 1][x - 1] == 'M';

    if (top_left || bottom_right) && (top_right || bottom_left) {
        counter += 1
    }

    counter
}

#[cfg(test)]
mod test {
    use crate::day04;

    #[test]
    fn part1() -> () {
        assert_eq!(day04::part1("./input/04-test.txt"), 18);
    }

    #[test]
    fn part2() -> () {
        assert_eq!(day04::part2("./input/04-test.txt"), 9);
    }
}
