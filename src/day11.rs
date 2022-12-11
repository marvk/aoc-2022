use std::cell::RefCell;
use std::fmt::{Debug, Formatter};

use crate::harness::{Day, Part};

pub fn day11() -> Day<u128, u128> {
    Day::new(11, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u128> for Part1 {
    fn expect_test(&self) -> u128 {
        10605
    }

    fn solve(&self, input: &Vec<String>) -> u128 {
        play(20, parse(input, true))
    }
}

pub struct Part2;

impl Part<u128> for Part2 {
    fn expect_test(&self) -> u128 {
        2713310158
    }

    fn solve(&self, input: &Vec<String>) -> u128 {
        play(10000, parse(input, false))
    }
}

struct Monkey {
    inspections: RefCell<u128>,
    worry_levels: RefCell<Vec<u128>>,
    calculate_new_worry: Box<dyn Fn(u128) -> u128>,
    calculate_throw_to: Box<dyn Fn(u128) -> usize>,
}

impl Monkey {
    pub fn new(worry_levels: Vec<u128>, calculate_new_worry: Box<dyn Fn(u128) -> u128>, calculate_throw_to: Box<dyn Fn(u128) -> usize>) -> Self {
        Self {
            inspections: RefCell::new(0),
            worry_levels: RefCell::new(worry_levels),
            calculate_new_worry,
            calculate_throw_to,
        }
    }

    fn inspect(&self) {
        self.inspections.replace_with(|old| *old + self.worry_levels.borrow().len() as u128);

        let new_worry_levels = self.worry_levels.borrow().iter().map(|&it| (self.calculate_new_worry)(it)).collect::<Vec<u128>>();
        self.worry_levels.borrow_mut().clear();
        self.worry_levels.borrow_mut().extend(&new_worry_levels);
    }

    fn receive(&self, item: u128) {
        self.worry_levels.borrow_mut().push(item);
    }

    fn throw_items(&self) -> Vec<(u128, usize)> {
        let result = self.worry_levels.borrow().iter().map(|&worry| (worry, (self.calculate_throw_to)(worry))).collect::<Vec<_>>();
        self.worry_levels.borrow_mut().clear();
        result
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("inspections", &self.inspections.borrow())
            .field("worry_levels", &self.worry_levels.borrow())
            .finish()
    }
}

fn parse(input: &Vec<String>, part_1: bool) -> Vec<Monkey> {
    input
        .chunks(7)
        .map(|it| parse_monkey(it, part_1))
        .collect::<Vec<_>>()
}

fn play(rounds: usize, monkeys: Vec<Monkey>) -> u128 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            monkey.inspect();
            for (item, to_monkey) in monkey.throw_items() {
                monkeys[to_monkey].receive(item);
            }
        }
    }

    calculate_monkey_business(&monkeys)
}

fn calculate_monkey_business(monkeys: &Vec<Monkey>) -> u128 {
    let mut inspections = monkeys.iter().map(|monkey| *monkey.inspections.borrow()).collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

fn parse_monkey(lines: &[String], part_1: bool) -> Monkey {
    let worry_levels =
        lines[1]
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(", ")
            .map(|it| it.parse().unwrap())
            .collect::<Vec<u128>>();

    Monkey::new(
        worry_levels,
        parse_calculate_new_worry(lines, part_1),
        parse_calculate_throw_to(lines),
    )
}

fn parse_calculate_new_worry(lines: &[String], part_1: bool) -> Box<dyn Fn(u128) -> u128> {
    let operation_raw =
        lines[2]
            .split("=")
            .nth(1)
            .unwrap()
            .trim()
            .split(" ")
            .collect::<Vec<&str>>();

    let operation = operation_raw[1].chars().next().unwrap();
    let operand: Option<u128> = operation_raw[2].parse().ok();

    Box::new(move |old| {
        let other = operand.unwrap_or(old);
        let r = match operation {
            '*' => old * other,
            '+' => old + other,
            _ => panic!(),
        };
        if part_1 {
            r / 3
        } else {
            r % 223092870
        }
    })
}

fn parse_calculate_throw_to(lines: &[String]) -> Box<dyn Fn(u128) -> usize> {
    let modulo: u128 = lines[3].split(" ").last().unwrap().parse().unwrap();
    let divisible_index: usize = lines[4].split(" ").last().unwrap().parse().unwrap();
    let not_divisible_index: usize = lines[5].split(" ").last().unwrap().parse().unwrap();

    Box::new(move |worry|
        match worry % modulo {
            0 => divisible_index,
            _ => not_divisible_index,
        }
    )
}
