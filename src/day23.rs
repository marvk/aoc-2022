use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

use crate::harness::{Day, Part};

pub fn day23() -> Day<u32, u32> {
    Day::new(23, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        110
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let result = Map::from(input);

        result.spread(10);

        result.count_empty()
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        20
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let result = Map::from(input);

        result.spread(usize::MAX) as u32
    }
}

struct Map {
    raw: RefCell<HashSet<Point>>,
}

impl Map {
    pub fn new(raw: HashSet<Point>) -> Self {
        Self { raw: RefCell::new(raw) }
    }

    fn extremes(&self) -> (Point, Point) {
        let min_x = self.raw.borrow().iter().map(|p| p.x).min().unwrap();
        let min_y = self.raw.borrow().iter().map(|p| p.y).min().unwrap();
        let max_x = self.raw.borrow().iter().map(|p| p.x).max().unwrap();
        let max_y = self.raw.borrow().iter().map(|p| p.y).max().unwrap();

        (p(min_x, min_y), p(max_x, max_y))
    }

    pub fn count_empty(&self) -> u32 {
        let (min, max) = self.extremes();

        let mut sum = 0;

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if let None = self.raw.borrow().get(&p(x, y)) {
                    sum += 1;
                }
            }
        }

        sum
    }

    pub fn spread(&self, max_iterations: usize) -> usize {
        let mut current = self.raw.replace(HashSet::new());

        let mut proposals = build_initial_proposals();

        let mut moves = HashMap::new();
        let mut targets = HashSet::new();
        let mut blocked_targets = HashSet::new();

        let mut i = 0;
        loop {
            if i == max_iterations {
                break;
            }

            i += 1;

            for &elf in &current {
                let considered_proposals =
                    proposals.iter()
                        .filter(|c|
                            c.checks.iter()
                                .all(|&d| !current.contains(&(elf + d)))
                        )
                        .collect::<Vec<_>>();

                if considered_proposals.len() == 4 {
                    moves.insert(elf, elf);
                    continue;
                }

                if let Some(proposal) = considered_proposals.first() {
                    let target = elf + proposal.direction;

                    if targets.contains(&target) {
                        blocked_targets.insert(target);
                    } else {
                        targets.insert(target);
                    }
                    moves.insert(elf, target);
                } else {
                    moves.insert(elf, elf);
                }
            }

            if targets.is_empty() {
                break;
            }

            current.clear();

            for (from, to) in &moves {
                if blocked_targets.contains(to) {
                    current.insert(*from);
                } else {
                    current.insert(*to);
                }
            }

            moves.clear();
            targets.clear();
            blocked_targets.clear();

            let front = proposals.pop_front().unwrap();
            proposals.push_back(front);
        }

        self.raw.replace(current);

        i
    }
}

fn build_initial_proposals() -> VecDeque<Proposal> {
    let mut result = VecDeque::new();
    result.push_back(Proposal::new(Point::NORTH, [Point::NORTH, Point::NORTH_EAST, Point::NORTH_WEST]));
    result.push_back(Proposal::new(Point::SOUTH, [Point::SOUTH, Point::SOUTH_EAST, Point::SOUTH_WEST]));
    result.push_back(Proposal::new(Point::WEST, [Point::WEST, Point::NORTH_WEST, Point::SOUTH_WEST]));
    result.push_back(Proposal::new(Point::EAST, [Point::EAST, Point::NORTH_EAST, Point::SOUTH_EAST]));
    result
}

struct Proposal {
    direction: Point,
    checks: [Point; 3],
}

impl Proposal {
    pub fn new(direction: Point, checks: [Point; 3]) -> Self {
        Self { direction, checks }
    }
}

impl From<&Vec<String>> for Map {
    fn from(value: &Vec<String>) -> Self {
        let raw =
            value.iter()
                .filter(|line| !line.is_empty())
                .enumerate()
                .flat_map(|(y, row)|
                    row.chars()
                        .enumerate()
                        .filter(|&(_, cell)| cell == '#')
                        .map(move |(x, _)| (x, y)))
                .map(|(x, y)| p(x as i32, y as i32))
                .collect();

        Map::new(raw)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

const fn p(x: i32, y: i32) -> Point {
    Point { x, y }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        p(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Point {
    const NORTH: Self = p(0, -1);
    const NORTH_EAST: Self = p(1, -1);
    const EAST: Self = p(1, 0);
    const SOUTH_EAST: Self = p(1, 1);
    const SOUTH: Self = p(0, 1);
    const SOUTH_WEST: Self = p(-1, 1);
    const WEST: Self = p(-1, 0);
    const NORTH_WEST: Self = p(-1, -1);
}
