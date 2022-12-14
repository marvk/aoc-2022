use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops;

use crate::day14::Material::{Rock, Sand, Source};
use crate::harness::{Day, Part};

pub fn day14() -> Day<u32, u32> {
    Day::new(14, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        24
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let map = Map::from(input);
        Self::simulate(&map);
        map.count_sand()
    }
}

impl Part1 {
    fn simulate(map: &Map) {
        loop {
            let mut current = SOURCE;

            loop {
                if let Some(next) = DROP_DIRECTIONS.iter().map(|d| current + *d).find(|p| map.get(*p).is_none()) {
                    current = next;

                    if current.y > map.max.y {
                        return;
                    }
                } else {
                    map.insert_sand(current);
                    break;
                }
            }
        }
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        93
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let map = Map::from(input);
        Self::simulate(&map);
        map.count_sand()
    }
}

impl Part2 {
    fn simulate(map: &Map) {
        loop {
            let mut current = SOURCE;

            loop {
                if let Some(next) = DROP_DIRECTIONS.iter().map(|d| current + *d).find(|p| map.get(*p).is_none()) {
                    current = next;

                    if current.y > map.max.y {
                        map.insert_sand(current);
                        break;
                    }
                } else {
                    map.insert_sand(current);
                    if current == SOURCE {
                        return;
                    } else {
                        break
                    }
                }
            }
        }
    }
}

const DROP_DIRECTIONS: [Point; 3] = [p(0, 1), p(-1, 1), p(1, 1)];
const SOURCE: Point = p(500, 0);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

const fn p(x: i32, y: i32) -> Point {
    Point { x, y }
}

impl ops::Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        p(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        p(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Point {
    fn d(&self, other: Self) -> Self {
        let d = other - *self;

        p(d.x.signum(), d.y.signum())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Material {
    Rock,
    Sand,
    Source,
}

struct Map {
    raw: RefCell<HashMap<Point, Material>>,
    min: Point,
    max: Point,
}

impl Map {
    pub fn new(raw: HashMap<Point, Material>) -> Self {
        let (min, max) = find_extremes(&raw);

        Self {
            raw: RefCell::new(raw),
            min,
            max,
        }
    }

    fn get(&self, p: Point) -> Option<Material> {
        self.raw.borrow().get(&p).map(|m| *m)
    }

    fn insert_sand(&self, p: Point) {
        self.raw.borrow_mut().insert(p, Sand);
    }

    fn count_sand(&self) -> u32 {
        self.raw.borrow().iter().filter(|(_, v)| **v == Sand).count() as u32
    }
}

fn find_extremes(raw: &HashMap<Point, Material>) -> (Point, Point) {
    let min = p(
        raw.iter().map(|(k, _)| k.x).min().unwrap(),
        raw.iter().map(|(k, _)| k.y).min().unwrap(),
    );
    let max = p(
        raw.iter().map(|(k, _)| k.x).max().unwrap(),
        raw.iter().map(|(k, _)| k.y).max().unwrap(),
    );
    (min, max)
}

impl From<&Vec<String>> for Map {
    fn from(value: &Vec<String>) -> Self {
        let map_line = |line: &str| -> Vec<(Point, Material)> {
            let points =
                line
                    .split(" -> ")
                    .map(|chunk| chunk.split(","))
                    .map(|mut arr| p(
                        arr.next().unwrap().parse().unwrap(),
                        arr.next().unwrap().parse().unwrap(),
                    ))
                    .collect::<Vec<_>>();


            let mut current = points[0];
            let mut result = vec![current];

            for i in 1..points.len() {
                let next = points[i];

                let d = current.d(next);

                while current != next {
                    current = current + d;
                    result.push(current);
                }
            }

            result.into_iter().map(|p| (p, Rock)).collect()
        };

        let raw =
            value
                .iter()
                .filter(|line| !line.is_empty())
                .flat_map(|line| map_line(line).into_iter())
                .chain(vec![(SOURCE, Source)])
                .collect::<HashMap<_, _>>();

        Map::new(raw)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = "".to_string();

        let (min, max) = find_extremes(&self.raw.borrow());

        for y in min.y - 2..max.y + 2 {
            for x in min.x - 2..max.x + 2 {
                let char = match self.get(p(x, y)) {
                    None => ' ',
                    Some(Rock) => '#',
                    Some(Sand) => 'o',
                    Some(Source) => '+',
                };

                builder.push(char)
            }
            builder.push('\n')
        }

        write!(f, "{}", builder)
    }
}
