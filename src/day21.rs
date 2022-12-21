use std::collections::{HashMap, HashSet};
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

        let (_, closed) = solve_monkeys(monkeys);

        *closed.get("root").unwrap()
    }
}

pub struct Part2;

impl Part<i64> for Part2 {
    fn expect_test(&self) -> i64 {
        301
    }

    fn solve(&self, input: &Vec<String>) -> i64 {
        let mut monkeys = parse_lines(input);
        monkeys.retain(|monkey| monkey.name != "humn");

        let (open, result) = solve_monkeys(monkeys);

        Part2Solver::new(open, result).solve()
    }
}

struct Part2Solver {
    open: Vec<Monkey>,
    closed: HashMap<String, i64>,
}

impl Part2Solver {
    pub fn new(open: Vec<Monkey>, closed: HashMap<String, i64>) -> Self {
        Self { open, closed }
    }

    fn solve(&self) -> i64 {
        self.solvify(0, self.open.iter().find(|m| m.name == "root"))
    }

    fn solvify(&self, a: i64, monkey: Option<&Monkey>) -> i64 {
        match monkey {
            Some(Monkey { name, value: Operation(term1, op, term2) }) => {
                let left = self.closed.get(term1);
                let right = self.closed.get(term2);

                let op = if name == "root" { '=' } else { *op };

                let (new_monkey_name, new_a) = match (left, right) {
                    (Some(&c), None) => {
                        (term2, match op {
                            '+' => a - c,
                            '-' => c - a,
                            '*' => a / c,
                            '/' => c / a,
                            '=' => c,
                            _ => panic!()
                        })
                    }
                    (None, Some(&c)) => {
                        (term1, match op {
                            '+' => a - c,
                            '-' => a + c,
                            '*' => a / c,
                            '/' => a * c,
                            '=' => c,
                            _ => panic!()
                        })
                    }
                    _ => panic!()
                };

                self.solvify(new_a, self.open.iter().find(|m| m.name == *new_monkey_name))
            }
            _ => a
        }
    }
}

fn solve_monkeys(monkeys: Vec<Monkey>) -> (Vec<Monkey>, HashMap<String, i64>) {
    let mut open = monkeys;
    let mut closed = HashMap::new();

    while open.is_empty().not() {
        let before_len = open.len();

        open.retain(|monkey| {
            match &monkey.value {
                Single(value) => {
                    closed.insert(monkey.name.clone(), *value);
                    false
                }
                Operation(term1, op, term2) => {
                    match (closed.get(term1), closed.get(term2)) {
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

                            closed.insert(monkey.name.clone(), insert);

                            false
                        }
                        _ => true
                    }
                }
            }
        });

        if open.len() == before_len {
            break;
        }
    }

    (open, closed)
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

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");
        let name = split.next().unwrap().to_string();
        let value = split.next().unwrap().parse().unwrap();
        Ok(Monkey { name, value })
    }
}


#[derive(Debug, Clone)]
enum Value {
    Single(i64),
    Operation(String, char, String),
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
