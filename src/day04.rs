use std::ops::Range;

use crate::harness::{Day, Part};

pub fn day04() -> Day<u32, u32> {
    Day::new(4, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        2
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        input.into_iter()
            .filter(|line| !line.is_empty())
            .map(parse_line)
            .map(|a| Self::score(a.get(0).unwrap(), a.get(1).unwrap()))
            .sum()
    }
}

impl Part1 {
    fn score(r1: &Range<usize>, r2: &Range<usize>) -> u32 {
        if range_contains_other(r1, r2) || range_contains_other(r2, r1) {
            1
        } else {
            0
        }
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        4
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        input.into_iter()
            .filter(|line| !line.is_empty())
            .map(parse_line)
            .map(|a| Self::score(a.get(0).unwrap(), a.get(1).unwrap()))
            .sum()
    }
}

impl Part2 {
    fn score(r1: &Range<usize>, r2: &Range<usize>) -> u32 {
        if ranges_overlap(r1, r2) {
            1
        } else {
            0
        }
    }
}


fn parse_line(line: &String) -> Vec<Range<usize>> {
    line.split(",").map(parse_section).collect::<Vec<_>>()
}

fn parse_section(s: &str) -> Range<usize> {
    let vec = s.split("-").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    *vec.get(0).unwrap()..*vec.get(1).unwrap() + 1
}

fn range_contains_other(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start >= b.start && a.end <= b.end
}

fn ranges_overlap(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.contains(&b.start) || a.contains(&(b.end-1)) || b.contains(&a.start) || b.contains(&(a.end-1))
}
