use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn numbers_in_line(line: String) -> Vec<i32> {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect();

    numbers
}
