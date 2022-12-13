use std::cmp::{max, Ordering};
use std::fmt::{Display, Formatter};
use std::iter::Zip;
use std::slice::Iter;
use std::str::FromStr;

use crate::harness::{Day, Part};

pub fn day13() -> Day<u32, u32> {
    Day::new(13, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        13
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        parse_input(input)
            .chunks(2)
            .enumerate()
            .filter(|(_, pair)| pair[0] < pair[1])
            .map(|(index, _)| index as u32 + 1)
            .sum()
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        140
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let markers = [
            Data::from("[[2]]"),
            Data::from("[[6]]"),
        ];

        let mut sorted = parse_input(input);
        sorted.extend(markers.clone().into_iter());
        sorted.sort();

        markers
            .into_iter()
            .map(|marker|
                sorted
                    .iter()
                    .enumerate()
                    .find(|(_, data)| **data == marker)
                    .map(|(index, _)| index as u32 + 1)
                    .unwrap()
            )
            .product()
    }
}

fn parse_input(p0: &Vec<String>) -> Vec<Data> {
    p0
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| Data::from(line.as_str()))
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Data {
    List(Vec<Data>),
    Single(i32),
}

impl Data {
    fn push_child(&mut self, child: Data) {
        match self {
            Data::List(children) => children.push(child),
            _ => panic!("Can't push child into Single"),
        }
    }

    fn children(&self) -> Vec<Data> {
        match self {
            Data::List(children) => children.clone(),
            Data::Single(value) => vec![Data::Single(*value)],
        }
    }
}

impl PartialOrd<Self> for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Data::Single(left), Data::Single(right)) = (self, other) {
            return left.cmp(right);
        }

        let left_data = self.children();
        let right_data = other.children();

        for i in 0..max(left_data.len(), right_data.len()) {
            let potential_result = match (&left_data.get(i), &right_data.get(i)) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (Some(left), Some(right)) => left.cmp(right)
            };

            let Ordering::Equal = potential_result else {
                return potential_result;
            };
        }

        Ordering::Equal
    }
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        parse_chunk(&value[1..]).0
    }
}

fn parse_chunk(substring: &str) -> (Data, usize) {
    let mut result = Data::List(vec![]);

    let x: Vec<_> = substring.chars().collect();

    let mut current_number = "".to_string();

    let mut i = 0;
    while i < substring.len() {
        match x[i] {
            '[' => {
                let (data, offset) = parse_chunk(&substring[(i + 1)..]);

                i += offset;
                result.push_child(data);
            }
            ']' => {
                if let Ok(data) = current_number.parse().map(Data::Single) {
                    result.push_child(data);
                }

                return (result, i + 1);
            }
            d if d.is_ascii_digit() => {
                current_number.push(d);
            }
            ',' => {
                if let Ok(data) = current_number.parse().map(Data::Single) {
                    result.push_child(data);
                    current_number = "".to_string();
                }
            }
            _ => ()
        }

        i += 1;
    }

    panic!("Invalid packet")
}
