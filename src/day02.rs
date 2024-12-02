use std::cmp::Ordering::Equal;

use crate::utils::*;

pub fn part1() -> u32 {
    let mut safe_count = 0;

    if let Ok(lines) = read_lines("./input/02.txt") {
        for line in lines.flatten() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect();

            if is_safe(&numbers) {
                safe_count += 1;
            }
        }
    }

    safe_count
}

fn is_safe(numbers: &Vec<i32>) -> bool {
    let operation = numbers[0].cmp(&numbers[1]);

    if operation == Equal {
        return false;
    }

    for i in 1..numbers.len() {
        let abs_diff = numbers[i - 1].abs_diff(numbers[i]);
        if !(1..4).contains(&abs_diff) {
            return false;
        }

        let current_operation = numbers[i - 1].cmp(&numbers[i]);
        if current_operation != operation {
            return false;
        }
    }

    true
}

pub fn part2() -> u32 {
    let mut safe_count = 0;

    if let Ok(lines) = read_lines("./input/02.txt") {
        for line in lines.flatten() {
            let numbers: Vec<i32> = numbers_in_line(line);
            if is_safe_with_dampener(&numbers) {
                safe_count += 1;
            }
            // is  428
        }
    }

    safe_count
}

fn numbers_without_index(numbers: &Vec<i32>, index: usize) -> Vec<i32> {
    numbers[..index]
        .iter()
        .chain(&numbers[index + 1..])
        .cloned()
        .collect()
}

fn is_safe_by_removing(numbers: &Vec<i32>, index_a: usize, index_b: usize) -> bool {
    return is_safe(&numbers_without_index(&numbers, index_a))
        || is_safe(&numbers_without_index(&numbers, index_b));
}

fn is_safe_with_dampener(numbers: &Vec<i32>) -> bool {
    if is_safe(numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        if is_safe(&numbers_without_index(&numbers, i)) {
            return true;
        }
    }

    false
}
