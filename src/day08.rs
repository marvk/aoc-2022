use std::collections::HashSet;
use std::fs::DirEntry;
use std::iter::{Enumerate, FlatMap, Map};
use std::slice::Iter;

use crate::harness::{Day, Part};

pub fn day08() -> Day<u32, u32> {
    Day::new(8, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        21
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        count_visible(&parse(input))
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        8
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        highest_scenic_score(&parse(input))
    }
}

struct Grid {
    grid: Vec<Vec<u32>>,
}

impl Grid {
    pub fn new(grid: Vec<Vec<u32>>) -> Self {
        Self { grid }
    }

    fn get(&self, p: &Point) -> Option<u32> {
        self.grid.get(p.y as usize).map(|arr| arr.get(p.x as usize).map(|i| *i)).flatten()
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

const fn p(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

const ORTHOGONAL_DIRECTIONS: [Point; 4] = [p(0, -1), p(-1, 0), p(0, 1), p(1, 0)];

fn parse(input: &Vec<String>) -> Grid {
    let vec = input.iter().filter(|line| !line.is_empty()).map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    Grid::new(vec)
}

fn count_visible(grid: &Grid) -> u32 {
    grid.grid.iter()
        .enumerate()
        .flat_map(|(y, row)|
            row.iter()
                .enumerate()
                .map(move |(x, _)| p(x as i32, y as i32))
        )
        .map(|p|
            if ORTHOGONAL_DIRECTIONS.iter().any(|d| is_visible(&grid, &p, d).0) { 1 } else { 0 }
        )
        .sum()
}

fn highest_scenic_score(grid: &Grid) -> u32 {
    grid.grid.iter()
        .enumerate()
        .flat_map(|(y, arr)|
            arr.iter()
                .enumerate()
                .map(move |(x, _)| p(x as i32, y as i32))
        )
        .map(|p|
            ORTHOGONAL_DIRECTIONS.iter()
                .map(|d| is_visible(&grid, &p, d))
                .map(|r| r.1)
                .product()
        )
        .max()
        .unwrap()
}

fn is_visible(grid: &Grid, point: &Point, direction: &Point) -> (bool, u32) {
    let mut location = point.clone();
    let original_height = grid.get(&location).unwrap();

    let mut count = 0;

    loop {
        location = location.add(&direction);
        let current_height = grid.get(&location);

        match current_height {
            None => break,
            Some(d) if d >= original_height => return (false, count + 1),
            _ => count += 1,
        }
    }

    (true, count)
}
