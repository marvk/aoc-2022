use std::collections::HashSet;
use std::ops;
use std::ops::{Add, Neg, Sub};

use crate::harness::{Day, Part};

pub fn day15() -> Day<u32, u64> {
    Day::new(15, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        26
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        // Smh this should be puzzle input >:(
        let line = if input.len() < 20 { 10_i32 } else { 2000000 };

        let sensors = parse_sensors(input);

        let manhattan_max = sensors.iter().map(|s| s.manhattan_radius).max().unwrap() as i32;
        let min_x = sensors.iter().map(|s| s.sensor_location.x).min().unwrap() - manhattan_max;
        let max_x = sensors.iter().map(|s| s.sensor_location.x).max().unwrap() + manhattan_max;

        (min_x..=max_x)
            .map(|x| p(x, line))
            .filter(|&p|
                sensors
                    .iter()
                    .any(|sensor| !sensor.collides(&p) && sensor.could_sense(&p))
            )
            .count() as u32
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        56000011
    }

    fn solve(&self, input: &Vec<String>) -> u64 {
        // >:(
        let range_max = if input.len() < 20 { 20 } else { 4000000 };

        let range = 0..=range_max;
        let sensors = parse_sensors(input);

        let find_solution = |sensor: &Sensor| {
            let dist = sensor.manhattan_radius + 1;

            (0..dist)
                .map(|i| p(i, i - dist))
                .flat_map(|p| [p, -p, p.swap(), -p.swap()])
                .map(|p| sensor.sensor_location.add(p))
                .filter(|p| range.contains(&p.x) && range.contains(&p.y))
                .filter(|p| sensors.iter().all(|sensor| !sensor.could_sense(p)))
                .map(|p| p.x as u64 * 4000000_u64 + p.y as u64)
                .next()
        };

        sensors.iter()
            .find_map(find_solution)
            .expect("No solution found!")
    }
}

fn parse_sensors(input: &Vec<String>) -> Vec<Sensor> {
    input.iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_str())
        .map(Sensor::from)
        .collect::<Vec<_>>()
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

impl Point {
    fn manhattan_distance(&self, rhs: &Self) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    fn swap(&self) -> Self {
        p(self.y, self.x)
    }
}


#[derive(Debug)]
struct Sensor {
    sensor_location: Point,
    beacon_location: Point,
    manhattan_radius: i32,
}

impl From<&str> for Sensor {
    fn from(value: &str) -> Self {
        let coords =
            value
                .replace(":", "")
                .replace(",", "")
                .split(" ")
                .flat_map(|s| s.split("="))
                .map(|segment| segment.parse::<i32>())
                .filter(|result| result.is_ok())
                .map(|result| result.unwrap())
                .collect::<Vec<_>>();

        Sensor::new(p(coords[0], coords[1]), p(coords[2], coords[3]))
    }
}

impl Sensor {
    pub fn new(sensor_location: Point, beacon_location: Point) -> Self {
        Self { sensor_location, beacon_location, manhattan_radius: sensor_location.manhattan_distance(&beacon_location) }
    }

    pub fn collides(&self, location: &Point) -> bool {
        self.beacon_location == *location
    }

    pub fn could_sense(&self, location: &Point) -> bool {
        self.sensor_location.manhattan_distance(location) <= self.manhattan_radius
    }
}
