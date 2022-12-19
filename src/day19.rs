// ¯\_(ツ)_/¯

use std::cmp::{max, min};
use std::fmt::{Debug, Formatter};
use std::iter::{Product, Sum};
use std::ops::{Add, Index, Sub};
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;

use rand::thread_rng;

use crate::harness::{Day, Part};

pub fn day19() -> Day<u32, u32> {
    Day::new(19, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        33
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let blueprints = parse_input(input);
        let n = blueprints.len();
        Solver::new(blueprints).solve(n, 24).into_iter().map(|(a, b)| a * b).sum()
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        56 * 62
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let blueprints = parse_input(input);

        Solver::new(blueprints).solve(3, 32).into_iter().map(|(a, _)| a).product()
    }
}

struct Solver {
    blueprints: Vec<Blueprint>,
}

impl Solver {
    pub fn new(blueprints: Vec<Blueprint>) -> Self {
        Self { blueprints }
    }

    fn solve(&self, n_blueprints: usize, time_remaining: u32) -> Vec<(u32, u32)> {
        let (tx, rx) = channel();

        let n_blueprints = min(n_blueprints, self.blueprints.len());

        for (i, blueprint) in self.blueprints.iter().enumerate().take(n_blueprints) {
            let tx = tx.clone();
            let blueprint1 = blueprint.clone();
            thread::spawn(move || {
                let result = Self::solve_rec(
                    &blueprint1,
                    &mut [1, 0, 0, 0],
                    Resources::ZERO,
                    time_remaining,
                    None,
                    0,
                    time_remaining,
                );
                tx.send((result, (i + 1) as u32))
            });
        }

        (0..n_blueprints).map(|_| rx.recv().unwrap()).collect::<Vec<_>>()
    }

    fn solve_rec(blueprint: &Blueprint, robots: &mut [u32], mut resources: Resources, time_remaining: u32, robot_to_add: Option<RobotType>, mut alpha: u32, max_time: u32) -> u32 {
        if time_remaining == 0 {
            return resources.geode as u32;
        }

        if let Some(robot) = robot_to_add {
            robots[robot] += 1
        }

        let time_passed = max_time - time_remaining;

        let production =
            robots.iter().enumerate()
                .map(|(i, cnt)| blueprint.robots[i].output.times(*cnt as i32))
                .sum::<Resources>();

        let geode_robot = blueprint.robots[3];
        let obsidian_robot = blueprint.robots[2];
        let clay_robot = blueprint.robots[1];
        let ore_robot = blueprint.robots[0];

        let mut run_robot = |resources: &mut Resources, cost: Resources, to_add: Option<RobotType>| {
            *resources = *resources - cost;
            *resources = *resources + production;
            alpha = max(alpha, Self::solve_rec(blueprint, robots, *resources, time_remaining - 1, to_add, alpha, max_time));
            *resources = *resources - production;
            *resources = *resources + cost;
        };

        // If we can afford to build a geode robot, always do so
        if resources.can_afford(geode_robot.cost) {
            run_robot(&mut resources, geode_robot.cost, Some(GEODE));
        } else {
            if resources.can_afford(obsidian_robot.cost) {
                run_robot(&mut resources, obsidian_robot.cost, Some(OBSIDIAN));
            }

            // We can never use more than 4 ore per minute, so only build ore robots if the ore production is smaller than 4 per minute
            if production.ore < 4 && resources.can_afford(ore_robot.cost) {
                run_robot(&mut resources, ore_robot.cost, Some(ORE));
            }

            // Don't consider building clay robots in the late game (after 20 minutes)
            // We can never use more than 4 ore per minute, so only build ore robots if the ore production is smaller than 4 per minute
            if time_passed < 20 && production.clay < 20 && resources.can_afford(clay_robot.cost) {
                run_robot(&mut resources, clay_robot.cost, Some(CLAY));
            }

            // Not spending ore is bad, so use it aggresively
            if resources.ore < 5 {
                run_robot(&mut resources, Resources::ZERO, None);
            }
        }

        if let Some(robot) = robot_to_add {
            robots[robot] -= 1
        }

        alpha
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Blueprint> {
    input.iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>()
}

const fn r(ore: i32, clay: i32, obsidian: i32, geode: i32) -> Resources {
    Resources { ore, clay, obsidian, geode }
}

#[derive(Copy, Clone)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Resources {
    const ZERO: Self = r(0, 0, 0, 0);
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        r(self.ore + rhs.ore, self.clay + rhs.clay, self.obsidian + rhs.obsidian, self.geode + rhs.geode)
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        r(self.ore - rhs.ore, self.clay - rhs.clay, self.obsidian - rhs.obsidian, self.geode - rhs.geode)
    }
}

impl Sum for Resources {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or(Resources::ZERO)
    }
}

impl Resources {
    fn can_afford(&self, cost: Self) -> bool {
        let x = *self - cost;
        x.ore >= 0 && x.clay >= 0 && x.obsidian >= 0 && x.geode >= 0
    }

    fn times(&self, factor: i32) -> Resources {
        r(self.ore * factor, self.clay * factor, self.obsidian * factor, self.geode * factor)
    }
}

impl Debug for Resources {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "(or {}, cl {}, ob {}, ge {})", self.ore, self.clay, self.obsidian, self.geode
        )
    }
}


#[derive(Debug, Clone)]
struct Blueprint {
    robots: [Robot; 4],
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let robots: [Robot; 4] =
            s
                .split(":")
                .last()
                .unwrap()
                .split(".")
                .filter(|raw| !raw.is_empty())
                .map(|raw| raw.trim())
                .map(|raw| raw.parse::<Robot>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

        Ok(Blueprint { robots })
    }
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    cost: Resources,
    output: Resources,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(".", "");
        let mut tokens = s.split(" ");
        let collects = index_from_word(tokens.nth(1).unwrap()).unwrap();

        let mut costs = [0_i32; 4];

        while let Some(token) = tokens.next() {
            if let Ok(number) = token.parse::<i32>() {
                costs[index_from_word(tokens.next().unwrap()).unwrap()] = number;
            }
        }

        let output = match collects {
            ORE => r(1, 0, 0, 0),
            CLAY => r(0, 1, 0, 0),
            OBSIDIAN => r(0, 0, 1, 0),
            GEODE => r(0, 0, 0, 1),
            _ => panic!()
        };

        Ok(Robot { output, cost: r(costs[0], costs[1], costs[2], costs[3]) })
    }
}

fn index_from_word(word: &str) -> Option<RobotType> {
    match word {
        "ore" => Some(ORE),
        "clay" => Some(CLAY),
        "obsidian" => Some(OBSIDIAN),
        "geode" => Some(GEODE),
        _ => None
    }
}

type RobotType = usize;

const ORE: RobotType = 0;
const CLAY: RobotType = 1;
const OBSIDIAN: RobotType = 2;
const GEODE: RobotType = 3;
