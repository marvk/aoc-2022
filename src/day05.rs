use std::collections::vec_deque::VecDeque;

use crate::harness::{Day, Part};

pub fn day05() -> Day<String, String> {
    Day::new(5, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<String> for Part1 {
    fn expect_test(&self) -> String {
        "CMZ".to_string()
    }

    fn solve(&self, input: &Vec<String>) -> String {
        let (mut stacks, b) = parse(input);

        for CraneMove { amount, from, to } in b {
            for _ in 0..amount {
                let from = stacks[from - 1].pop_back().unwrap();
                stacks[to - 1].push_back(from);
            }
        }

        stacks.into_iter().map(|s| *s.back().unwrap()).collect::<String>()
    }
}

pub struct Part2;

impl Part<String> for Part2 {
    fn expect_test(&self) -> String {
        "MCD".to_string()
    }

    fn solve(&self, input: &Vec<String>) -> String {
        let (mut stacks, b) = parse(input);

        for CraneMove { amount, from, to } in b {
            let mut buffer = VecDeque::new();

            for _ in 0..amount {
                buffer.push_front(stacks[from - 1].pop_back().unwrap());
            }

            for ch in buffer {
                stacks[to - 1].push_back(ch);
            }
        }

        stacks.into_iter().map(|s| *s.back().unwrap()).collect::<String>()
    }
}

fn parse(input: &Vec<String>) -> (Vec<VecDeque<char>>, Vec<CraneMove>) {
    let x = input.split(|line| line.is_empty()).collect::<Vec<_>>();
    let stacks = x.get(0).unwrap();
    let moves = x.get(1).unwrap();

    (parse_stacks(stacks), parse_moves(moves))
}

#[derive(Debug)]
struct CraneMove {
    amount: usize,
    from: usize,
    to: usize,
}

impl From<&String> for CraneMove {
    fn from(value: &String) -> Self {
        let split = value.split(" ").collect::<Vec<_>>();
        CraneMove::new(
            split[1].parse::<usize>().unwrap(),
            split[3].parse::<usize>().unwrap(),
            split[5].parse::<usize>().unwrap(),
        )
    }
}

impl CraneMove {
    pub fn new(amount: usize, from: usize, to: usize) -> Self {
        Self { amount, from, to }
    }
}

fn parse_stacks(input: &[String]) -> Vec<VecDeque<char>> {
    let x: &String = input.last().unwrap();
    let number_of_stacks = (x.len() as f32 / 4.0).ceil() as usize;

    let mut result = vec![];

    for _ in 0..number_of_stacks {
        result.push(VecDeque::new());
    }

    let fold_into = |mut acc: Vec<VecDeque<char>>, item: &String| -> Vec<VecDeque<char>> {
        let chars = item.chars().collect::<Vec<_>>();

        for i in 0..number_of_stacks {
            let char_index = (i) * 4 + 1;

            if let Some(ch) = chars.get(char_index) {
                if ch.is_alphabetic() {
                    acc[i].push_back(*ch);
                }
            }
        }

        acc
    };


    input.iter().rev().skip(1).fold(result, fold_into)
}


fn parse_moves(input: &[String]) -> Vec<CraneMove> {
    input.iter().map(CraneMove::from).collect::<Vec<_>>()
}


