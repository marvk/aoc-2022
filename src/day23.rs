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

        result.count()
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

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.extremes();

        let mut result = String::new();

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if let None = self.raw.borrow().get(&p(x, y)) {
                    result.push('.');
                } else {
                    result.push('#');
                }
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
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

    pub fn count(&self) -> u32 {
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

    pub fn spread(&self, max: usize) -> usize {
        // println!("{:?}", self);
        // println!("---");
        let mut current = self.raw.replace(HashSet::new());

        let mut considerations = build_initial_considerations();

        let mut considered_moves: HashMap<Point, Point> = HashMap::new();
        let mut considered_targets = HashSet::new();
        let mut blocked_moves: HashSet<Point> = HashSet::new();
        let mut static_elves = HashSet::new();


        let mut i = 0;
        loop {
            i+=1;
            if (i - 1) == max {
                break;
            }
            // println!("FIRST: {:?}", considerations.front().unwrap().direction);

            for &elf in &current {
                let considered_considerations = considerations.iter()
                    .filter(|c|
                        c.checks.iter()
                            .all(|&d| !current.contains(&(elf + d)))
                    )
                    .collect::<Vec<_>>();

                if considered_considerations.len() == 4 {
                    static_elves.insert(elf);
                    continue;
                }

                if let Some(consideration) = considered_considerations.first() {
                    let target = elf + consideration.direction;

                    if considered_targets.contains(&target) {
                        blocked_moves.insert(target);
                    } else {
                        considered_targets.insert(target);
                    }
                    considered_moves.insert(elf, target);
                } else {
                    static_elves.insert(elf);
                }
            }

            // for x in &considered_moves {
            //     println!("{:?}", x);
            // }
            // println!("---");
            //
            // for x in &blocked_moves {
            //     println!("{:?}", x);
            // }
            // println!("---");

            if considered_targets.is_empty() {
                break;
            }

            current.clear();


            for (&from, &to) in &considered_moves {
                if blocked_moves.contains(&to) {
                    current.insert(from);
                } else {
                    current.insert(to);
                }
            }
            for &x in &static_elves {
                current.insert(x);
            }

            // for x in &current {
            //     println!("{:?}", x);
            // }

            static_elves.clear();
            considered_targets.clear();
            considered_moves.clear();
            blocked_moves.clear();
            let front = considerations.pop_front().unwrap();
            considerations.push_back(front);
            self.raw.replace(current);

            // println!("---");
            // println!("{:?}", self);

            current = self.raw.replace(HashSet::new())
        }

        self.raw.replace(current);

        i
    }
}

fn build_initial_considerations() -> VecDeque<Consideration> {
    let mut result = VecDeque::new();
    result.push_back(Consideration::new(Point::NORTH, [Point::NORTH, Point::NORTH_EAST, Point::NORTH_WEST]));
    result.push_back(Consideration::new(Point::SOUTH, [Point::SOUTH, Point::SOUTH_EAST, Point::SOUTH_WEST]));
    result.push_back(Consideration::new(Point::WEST, [Point::WEST, Point::NORTH_WEST, Point::SOUTH_WEST]));
    result.push_back(Consideration::new(Point::EAST, [Point::EAST, Point::NORTH_EAST, Point::SOUTH_EAST]));
    result
}

struct Consideration {
    direction: Point,
    checks: [Point; 3],
}

impl Consideration {
    pub fn new(direction: Point, checks: [Point; 3]) -> Self {
        Self { direction, checks }
    }
}

impl From<&Vec<String>> for Map {
    fn from(value: &Vec<String>) -> Self {
        let raw = value.iter()
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

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        p(self.x * rhs, self.y * rhs)
    }
}


impl Div<i32> for Point {
    type Output = Point;

    fn div(self, rhs: i32) -> Self::Output {
        p(self.x / rhs, self.y / rhs)
    }
}

const ORTHOGONAL_DIRECTIONS: [Point; 4] = [Point::NORTH, Point::EAST, Point::SOUTH, Point::WEST];
const CARDINAL_DIRECTIONS: [Point; 8] = [Point::NORTH, Point::NORTH_EAST, Point::EAST, Point::SOUTH_EAST, Point::SOUTH, Point::SOUTH_WEST, Point::WEST, Point::NORTH_WEST];

impl Point {
    const ZERO: Self = p(0, 0);
    const NORTH: Self = p(0, -1);
    const NORTH_EAST: Self = p(1, -1);
    const EAST: Self = p(1, 0);
    const SOUTH_EAST: Self = p(1, 1);
    const SOUTH: Self = p(0, 1);
    const SOUTH_WEST: Self = p(-1, 1);
    const WEST: Self = p(-1, 0);
    const NORTH_WEST: Self = p(-1, -1);
}
