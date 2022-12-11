use std::collections::HashMap;
use std::fmt::Write;

use crate::harness::{Day, Part};

pub fn day10() -> Day<i32, String> {
    Day::new(10, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        13140
    }

    fn solve(&self, input: &Vec<String>) -> i32 {
        solve(input).0
    }
}

pub struct Part2;

impl Part<String> for Part2 {
    fn expect_test(&self) -> String {
        r#"
██..██..██..██..██..██..██..██..██..██..
███...███...███...███...███...███...███.
████....████....████....████....████....
█████.....█████.....█████.....█████.....
██████......██████......██████......████
███████.......███████.......███████.....
        "#.trim().to_string()
    }

    fn solve(&self, input: &Vec<String>) -> String {
        solve(input).1
    }
}

fn solve(input: &Vec<String>) -> (i32, String) {
    let mut x = 1;

    let mut result = 0;

    let mut cycle = 1;

    let mut result2 = "".to_string();

    // shh sleep now

    let cycle_mod = cycle % 40;
    if cycle_mod == x || cycle_mod - 1 == x || cycle_mod + 1 == x {
        result2.push('█');
    } else {
        result2.push('.');
    }

    for inst in parse(input).into_iter() {
        if let Some(summand) = inst {
            let cycle_mod = cycle % 40;
            if cycle_mod == x || cycle_mod - 1 == x || cycle_mod + 1 == x {
                result2.push('█');
            } else {
                result2.push('.');
            }

            cycle += 1;

            if (cycle + 20) % 40 == 0 {
                result += cycle * x;
            }


            x += summand;
        }
        let cycle_mod = cycle % 40;
        if cycle_mod == x || cycle_mod - 1 == x || cycle_mod + 1 == x {
            result2.push('█');
        } else {
            result2.push('.');
        }
        cycle += 1;
        if (cycle + 20) % 40 == 0 {
            result += cycle * x;
        }
    }

    let map = result2.chars().collect::<Vec<_>>().chunks(40).map(|w| w.iter().collect::<String>()).take(6).collect::<Vec<_>>().join("\n");
    (result, map)
}

fn parse(input: &Vec<String>) -> Vec<Option<i32>> {
    input.iter().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Option<i32> {
    line.split(" ").collect::<Vec<_>>().last().unwrap().parse().ok()
}
