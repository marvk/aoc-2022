#![allow(dead_code)]

use std::fmt::Debug;
use std::fs;
use std::time::{Duration, Instant};

use colored::Colorize;

pub trait Part<Result: PartialEq + Debug> {
    fn expect_test(&self) -> Result;
    fn solve(&self, input: &Vec<String>) -> Result;
}

pub struct Day<Result1: PartialEq + Debug, Result2: PartialEq + Debug> {
    id: u8,
    test_input: Vec<String>,
    actual_input: Vec<String>,
    part1: Box<dyn Part<Result1>>,
    part2: Box<dyn Part<Result2>>,
}

impl<Result1: PartialEq + Debug, Result2: PartialEq + Debug> Day<Result1, Result2> {
    pub fn new(id: u8, part1: Box<dyn Part<Result1>>, part2: Box<dyn Part<Result2>>) -> Self {
        Self {
            id,
            test_input: read_input(format!("input/{:0>2}_test.txt", id).as_str()),
            actual_input: read_input(format!("input/{:0>2}.txt", id).as_str()),
            part1,
            part2,
        }
    }

    fn timed<Result: PartialEq + Debug, F: Fn() -> Result>(f: F) -> (Result, Duration) {
        let start = Instant::now();
        let result = f();
        (result, start.elapsed())
    }

    fn run_part_test<Result: PartialEq + Debug>(&self, id: u8, part: &Box<dyn Part<Result>>) {
        let (actual, duration) = Self::timed(|| { part.solve(&self.test_input) });
        let expected = part.expect_test();
        assert_eq!(actual, expected, "Part {} test failed: Expected {:?} but got {:?}", id, actual, expected);
        println!("{}", format!("Part {} test was {} {:>10}", id, "successful".on_bright_green(), format!("{:?}", duration).purple()));
    }

    fn run_part_actual<Result: PartialEq + Debug>(&self, id: u8, part: &Box<dyn Part<Result>>) {
        let (actual, duration) = Self::timed(|| { part.solve(&self.actual_input) });
        println!("{}", format!("Part {} output {:>12} {:>10}", id, format!("{:?}", actual).blue(), format!("{:?}", duration).purple()).on_blue());
    }

    pub fn run_test(&self) {
        self.run_part_test(1, &self.part1);
        self.run_part_test(2, &self.part2);
    }

    pub fn run_actual(&self) {
        self.run_part_actual(1, &self.part1);
        self.run_part_actual(2, &self.part2);
    }

    pub fn run(&self) {
        println!("~~~~~~~~{{ {} }} ~~~~~~~~", format!("Day{:0>2}", self.id).yellow());
        self.run_part_test(1, &self.part1);
        self.run_part_actual(1, &self.part1);
        self.run_part_test(2, &self.part2);
        self.run_part_actual(2, &self.part2);
    }
}

fn read_input(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap().split("\n").map(String::from).collect::<Vec<_>>()
}
