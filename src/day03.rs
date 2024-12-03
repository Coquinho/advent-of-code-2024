use regex::{Regex, RegexSet};

use crate::utils::*;

pub fn part1() -> i32 {
    let mut mul: i32 = 0;

    if let Ok(lines) = read_lines("./input/03.txt") {
        for line in lines.flatten() {
            // println!("line {:?}", line);

            let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
            // println!("{:?} ", re.find(&line[..]));
            for (_, [x1, x2]) in re.captures_iter(&line[..]).map(|caps| caps.extract()) {
                // println!("x1 {:?} x2 {:?}", x1, x2);
                mul += x1.parse::<i32>().unwrap() * x2.parse::<i32>().unwrap();
            }
        }
    }

    mul
}

pub fn part2() -> i32 {
    let mut mul: i32 = 0;
    let mut enabled = true;

    let re_mul = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";
    let re_dont = r"don't\(\)";
    let re_do = r"do\(\)";

    let re = Regex::new(&format!("{}|{}|{}", re_mul, re_dont, re_do)).unwrap();

    if let Ok(lines) = read_lines("./input/03.txt") {
        for line in lines.flatten() {
            // println!("line {:?}", line);

            for caps in re.captures_iter(&line) {
                match caps.get(0).unwrap().as_str() {
                    "don't()" => {
                        enabled = false;
                        continue;
                    }
                    "do()" => {
                        enabled = true;
                        continue;
                    }
                    _ => {
                        if enabled {
                            let x1 = caps[1].parse::<i32>().unwrap();
                            let x2 = caps[2].parse::<i32>().unwrap();
                            mul += x1 * x2;
                        }
                    }
                }
            }
        }
    }

    mul
}
