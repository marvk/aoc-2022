#![feature(concat_idents)]

use std::env;

use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;
use crate::day04::day04;
use crate::day05::day05;
use crate::day06::day06;

mod harness;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let days = vec![day06()];

    let run_one = |id: usize| days[id].run();

    let run_all = ||
        for day in &days {
            day.run();
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
