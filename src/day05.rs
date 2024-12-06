use std::{
    collections::{HashMap, HashSet},
    fs,
};

use crate::utils::{numbers_in_line, read_lines};

pub fn is_valid_pages(pages: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut visited = HashSet::new();
    for &page in pages {
        if let Some(lowers) = rules.get(&page) {
            for &lower in lowers {
                if visited.contains(&lower) {
                    return false;
                }
            }
        }
        visited.insert(page);
    }
    true
}

pub fn part1(filename: &str) -> u32 {
    let mut counter = 0;
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    let file = fs::read_to_string(filename).unwrap();
    let (rules_str, updates_str) = file.split_once("\r\n\r\n").unwrap();

    for line in rules_str.lines() {
        let (a, b) = line.split_once('|').unwrap();
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();

        rules.entry(a).or_insert(HashSet::new()).insert(b);
    }
    let updates: Vec<Vec<u32>> = updates_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    for pages in updates {
        if is_valid_pages(&pages, &rules) {
            let middle = pages.len() / 2;
            counter += pages[middle];
        }
    }
    counter
}

pub fn part2(filename: &str) -> u32 {
    let mut counter = 0;
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    let file = fs::read_to_string(filename).unwrap();
    let (rules_str, updates_str) = file.split_once("\r\n\r\n").unwrap();

    for line in rules_str.lines() {
        let (a, b) = line.split_once('|').unwrap();
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();

        rules.entry(a).or_insert(HashSet::new()).insert(b);
    }
    let updates: Vec<Vec<u32>> = updates_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    for pages in updates {
        if !is_valid_pages(&pages, &rules) {
            let sorted_pages = sort_updates(&pages, &rules);
            let middle = pages.len() / 2;
            counter += pages[middle];
        }
    }
    counter
}
#[cfg(test)]
mod test {
    use crate::day05;

    #[test]
    fn part1() {
        assert_eq!(day05::part1("./input/05-test.txt"), 143)
    }

    #[test]
    fn part2() {
        assert_eq!(day05::part2("./input/05-test.txt"), 123)
    }
}
