#![allow(dead_code, unused_imports)]

extern crate core;

use std::any::Any;
use std::cmp::max;
use std::env;
use std::fmt::Debug;
use std::io::Error;
use std::process::{Command, ExitStatus, Stdio};
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
use crate::day10::day10;
use crate::day11::day11;
use crate::day12::day12;
use crate::day13::day13;
use crate::day14::day14;
use crate::day15::day15;
use crate::day16::day16;
use crate::day17::day17;
use crate::day18::day18;
use crate::day19::day19;
use crate::day20::day20;
// use crate::day21::day21;
// use crate::day22::day22;
// use crate::day23::day23;
// use crate::day24::day24;
// use crate::day25::day25;
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
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

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
        day10().f(),
        day11().f(),
        day12().f(),
        day13().f(),
        day14().f(),
        day15().f(),
        day16().f(),
        day17().f(),
        day18().f(),
        day19().f(),
        day20().f(),
        // day21().f(),
        // day22().f(),
        // day23().f(),
        // day24().f(),
        // day25().f(),
    ];


    let run_one = |id: usize| (days[id - 1].f)();

    let run_all = || {
        let (p1, p2) = days.iter().map(|d| (d.f)()).unzip();
        plot(p1, p2).unwrap();
    };

    let run_latest = || run_one(days.len());

    let args = env::args().collect::<Vec<_>>();

    match args.get(1) {
        Some(arg) => {
            if let Ok(id) = arg.parse::<usize>() {
                run_one(id);
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

fn plot(part1: Vec<Duration>, part2: Vec<Duration>) -> Result<(), Error> {
    let convert = |v: Vec<Duration>| v.into_iter().map(|d| d.as_micros()).map(|n| n.to_string()).collect::<Vec<_>>().join("#");

    // Lol just plot with kotlin who's gonna stop me???
    Command::new("java")
        .arg("-jar")
        .arg("plotter.jar")
        .arg(&convert(part1))
        .arg(&convert(part2))
        .spawn()?
        .wait()?;

    Ok(())
}
