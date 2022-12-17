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
        1514285714288
    }

    fn solve(&self, input: &Vec<String>) -> u64 {
        let vec = parse_input(input);
        play(vec, 1000000000000)
    }
}

fn play(directions: Vec<Point>, steps: u64) -> u64 {
    let mut direction_index = 0;
    let mut next_direction = move || {
        let result = directions[direction_index].clone();
        direction_index = (direction_index + 1) % directions.len();
        result
    };

    let rocks = vec![RockShape::a(), RockShape::b(), RockShape::c(), RockShape::d(), RockShape::e()];
    let occupied_per_rock_cycle: u32 = rocks.iter().map(|r| r.points.len() as u32).sum();
    let n_rocks = rocks.len();
    let mut rock_index = 0;
    let mut next_rock = move || {
        let result = rocks[rock_index].clone();
        rock_index = (rock_index + 1) % rocks.len();
        result
    };

    let map = Map::new();

    let mut step = |search_cycles: bool| -> Option<usize> {
        if search_cycles {
            if let Some(cycle) = map.detect_cycle() {
                return Some(cycle);
            }
        }

        let rock = Rock::new(map.insertion_origin(), next_rock());

        loop {
            let direction = next_direction();
            let can_be_pushed = rock.shape.points.iter().map(|p| *p + *rock.position.borrow() + direction).all(|p| {
                !map.is_occupied(&p)
            });
            if can_be_pushed {
                rock.translate(direction);
            }

            let can_fall = rock.shape.points.iter().map(|p| *p + *rock.position.borrow() + Point::DOWN).all(|p| {
                !map.is_occupied(&p)
            });
            if can_fall {
                rock.translate(Point::DOWN);
            } else {
                rock.shape.points.iter().map(|p| *p + *rock.position.borrow()).for_each(|p| {
                    map.insert(p);
                });
                break;
            }
        }

        None
    };


    let maybe_cycle =
        (0..steps)
            .find_map(|s| step(true).map(|cycle_len| (s, cycle_len)));

    if let Some((n_steps, cycle_len)) = maybe_cycle {
        let remaining_steps = steps - n_steps;

        let occupied_in_last_rows = map.count_occupied_in_last_rows(cycle_len);
        let cycle_len = ((occupied_in_last_rows / occupied_per_rock_cycle) as u64) * n_rocks as u64;

        let residue = remaining_steps % cycle_len;

        for _ in 0..residue {
            step(false);
        }

        let y_before_cycle = *map.min_y.borrow();

        for _ in 0..cycle_len {
            step(false);
        }

        let y_after_cycle = *map.min_y.borrow();

        let y_per_cycle = y_after_cycle - y_before_cycle;


        let i = remaining_steps - residue;
        let remaining_cycles = ((i) / cycle_len) as i64;
        // println!(" occupied_in_last_rows     {}", occupied_in_last_rows);
        // println!(" occupied_per_rock_cycle   {}", occupied_per_rock_cycle);
        // println!(" steps_before_cycle_found  {}", n_steps);
        // println!(" cycle_len                 {}", cycle_len);
        // println!(" residue                   {}", residue);
        // println!(" y_before_cycle            {}", y_before_cycle);
        // println!(" y_after_cycle             {}", y_after_cycle);
        // println!(" remaining_cycles          {}", remaining_cycles);
        // println!(" y_per_cycle               {}", y_per_cycle);

        1 - (y_before_cycle + remaining_cycles * y_per_cycle) as u64
    } else {
        let r = *map.min_y.borrow();
        (1 - (r)) as u64
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
        p.x < 0 || p.x > 6 || p.y > 0 || self.raw.borrow().get(-p.y as usize).map(|u| u & (1 << (p.x as u8)) != 0).unwrap_or(false)
    }

    fn insertion_origin(&self) -> Point {
        p(2, *self.min_y.borrow() - 4)
    }

    fn insert(&self, p: Point) {
        let new_min_y = min(*self.min_y.borrow(), p.y);

        for _ in 0..-(new_min_y - *self.min_y.borrow()) {
            self.raw.borrow_mut().push(0);
        }

        self.min_y.replace(new_min_y);

        let i = &mut self.raw.borrow_mut()[-p.y as usize];
        *i = *i | (1 << p.x)
    }

    fn count_occupied_in_last_rows(&self, n: usize) -> u32 {
        self.raw.borrow().iter()
            .rev()
            .take(n)
            .map(|u| u.count_ones())
            .sum()
    }

    fn detect_cycle(&self) -> Option<usize> {
        let arr = self.raw.borrow();
        let y0 = arr.len() - 1;

        if arr.len() < 100 {
            return None;
        }

        'outer: for cycle_length in (5..(arr.len() / 4)).rev() {
            for i in 0..cycle_length {
                let i1 = arr[y0 - i];
                let i2 = arr[y0 - i - cycle_length];
                let i3 = arr[y0 - i - cycle_length * 2];
                // println!();
                // println!("{}", arr.len());
                // println!("{}", cycle_length);
                let i4 = arr[y0 - i - cycle_length * 3];

                if i1 != i2 || i2 != i3 || i3 != i4 {
                    continue 'outer;
                }
            }

            return Some(cycle_length);
        }

        None
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

    fn translate(&self, d: Point) {
        let new_position = *self.position.borrow() + d;
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
