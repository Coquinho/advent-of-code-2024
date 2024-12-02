use std::collections::HashMap;

use crate::utils::read_lines;

pub fn part1() -> u32 {
    let mut diference = 0;
    let mut list_01: Vec<i32> = Vec::new();
    let mut list_02: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./input/01.txt") {
        for line in lines.flatten() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect();

            list_01.push(numbers[0]);
            list_02.push(numbers[1]);
        }
    }

    list_01.sort();
    list_02.sort();

    for i in 0..list_01.len() {
        diference += list_01[i].abs_diff(list_02[i]);
    }

    diference
}

pub fn part2() -> i32 {
    let mut score = 0;
    let mut count_01: HashMap<i32, i32> = HashMap::new();
    let mut count_02: HashMap<i32, i32> = HashMap::new();

    if let Ok(lines) = read_lines("./input/01.txt") {
        for line in lines.flatten() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect();

            *count_01.entry(numbers[0]).or_insert(0) += 1;
            *count_02.entry(numbers[1]).or_insert(0) += 1;
        }
    }

    for (num, count) in count_01.iter() {
        score += num * count * count_02.get(num).unwrap_or(&0);
    }

    score
}
