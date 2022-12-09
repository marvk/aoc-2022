#![allow(dead_code, unused_imports)]

use std::any::Any;
use std::cmp::max;
use std::env;
use std::fmt::Debug;
use std::process::Command;
use std::time::Duration;

use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;
use crate::day04::day04;
use crate::day05::day05;
use crate::day06::day06;
use crate::day07::day07;
use crate::day08::day08;
use crate::day09::day09;
use crate::harness::{AocResult, Day};

mod harness;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
    let days = vec![
        day01().f(),
        day02().f(),
        day03().f(),
        day04().f(),
        day05().f(),
        day06().f(),
        day07().f(),
        day08().f(),
        day09().f(),
    ];


    let run_one = |id: usize| (days[id].f)();

    let run_all = ||
        for day in &days {
            (day.f)();
        };

    let run_latest = || run_one(days.len());

    let args = env::args().collect::<Vec<_>>();

    match args.get(1) {
        Some(arg) => {
            if let Ok(id) = arg.parse::<usize>() {
                run_one(id - 1);
            } else {
                match arg.as_str() {
                    "all" => { run_all(); }
                    _ => { run_latest(); }
                }
            }
        }
        None => { run_latest(); }
    };
}
