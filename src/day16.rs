use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::iter::Map;
use std::ops::Range;
use std::pin::Pin;
use std::ptr::hash;
use std::rc::Rc;
use std::sync::mpsc::{channel, Sender};
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use rand::prelude::SliceRandom;

use crate::harness::{Day, Part};

pub fn day16() -> Day<u32, u32> {
    Day::new(16, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        1651
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let nodes = parse_nodes(input);

        Search::new("AA".to_string(), nodes).search_part_1()
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        1707
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let nodes = parse_nodes(input);

        Search::new("AA".to_string(), nodes).search_part_2()
    }
}

struct Search {
    start: String,
    flow_rates: HashMap<String, u32>,
    adjacency: HashMap<String, HashMap<String, u32>>,
}

impl Search {
    pub fn new(start: String, nodes: HashMap<String, RawNode>) -> Self {
        let adjacency = Self::shortest_paths(&nodes);
        let flow_rates = nodes.into_iter().map(|(name, node)| (name, node.flow_rate)).collect();

        Self { start, flow_rates, adjacency }
    }

    fn search_part_1(&self) -> u32 {
        Self::start_search(Self::build_nodes(&self.start, &self.adjacency, &self.flow_rates), 30)
    }

    fn search_part_1_rec(current_node: Rc<SearchNode>, visited: &mut Vec<Rc<SearchNode>>, flow: u32, time_remaining: i32) -> u32 {
        if time_remaining <= 0 {
            return flow;
        }

        let mut best = flow;

        for (neighbour, distance) in current_node.neighbours.borrow().iter() {
            if visited.contains(neighbour) {
                continue;
            }

            let new_time = time_remaining - *distance as i32 - 1;
            let new_flow = flow + (neighbour.flow_rate * (max(new_time, 0)) as u32);

            visited.push(neighbour.clone());

            best = max(best, Self::search_part_1_rec(neighbour.clone(), visited, new_flow, new_time));

            visited.remove(visited.len() - 1);
        }


        best
    }


    fn search_part_2(&self) -> u32 {
        let ranges = self.build_search_ranges();

        let n = ranges.len();

        let (tx, rx) = channel();

        for range in ranges {
            let start = self.start.clone();
            let adjacency = self.adjacency.clone();
            let flow_rates = self.flow_rates.clone();
            let tx = tx.clone();
            thread::spawn(move || {
                let r = Self::search_part_2_part(start, range, &adjacency, &flow_rates);
                tx.send(r).unwrap();
            });
        }

        (0..n).map(|_| rx.recv_timeout(Duration::from_secs(u64::MAX)).unwrap()).max().unwrap()
    }

    fn build_search_ranges(&self) -> Vec<Vec<usize>> {
        let n = self.adjacency.len();
        let mut bitsets = (0_usize..(1 << n)).collect::<Vec<_>>();
        bitsets.shuffle(&mut rand::thread_rng());

        let threads = thread::available_parallelism().unwrap().get();
        let chunk_size = max(1, (bitsets.len() as f64 / threads as f64).ceil() as usize);

        bitsets.chunks(chunk_size)
            .map(|c| c.into_iter().map(|i| i.clone()).collect::<Vec<_>>())
            .filter(|vec| !vec.is_empty())
            .collect::<Vec<_>>()
    }

    fn search_part_2_part(start: String, range: Vec<usize>, adjacency: &HashMap<String, HashMap<String, u32>>, flow_rates: &HashMap<String, u32>) -> u32 {
        let mut result = 0;

        for i in range {
            // There is just waaay to much cloning and data transformation going on here but I can't be asked to clean this garbage up

            let (human, mut elephant): (_, Vec<_>) = adjacency.clone().into_iter().enumerate().partition(|(j, (name, _))| i & 1 << j != 0 || *name == "AA");
            // Add AA back into elephant
            let start_node_for_elephant = adjacency.iter().find(|(u, _)| **u == start).map(|(a, b)| (a.clone(), b.clone())).unwrap();
            elephant.push((elephant.len(), start_node_for_elephant));

            let human = Self::transform(human);
            let elephant = Self::transform(elephant);

            let human = Self::build_nodes(&start, &human, flow_rates);
            let elephant = Self::build_nodes(&start, &elephant, flow_rates);

            let human_score = Self::start_search(human, 26);
            let elephant_score = Self::start_search(elephant, 26);

            let current = human_score + elephant_score;
            result = max(result, current);
        }

        result
    }

    fn transform(human: Vec<(usize, (String, HashMap<String, u32>))>) -> HashMap<String, HashMap<String, u32>> {
        let nodes = human.into_iter().map(|(_, pair)| pair).collect::<Vec<_>>();
        let retain = nodes.iter().map(|(n, _)| n.clone()).chain(vec!["AA".to_string()].into_iter()).collect::<Vec<_>>();
        nodes.into_iter().map(|(u, mut neighbours)| {
            neighbours.retain(|v, _| retain.contains(&v));
            (u, neighbours)
        }).collect::<HashMap<_, _>>()
    }


    fn start_search(start: Rc<SearchNode>, time_remaining: i32) -> u32 {
        let mut visited = vec![];
        visited.push(start.clone());
        Self::search_part_1_rec(start, &mut visited, 0, time_remaining)
    }

    fn build_nodes(start: &String, adjacency: &HashMap<String, HashMap<String, u32>>, flow_rates: &HashMap<String, u32>) -> Rc<SearchNode> {
        let nodes = flow_rates.iter().map(|(name, flow_rate)| (name.clone(), Rc::new(SearchNode::new(name.clone(), *flow_rate)))).collect::<HashMap<_, _>>();
        for (u, adjacent_nodes) in adjacency {
            let current: &SearchNode = nodes[u].borrow();

            let map = adjacent_nodes.iter().map(|(v, weight)| (nodes[v].clone(), *weight)).collect::<Vec<_>>();

            current.neighbours.replace(map);
        }
        nodes[start].clone()
    }

    fn shortest_paths(nodes: &HashMap<String, RawNode>) -> HashMap<String, HashMap<String, u32>> {
        let mut dist: HashMap<(RawNode, RawNode), u32> = HashMap::new();

        for (_, u) in nodes {
            dist.insert((u.clone(), u.clone()), 0);
            let n = u.neighbours.len();
            for i in 0..n {
                let v = &u.neighbours[i];
                dist.insert((u.clone(), nodes[v].clone()), 1);
            }
        }

        for (_, k) in nodes {
            for (_, i) in nodes {
                for (_, j) in nodes {
                    let a = dist.get(&(i.clone(), k.clone())).unwrap_or(&1000000);
                    let b = dist.get(&(k.clone(), j.clone())).unwrap_or(&1000000);
                    let candidate = a + b;
                    let index = (i.clone(), j.clone());
                    if *dist.get(&index).unwrap_or(&1000000) > candidate {
                        dist.insert(index, candidate);
                    }
                }
            }
        }

        let dist = dist;

        nodes.iter().filter(|(name, n)| *name == "AA" || n.flow_rate > 0).map(|(name, n)| {
            let mut map = HashMap::new();

            for ((u, v), d) in &dist {
                if u == n && v.flow_rate != 0 && u != v {
                    map.insert(v.name.clone(), *d);
                }
            }

            (name.clone(), map)
        }).collect()
    }
}

fn parse_nodes(input: &Vec<String>) -> HashMap<String, RawNode> {
    let mut neighbours_map = HashMap::new();

    let nodes: HashMap<_, _> = input.iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (name, neighbours, flow_rate) = parse_line(line);
            neighbours_map.insert(name.clone(), neighbours);
            (name.clone(), flow_rate)
        })
        .collect();

    nodes.into_iter().map(|(name, flow_rate)| RawNode::new(name.clone(), flow_rate, neighbours_map[&name].clone())).map(|node| (node.name.clone(), node)).collect::<HashMap<_, _>>()
}

fn parse_line(line: &String) -> (String, Vec<String>, u32) {
    let mut split = line.split(" ");
    let name = split.nth(1).unwrap().to_string();
    let flow_rate = split.nth(2).unwrap().replace("rate=", "").replace(";", "").parse::<u32>().unwrap();
    let neighbours = split.skip(4).map(|e| e.replace(",", "")).collect::<Vec<_>>();
    (name, neighbours, flow_rate)
}

#[derive(Clone)]
struct RawNode {
    name: String,
    neighbours: Vec<String>,
    flow_rate: u32,
}

impl RawNode {
    pub fn new(name: String, flow_rate: u32, neighbours: Vec<String>) -> Self {
        Self { name, neighbours, flow_rate }
    }
}

impl Hash for RawNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq<Self> for RawNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for RawNode {}

struct SearchNode {
    name: String,
    neighbours: RefCell<Vec<(Rc<SearchNode>, u32)>>,
    flow_rate: u32,
}

impl SearchNode {
    pub fn new(name: String, flow_rate: u32) -> Self {
        Self { name, neighbours: RefCell::new(vec![]), flow_rate }
    }
}

impl Hash for SearchNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq<Self> for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for SearchNode {}
