use std::cmp::max;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::ops::Not;

use crate::harness::{Day, Part};

pub fn day09() -> Day<u32, u32> {
    Day::new(9, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        13
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        solve(input, 2)
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        1
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        solve(input, 10)
    }
}

fn solve(input: &Vec<String>, n: usize) -> u32 {
    plot(n, &parse(input)).len() as u32
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub const NORTH: Self = p(0, -1);
    pub const EAST: Self = p(1, 0);
    pub const SOUTH: Self = p(0, 1);
    pub const WEST: Self = p(-1, 0);

    pub const ZERO: Self = p(0, 0);

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
    pub const fn sub(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

const fn p(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

fn parse(input: &Vec<String>) -> Vec<Point> {
    input.iter()
        .filter(|line| line.is_empty().not())
        .map(|line| parse_line(line))
        .flat_map(|(direction, count)| vec![direction; count].into_iter())
        .collect::<Vec<_>>()
}

fn parse_line(line: &String) -> (Point, usize) {
    let vec = line.split(" ").collect::<Vec<_>>();
    let direction = match vec[0] {
        "U" => Point::NORTH,
        "R" => Point::EAST,
        "D" => Point::SOUTH,
        "L" => Point::WEST,
        _ => panic!(),
    };

    (direction, vec[1].parse().unwrap())
}


fn plot(n: usize, directions: &Vec<Point>) -> HashSet<Point> {
    let start = p(0, 0);
    let mut segments = vec![start; n];

    let mut visited = vec![start].into_iter().collect::<HashSet<_>>();
    for x in directions {
        segments[0] = segments[0].add(&x);

        for i in 1..segments.len() {
            let head = segments[i - 1];
            let tail = segments[i];

            let distance = head.sub(&tail);

            let new_distance = dx(distance).add(&dy(&distance));

            if new_distance != Point::ZERO {
                segments[i] = head.add(&new_distance);
            }
        }

        segments = segments;

        visited.insert(*segments.last().unwrap());
    }

    visited
}

fn dy(distance: &Point) -> Point {
    match distance.y {
        2 => Point::NORTH,
        -2 => Point::SOUTH,
        _ => Point::ZERO,
    }
}

fn dx(distance: Point) -> Point {
    match distance.x {
        2 => Point::WEST,
        -2 => Point::EAST,
        _ => Point::ZERO,
    }
}
