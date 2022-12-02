use crate::harness::{Day, Part};

pub fn day01() -> Day<u32, u32> {
    Day::new(1, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        24000
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        parse_elves(input).into_iter().max().unwrap()
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        45000
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let mut vec = parse_elves(input);
        vec.sort();
        vec.reverse();
        vec[0..3].iter().sum::<u32>()
    }
}

fn parse_elves(input: &Vec<String>) -> Vec<u32> {
    let mut elves = Vec::new();

    let mut current_elf = 0_u32;

    for calories in input {
        if let Ok(u) = calories.parse::<u32>() {
            current_elf += u;
        } else {
            elves.push(current_elf);
            current_elf = 0;
        }
    }

    if current_elf != 0 {
        elves.push(current_elf);
    }

    elves
}
