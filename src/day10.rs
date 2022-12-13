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
    let mut result_1 = 0;
    let mut result_2 = "".to_string();

    let mut x = 1;
    let mut cycle = 1;

    append_result_2(x, cycle, &mut result_2);

    for inst in parse(input).into_iter() {
        if let Some(summand) = inst {
            append_result_2(x, cycle, &mut result_2);
            cycle += 1;
            append_result_1(x, cycle, &mut result_1);

            x += summand;
        }

        append_result_2(x, cycle, &mut result_2);
        cycle += 1;
        append_result_1(x, cycle, &mut result_1);
    }

    (result_1, compose_result_2(result_2))
}

fn compose_result_2(result_2: String) -> String {
    result_2
        .chars()
        .collect::<Vec<_>>()
        .chunks(40)
        .map(|w| w.iter().collect::<String>())
        .take(6)
        .collect::<Vec<_>>()
        .join("\n")
}

fn append_result_1(x: i32, cycle: i32, result_1: &mut i32) {
    if (cycle + 20) % 40 == 0 {
        *result_1 = *result_1 + cycle * x;
    }
}

fn append_result_2(x: i32, cycle: i32, result2: &mut String) {
    let cycle_mod = cycle % 40;
    if cycle_mod == x || cycle_mod - 1 == x || cycle_mod + 1 == x {
        result2.push('█');
    } else {
        result2.push('.');
    }
}

fn parse(input: &Vec<String>) -> Vec<Option<i32>> {
    input.iter().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Option<i32> {
    line.split(" ").collect::<Vec<_>>().last().unwrap().parse().ok()
}
