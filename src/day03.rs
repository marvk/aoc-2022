use std::collections::HashSet;

use crate::harness::{Day, Part};

pub fn day03() -> Day<u32, u32> {
    Day::new(3, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        157
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        input.iter().filter(|line| !line.is_empty()).map(part1).sum()
    }
}

fn part1(p0: &String) -> u32 {
    let compartments: Vec<HashSet<char>> =
        p0
            .chars()
            .collect::<Vec<_>>()
            .chunks(p0.len() / 2)
            .map(|arr| HashSet::from_iter(arr.iter().cloned()))
            .collect::<Vec<_>>();

    compartments[0].intersection(&compartments[1]).map(|c| score(*c)).sum()
}

fn part2(p0: &[String]) -> u32 {
    let rucksacks = p0.iter().map(|arr| arr.chars().collect::<Vec<char>>()).map(|chars: Vec<char>| -> HashSet<char> { HashSet::from_iter(chars.iter().cloned()) }).collect::<Vec<_>>();

    let x = rucksacks.iter().flatten().collect::<HashSet<_>>();

    x.into_iter().filter(|c| rucksacks.iter().all(|set| set.contains(c))).map(|c| score(*c)).sum()
}

fn score(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        70
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        input.chunks(3).collect::<Vec<_>>().into_iter().map(|x| part2(x)).sum()
    }
}
