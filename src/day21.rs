use std::collections::{HashMap, HashSet};
use std::ops::Not;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::thread;

use crate::day21::Value::{Operation, Single};
use crate::harness::{Day, Part};

pub fn day21() -> Day<i128, i128> {
    Day::new(21, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i128> for Part1 {
    fn expect_test(&self) -> i128 {
        152
    }

    fn solve(&self, input: &Vec<String>) -> i128 {
        let monkeys = parse_lines(input);
        let (_, result) = solve_monkeys(monkeys);

        *result.get("root").unwrap()
    }
}

pub struct Part2;

impl Part<i128> for Part2 {
    fn expect_test(&self) -> i128 {
        0
    }

    fn solve(&self, input: &Vec<String>) -> i128 {
        let mut monkeys = parse_lines(input);

        let mut vec = vec![];
        let mut set = HashSet::new();

        for x in &monkeys {
            match &x.value {
                Operation(s1, _, s2) => {
                    vec.push(s1.clone());
                    vec.push(s2.clone());
                    set.insert(s1.clone());
                    set.insert(s2.clone());
                }
                _ => (),
            }
        }

        monkeys.retain(|monkey| monkey.name != "humn");

        let (open, result) = solve_monkeys(monkeys);

        P2Solver::new(open, result).solve()
    }
}

struct P2Solver {
    open: Vec<Monkey>,
    closed: HashMap<String, i128>,
}

impl P2Solver {
    fn find_open(&self, name: &str) -> &Monkey {
        self.open.iter().find(|m| m.name == name).expect(&format!("Monkey {} not found", name))
    }

    fn stringify(&self, monkey: &Monkey) -> String {
        let do_the_thing = |term| {
            if term == "humn" {
                "x".to_string()
            } else {
                self.closed.get(term).map(|i| i.to_string()).unwrap_or_else(|| self.stringify(self.find_open(term)))
            }
        };

        match monkey {
            Monkey { name, value: Operation(term1, mut op, term2) } => {
                if name == "root" {
                    op = '='
                }

                let left = do_the_thing(term1);
                let right = do_the_thing(term2);

                format!("({} {} {})", left, op, right)
            }
            _ => panic!()
        }
    }

    fn solve(&self) -> i128 {
        for x in &self.open {
            println!("{:?}", x);
        }

        let x = self.stringify(self.find_open("root"));


        println!("{}", x);

        0
    }
}

impl P2Solver {
    pub fn new(open: Vec<Monkey>, closed: HashMap<String, i128>) -> Self {
        Self { open, closed }
    }
}

fn solve_monkeys(monkeys: Vec<Monkey>) -> (Vec<Monkey>, HashMap<String, i128>) {
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

#[derive(Debug, Clone)]
enum Value {
    Single(i128),
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
