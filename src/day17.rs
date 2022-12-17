use std::cell::RefCell;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Neg, Sub};
use std::usize;

use crate::harness::{Day, Part};

pub fn day17() -> Day<u32, u64> {
    Day::new(17, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        3068
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let vec = parse_input(input);
        play(vec, 2022) as u32
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        1_514_285_714_288
    }

    fn solve(&self, input: &Vec<String>) -> u64 {
        let vec = parse_input(input);
        play(vec, 1_000_000_000_000)
    }
}

fn play(directions: Vec<Point>, steps: u64) -> u64 {
    let mut next_direction = {
        let mut direction_index = 0;
        move || {
            let result = directions[direction_index].clone();
            direction_index = (direction_index + 1) % directions.len();
            result
        }
    };

    let rocks = vec![RockShape::a(), RockShape::b(), RockShape::c(), RockShape::d(), RockShape::e()];
    let occupied_per_rock_cycle = rocks.iter().map(|r| r.points.len()).sum::<usize>() as u64;
    let n_rocks = rocks.len() as u64;
    let mut next_rock = {
        let mut rock_index = 0;
        move || {
            let result = rocks[rock_index].clone();
            rock_index = (rock_index + 1) % rocks.len();
            result
        }
    };

    let map = Map::new();

    let mut step = || {
        let rock = Rock::new(map.insertion_origin(), next_rock());

        loop {
            let direction = next_direction();
            if map.can_accommodate_rock_with_offset(&rock, &direction) {
                rock.translate(&direction);
            }

            if map.can_accommodate_rock_with_offset(&rock, &Point::DOWN) {
                rock.translate(&Point::DOWN);
            } else {
                map.insert_rock(rock);
                return;
            }
        }
    };

    let maybe_cycle =
        (0..steps)
            .find_map(|s| {
                // Checking for a cycle is expensive, so only do it every few hundred steps
                if s % 1000 == 0 {
                    if let Some(cycle) = map.find_cycle() {
                        return Some((s, cycle));
                    }
                }
                step();
                None
            });

    if let Some((steps_taken, cycle_height)) = maybe_cycle {
        let occupied_in_last_rows = map.count_occupied_in_last_rows(cycle_height);
        let cycle_len = n_rocks * occupied_in_last_rows / occupied_per_rock_cycle;

        let remaining_steps = steps - steps_taken;
        let residual_steps = remaining_steps % cycle_len;
        let remaining_cycles = remaining_steps / cycle_len;

        for _ in 0..residual_steps { step() }

        let y_per_cycle = cycle_height as u64;

        remaining_cycles * y_per_cycle + map.height()
    } else {
        // No cycle found, just calculate the result
        map.height()
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Point> {
    input
        .first()
        .unwrap()
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Point::LEFT,
            '>' => Point::RIGHT,
            _ => panic!(),
        })
        .collect()
}

struct Map {
    raw: RefCell<Vec<u8>>,
    min_y: RefCell<i64>,
}

impl Map {
    pub fn new() -> Self {
        Self { raw: RefCell::new(vec![]), min_y: RefCell::new(1) }
    }

    fn is_occupied(&self, p: &Point) -> bool {
        p.x < 0 || p.x > 6 || p.y > 0 || self.is_raw_occupied(p)
    }

    fn is_raw_occupied(&self, p: &Point) -> bool {
        self.raw.borrow().get(-p.y as usize)
            .map(|u| u & (1 << (p.x)) != 0)
            .unwrap_or(false)
    }

    fn can_accommodate_rock_with_offset(&self, rock: &Rock, offset: &Point) -> bool {
        rock.shape.points.iter()
            .map(|p| *p + *rock.position.borrow() + *offset)
            .all(|p| !self.is_occupied(&p))
    }

    fn insertion_origin(&self) -> Point {
        p(2, *self.min_y.borrow() - 4)
    }

    fn insert_rock(&self, rock: Rock) {
        rock.shape.points.iter()
            .map(|p| *p + *rock.position.borrow())
            .for_each(|p| self.insert(p));
    }

    fn insert(&self, p: Point) {
        let new_min_y = min(*self.min_y.borrow(), p.y);

        self.insert_blank_rows(*self.min_y.borrow() - new_min_y);
        self.insert_raw(p);

        self.min_y.replace(new_min_y);
    }

    fn insert_blank_rows(&self, n: i64) {
        for _ in 0..n {
            self.raw.borrow_mut().push(0);
        }
    }

    fn insert_raw(&self, p: Point) {
        let row = &mut self.raw.borrow_mut()[-p.y as usize];
        *row = *row | (1 << p.x)
    }

    fn count_occupied_in_last_rows(&self, n: usize) -> u64 {
        self.raw.borrow().iter()
            .rev()
            .take(n)
            .map(|u| u.count_ones() as u64)
            .sum()
    }

    fn find_cycle(&self) -> Option<usize> {
        let arr = self.raw.borrow();
        let y0 = arr.len() - 1;

        'outer: for cycle_length in (5..(y0 / 3)).rev() {
            for i in 0..cycle_length {
                let i1 = arr.get(y0 - i)?;
                let i2 = arr.get(y0 - i - cycle_length)?;
                let i3 = arr.get(y0 - i - cycle_length * 2)?;

                if i1 != i2 || i2 != i3 {
                    continue 'outer;
                }
            }

            return Some(cycle_length);
        }

        None
    }

    fn height(&self) -> u64 {
        1 - *self.min_y.borrow() as u64
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for y in (*self.min_y.borrow() - 3)..=0 {
            for x in -1..=7 {
                if x == -1 || x == 7 {
                    result.push('|');
                } else if self.is_occupied(&p(x, y)) {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }

        result.push_str("+-------+");

        write!(f, "{}", result)
    }
}

#[derive(Clone)]
struct RockShape {
    points: Vec<Point>,
    width: i64,
}

impl RockShape {
    pub fn new(points: Vec<Point>) -> Self {
        let width = points.iter().map(|p| p.y).max().unwrap();

        Self { points, width }
    }

    fn a() -> Self { RockShape::new(vec![p(0, 0), p(1, 0), p(2, 0), p(3, 0)]) }
    fn b() -> Self { RockShape::new(vec![p(1, 0), p(0, -1), p(1, -1), p(2, -1), p(1, -2)]) }
    fn c() -> Self { RockShape::new(vec![p(0, 0), p(1, 0), p(2, 0), p(2, -1), p(2, -2)]) }
    fn d() -> Self { RockShape::new(vec![p(0, 0), p(0, -1), p(0, -2), p(0, -3)]) }
    fn e() -> Self { RockShape::new(vec![p(0, 0), p(1, 0), p(0, -1), p(1, -1)]) }
}

#[derive(Clone)]
struct Rock {
    position: RefCell<Point>,
    shape: RockShape,
}

impl Rock {
    pub fn new(position: Point, shape: RockShape) -> Self {
        Self { position: RefCell::new(position), shape }
    }

    fn width(&self) -> i64 {
        self.shape.width
    }

    fn min_x(&self) -> i64 {
        self.position.borrow().x
    }

    fn max_x(&self) -> i64 {
        self.position.borrow().x + self.width() - 1
    }

    fn translate(&self, d: &Point) {
        let new_position = *self.position.borrow() + *d;
        self.position.replace(new_position);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

const fn p(x: i64, y: i64) -> Point {
    Point { x, y }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        p(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        p(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        p(-self.x, -self.y)
    }
}

impl Point {
    fn manhattan_distance(&self, rhs: &Self) -> i64 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    fn swap(&self) -> Self {
        p(self.y, self.x)
    }

    const LEFT: Self = p(-1, 0);
    const RIGHT: Self = p(1, 0);
    const DOWN: Self = p(0, 1);
}
