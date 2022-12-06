use std::collections::HashSet;

use crate::harness::{Day, Part};

pub fn day06() -> Day<u32, u32> {
    Day::new(06, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        7
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        do_the_thing(input, 4)
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        19
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        do_the_thing(input, 14)
    }
}

fn do_the_thing(input: &Vec<String>, n: usize) -> u32 {
    input[0]
        .chars()
        .collect::<Vec<_>>()
        .windows(n)
        .enumerate()
        .find(|(_, arr)| are_all_elements_unique(arr))
        .map(|(i, _)| (i + n) as u32)
        .unwrap()
}

fn are_all_elements_unique(arr: &&[char]) -> bool {
    HashSet::<_>::from_iter(arr.iter()).len() == arr.len()
}
