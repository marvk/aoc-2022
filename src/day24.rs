use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::rc::Rc;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use crate::harness::{Day, Part};

pub fn day24() -> Day<u32, u32> {
    Day::new(24, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        18
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let map = Map::from(input);

        let start = map.start();
        let goal = map.goal();

        let minutes = search_iter(&map, start, goal, 0).unwrap();

        minutes as u32
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        54
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let map = Map::from(input);

        let start = map.start();
        let goal = map.goal();

        let minutes = search_iter(&map, start, goal, 0).unwrap();
        let minutes = search_iter(&map, goal, start, minutes).unwrap();
        let minutes = search_iter(&map, start, goal, minutes).unwrap();

        minutes as u32
    }
}

const MOVEMENT_OPTIONS: [Point; 5] = [Point::EAST, Point::SOUTH, Point::ZERO, Point::NORTH, Point::WEST];

#[derive(PartialEq, Eq, Hash)]
struct SearchState {
    minutes_passed: usize,
    position: Point,
}

impl Debug for SearchState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, ({},{}))", self.minutes_passed, self.position.x, self.position.y)
    }
}

impl PartialOrd<Self> for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SearchState {
    pub fn new(minutes_passed: usize, position: Point) -> Self {
        Self { minutes_passed, position }
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        let x_order = self.position.x.cmp(&other.position.x);
        let y_order = self.position.y.cmp(&other.position.y);
        let time_order = self.minutes_passed.cmp(&other.minutes_passed).reverse();

        time_order.then(x_order).then(y_order)
    }
}

fn search_iter(map: &Map, start: Point, target: Point, minutes_passed: usize) -> Option<usize> {
    let mut open: BinaryHeap<SearchState> = BinaryHeap::new();
    let mut closed: HashSet<SearchState> = HashSet::new();

    open.push(SearchState::new(minutes_passed, start));

    let mut best: Option<usize> = None;

    while let Some(state) = open.pop() {
        let SearchState { minutes_passed, position } = state;

        if !closed.insert(state) {
            continue;
        }

        if let Some(best) = best {
            if minutes_passed >= best {
                continue;
            }
        }

        if position == target {
            best = Some(minutes_passed);
            continue;
        }

        let next_blizzard = map.blizzards_at(minutes_passed + 1);

        for direction in MOVEMENT_OPTIONS {
            let next_position = position + direction;
            if next_blizzard.contains(&next_position) {
                continue;
            }
            if map.is_wall(next_position) {
                continue;
            }
            open.push(SearchState::new(minutes_passed + 1, next_position));
        }
    }

    best
}

struct Map {
    raw: Vec<Vec<char>>,
    blizzards: RefCell<Vec<Rc<Vec<Blizzard>>>>,
    blizzards_set: RefCell<Vec<Rc<HashSet<Point>>>>,
    min: Point,
    max: Point,
}

impl Map {
    pub fn new(raw: Vec<Vec<char>>, blizzard: Vec<Blizzard>, min: Point, max: Point) -> Self {
        let blizzard_set: HashSet<Point> = blizzard.iter().map(|b| b.position).collect();

        Self {
            raw,
            blizzards: RefCell::new(vec![Rc::new(blizzard)]),
            blizzards_set: RefCell::new(vec![Rc::new(blizzard_set)]),
            min,
            max,
        }
    }

    fn start(&self) -> Point {
        self.raw[0].iter()
            .enumerate()
            .find(|(_, c)| **c == '.')
            .map(|(x, _)| p(x as i32, self.min.y))
            .unwrap()
    }

    fn goal(&self) -> Point {
        self.raw.last().unwrap().iter()
            .enumerate()
            .find(|(_, c)| **c == '.')
            .map(|(x, _)| p(x as i32, self.max.y))
            .unwrap()
    }

    fn is_wall(&self, position: Point) -> bool {
        if let Some('.') = self.raw.get(position.y as usize).map(|row| row.get(position.x as usize)).flatten() {
            false
        } else {
            true
        }
    }

    fn blizzards_at(&self, minute: usize) -> Rc<HashSet<Point>> {
        let n_blizzards = self.blizzards.borrow().len();
        for i in n_blizzards..=minute {
            let current_blizzard = self.step_blizzard(self.blizzards.borrow()[i - 1].as_ref());
            self.blizzards_set.borrow_mut().insert(i, Rc::new(current_blizzard.iter().map(|b| b.position).collect()));
            self.blizzards.borrow_mut().insert(i, Rc::new(current_blizzard));
        }

        self.blizzards_set.borrow()[minute].clone()
    }

    fn step_blizzard(&self, blizzards: &Vec<Blizzard>) -> Vec<Blizzard> {
        blizzards.iter()
            .copied()
            .map(|mut blizzard| {
                blizzard.position = match blizzard.position + blizzard.direction {
                    q if q.x <= self.min.x => p(self.max.x - 1, q.y),
                    q if q.x >= self.max.x => p(self.min.x + 1, q.y),
                    q if q.y <= self.min.y => p(q.x, self.max.y - 1),
                    q if q.y >= self.max.y => p(q.x, self.min.y + 1),
                    q => q,
                };

                blizzard
            })
            .collect()
    }
}

impl From<&Vec<String>> for Map {
    fn from(value: &Vec<String>) -> Self {
        let mut blizzard = vec![];

        let raw: Vec<Vec<_>> =
            value.iter()
                .filter(|line| !line.is_empty())
                .enumerate()
                .map(|(y, row)|
                    row.chars()
                        .enumerate()
                        .map(|(x, char)| {
                            let direction = match char {
                                '^' => Some(Point::NORTH),
                                '>' => Some(Point::EAST),
                                'v' => Some(Point::SOUTH),
                                '<' => Some(Point::WEST),
                                _ => None
                            };

                            if let Some(direction) = direction {
                                blizzard.push(Blizzard::new(p(x as i32, y as i32), direction));
                            }

                            direction.map(|_| '.').unwrap_or(char)
                        })
                        .collect()
                )
                .collect();

        let min = p(0, 0);
        let max = p(raw[0].len() as i32 - 1, raw.len() as i32 - 1);

        Map::new(raw, blizzard, min, max)
    }
}

#[derive(Copy, Clone, Debug)]
struct Blizzard {
    position: Point,
    direction: Point,
}

impl Blizzard {
    pub fn new(position: Point, direction: Point) -> Self {
        Self { position, direction }
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
    const ZERO: Self = p(0, 0);
    const NORTH: Self = p(0, -1);
    const EAST: Self = p(1, 0);
    const SOUTH: Self = p(0, 1);
    const WEST: Self = p(-1, 0);
}
