#![allow(dead_code)]

use std::fmt::Debug;
use std::fs;
use std::time::{Duration, Instant};

use colored::Colorize;

pub trait AocResult: Debug + PartialEq {}

impl<T: Debug + PartialEq> AocResult for T {}

pub trait Part<R: AocResult> {
    fn expect_test(&self) -> R;
    fn solve(&self, input: &Vec<String>) -> R;
}

pub struct Day<R1: AocResult, R2: AocResult> {
    id: u8,
    test_input: Vec<String>,
    actual_input: Vec<String>,
    part1: Box<dyn Part<R1>>,
    part2: Box<dyn Part<R2>>,
}

impl<R1: AocResult + 'static, R2: AocResult+ 'static> Day<R1, R2> {
    pub fn new(id: u8, part1: Box<dyn Part<R1>>, part2: Box<dyn Part<R2>>) -> Self {
        Self {
            id,
            test_input: read_input(format!("input/{:0>2}_test.txt", id).as_str()),
            actual_input: read_input(format!("input/{:0>2}.txt", id).as_str()),
            part1,
            part2,
        }
    }

    fn timed<R: AocResult, F: Fn() -> R>(f: F) -> (R, Duration) {
        let start = Instant::now();
        let result = f();
        (result, start.elapsed())
    }

    fn run_part_test<R: AocResult>(&self, id: u8, part: &Box<dyn Part<R>>) -> Duration {
        let (actual, duration) = Self::timed(|| { part.solve(&self.test_input) });
        let expected = part.expect_test();
        assert_eq!(actual, expected, "Part {} test failed: Expected {:?} but got {:?}", id, expected, actual);
        println!("{}", format!("Part {} test was {} {:>10}", id, "successful".on_bright_green(), format!("{:?}", duration).purple()));
        duration
    }

    fn run_part_actual<R: AocResult>(&self, id: u8, part: &Box<dyn Part<R>>) -> Duration {
        let (actual, duration) = Self::timed(|| { part.solve(&self.actual_input) });
        println!("{}", format!("Part {} output {:>12} {:>10}", id, format!("{:?}", actual).blue(), format!("{:?}", duration).purple()).on_blue());
        duration
    }

    pub fn run_test(&self) {
        self.run_part_test(1, &self.part1);
        self.run_part_test(2, &self.part2);
    }

    pub fn run_actual(&self) {
        self.run_part_actual(1, &self.part1);
        self.run_part_actual(2, &self.part2);
    }

    pub fn run(&self) -> (Duration, Duration) {
        println!("~~~~~~~~{{ {} }} ~~~~~~~~", format!("Day{:0>2}", self.id).yellow());
        self.run_part_test(1, &self.part1);
        let first = self.run_part_actual(1, &self.part1);
        self.run_part_test(2, &self.part2);
        let second = self.run_part_actual(2, &self.part2);
        (first, second)
    }

    pub fn f(self) -> DayRunner {
        DayRunner::new(Box::new(move || self.run()))
    }
}

pub struct DayRunner {
    pub f: Box<dyn Fn() -> (Duration, Duration)>,
}

impl DayRunner {
    pub fn new(f: Box<dyn Fn() -> (Duration, Duration)>) -> Self {
        Self { f }
    }
}

fn read_input(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap().split("\n").map(String::from).collect::<Vec<_>>()
}
