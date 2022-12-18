use std::collections::{HashSet, VecDeque};
use std::ops::{Add, Neg, Sub};

use crate::harness::{Day, Part};

pub fn day18() -> Day<u32, u32> {
    Day::new(18, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        64
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let droplets = parse_input(input);
        let spatial = build_spatial(&droplets);

        calculate_hull(&droplets, &spatial, false)
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        58
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let droplets = parse_input(input);
        let spatial = build_spatial(&droplets);

        calculate_hull(&flood_fill(&spatial), &spatial, true)
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Point3> {
    input.into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.split(","))
        .map(|arr| arr.map(|s| s.parse().unwrap()).collect::<Vec<i64>>())
        // Add a buffer of 1 around the near side of the droplet so the flood fill will form a complete hull
        .map(|arr| p(arr[0] + 1, arr[1] + 1, arr[2] + 1))
        .collect::<Vec<_>>()
}


fn build_spatial(points: &Vec<Point3>) -> Vec<Vec<Vec<bool>>> {
    let (max_x, max_y, max_z) = max(points);

    // Add a buffer of 1 around the far side of the droplet so the flood fill will form a complete hull
    let mut droplet = vec![vec![vec![false; max_x + 2]; max_y + 2]; max_z + 2];

    for p in points {
        droplet[p.z as usize][p.y as usize][p.x as usize] = true;
    }

    droplet
}

fn flood_fill(spatial: &Vec<Vec<Vec<bool>>>) -> Vec<Point3> {
    let mut open = VecDeque::new();
    // (0, 0, 0) is guaranteed to be an empty space, since we padded the droplet in all directions.
    open.push_back(p(0, 0, 0));
    let mut closed = HashSet::new();

    while !open.is_empty() {
        let current = open.pop_back().unwrap();
        closed.insert(current);

        for direction in ORTHOGONAL_DIRECTIONS {
            let neighbour = current + direction;

            if !closed.contains(&neighbour) && !get(spatial, &neighbour).unwrap_or(true) {
                open.push_back(neighbour);
            }
        }
    }

    closed.into_iter().collect()
}

fn calculate_hull(droplets: &Vec<Point3>, spatial: &Vec<Vec<Vec<bool>>>, inverted: bool) -> u32 {
    droplets
        .into_iter()
        .flat_map(|p| ORTHOGONAL_DIRECTIONS.iter().map(|d| *p + *d))
        .map(|d| get(&spatial, &d))
        .map(|o| o.unwrap_or(false))
        .filter(|v| *v == inverted)
        .count() as u32
}

fn get(p0: &Vec<Vec<Vec<bool>>>, neighbour: &Point3) -> Option<bool> {
    p0
        .get(neighbour.z as usize)
        .map(|arr| arr.get(neighbour.y as usize))
        .flatten()
        .map(|arr| arr.get(neighbour.x as usize))
        .flatten()
        .cloned()
}

fn max(points: &Vec<Point3>) -> (usize, usize, usize) {
    (
        points.iter().map(|p| p.x).max().unwrap() as usize,
        points.iter().map(|p| p.y).max().unwrap() as usize,
        points.iter().map(|p| p.z).max().unwrap() as usize,
    )
}

const ORTHOGONAL_DIRECTIONS: [Point3; 6] = [p(1, 0, 0), p(-1, 0, 0), p(0, 1, 0), p(0, -1, 0), p(0, 0, 1), p(0, 0, -1)];

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

const fn p(x: i64, y: i64, z: i64) -> Point3 {
    Point3 { x, y, z }
}

impl Add<Point3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Point3) -> Self::Output {
        p(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
