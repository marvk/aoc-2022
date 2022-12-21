use std::collections::HashMap;
use std::ops::Not;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::thread;

use crate::day21::Value::{Operation, Single};
use crate::harness::{Day, Part};

pub fn day21() -> Day<i64, i64> {
    Day::new(21, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i64> for Part1 {
    fn expect_test(&self) -> i64 {
        152
    }

    fn solve(&self, input: &Vec<String>) -> i64 {
        let monkeys = parse_lines(input);
        let result = solve_monkeys(monkeys);

        *result.get("root").unwrap()
    }
}

pub struct Part2;

impl Part<i64> for Part2 {
    fn expect_test(&self) -> i64 {
        0
    }

    fn solve(&self, input: &Vec<String>) -> i64 {
        0
    }
}

fn solve_monkeys(monkeys: Vec<Monkey>) -> HashMap<String, i64> {
    let mut open_monkeys = monkeys;

    let mut results = HashMap::new();

    while open_monkeys.is_empty().not() {
        open_monkeys.retain(|monkey| {
            match &monkey.value {
                Single(value) => {
                    results.insert(monkey.name.clone(), *value);
                    false
                }
                Operation(term1, op, term2) => {
                    match (results.get(term1), results.get(term2)) {
                        (Some(term1), Some(term2)) => {
                            let insert = match op {
                                '+' => term1 + term2,
                                '-' => term1 - term2,
                                '*' => term1 * term2,
                                '/' => term1 / term2,
                                _ => {
                                    println!("何？");
                                    panic!("お前はもう死んでる！")
                                }
                            };

                            results.insert(monkey.name.clone(), insert);

                            false
                        }
                        _ => { true }
                    }
                }
            }
        });
    }

    results
}

fn parse_lines(input: &Vec<String>) -> Vec<Monkey> {
    input.iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    value: Value,
}

#[derive(Debug, Clone)]
enum Value {
    Single(i64),
    Operation(String, char, String),
}

impl Value {
    fn as_operation(&self) -> (String, char, String) {
        match self {
            Operation(term1, op, term2) => (term1.clone(), *op, term2.clone()),
            _ => panic!("Not an operation")
        }
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.trim().split(" ").collect::<Vec<_>>();
        if split.len() == 1 {
            Ok(Single(split[0].parse().unwrap()))
        } else {
            Ok(Operation(
                split[0].to_string(),
                split[1].chars().next().unwrap(),
                split[2].to_string(),
            ))
        }
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");
        let name = split.next().unwrap().to_string();
        let value = split.next().unwrap().parse().unwrap();
        Ok(Monkey { name, value })
    }
}
