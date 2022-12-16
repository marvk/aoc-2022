use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::Map;
use std::ptr::hash;
use std::rc::Rc;

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
        let (start, nodes) = parse_nodes(input);

        Search2::new(start, nodes).search()
    }
}

struct Search2 {
    start: Rc<Node>,
    nodes: Vec<Rc<Node>>,
    adjacency: HashMap<Rc<Node>, HashMap<Rc<Node>, u32>>,
    hash_table: RefCell<HashMap<u64, u32>>,
}

impl Search2 {
    pub fn new(start: Rc<Node>, nodes: Vec<Rc<Node>>) -> Self {
        let adjacency = Self::shortest_paths(&nodes);

        Self { start, nodes, adjacency, hash_table: RefCell::new(HashMap::new()) }
    }


    fn search_part_2(&self) -> u32 {
        let mut result = 0;


        for i in 0..(1 << self.adjacency.len()) {
            let (human, elephant): (_, Vec<_>) = self.adjacency.clone().into_iter().enumerate().partition(|(j, (node, _))| i & 1 << j != 0 || node.name == "AA");

            let x1 = self.adjacency.iter().find(|(u, a)| u.name == "AA").map(|(a,b)| (a.clone(), b.clone())).unwrap();

            let human = human.into_iter().map(|(_, pair)| pair).collect::<Vec<_>>();
            let elephant = elephant.into_iter().map(|(_, pair)| pair).chain(vec![x1].into_iter()).collect::<Vec<_>>();

            let retain_human = human.iter().map(|(n, _)| n.name.clone()).chain(vec!["AA".to_string()].into_iter()).collect::<Vec<_>>();
            let retain_elephant = elephant.iter().map(|(n, _)| n.name.clone()).chain(vec!["AA".to_string()].into_iter()).collect::<Vec<_>>();

            let human = human.into_iter().map(|(u, mut neighbours)| {
                neighbours.retain(|v, _| retain_human.contains(&v.name));
                (u, neighbours)
            }).collect::<HashMap<_,_>>();

            let elephant = elephant.into_iter().map(|(u, mut neighbours)| {
                neighbours.retain(|v, _| retain_elephant.contains(&v.name));
                (u, neighbours)
            }).collect::<HashMap<_,_>>();


            let human_score = self.search_part_1(&human, 26);
            let elephant_score = self.search_part_1(&elephant, 26);

            let current = human_score + elephant_score;
            result = max(result, current);

            // println!("HUMAN");
            // for x in &human {
            //     println!("{:?}", x);
            // }
            // println!();
            // println!("ELEPHANT");
            // for x in &elephant {
            //     println!("{:?}", x);
            // }
            // println!();
            // println!("human {}", human_score);
            // println!("eleph {}", elephant_score);
            // println!("total {}", current);
            // println!("best  {}", result);
            // println!();
            // println!();

        }

        result
    }

    fn hash(p: (u32, &Vec<Rc<Node>>, &Node, i32, &Node, i32)) -> u64 {
        let mut hasher = DefaultHasher::new();
        p.hash(&mut hasher);
        hasher.finish()
    }

    fn search(&self) -> u32 {
        self.search_part_1(&self.adjacency, 30)
    }

    fn search_part_1(&self, adjacency: &HashMap<Rc<Node>, HashMap<Rc<Node>, u32>>, time_remaining: i32) -> u32 {
        let mut visited = vec![];
        visited.push(self.start.clone());
        self.search_part_1_rec(self.start.clone(), &mut visited, 0, time_remaining, adjacency)
    }

    fn search_part_1_rec(&self, current_node: Rc<Node>, visited: &mut Vec<Rc<Node>>, flow: u32, time_remaining: i32, adjacency: &HashMap<Rc<Node>, HashMap<Rc<Node>, u32>>) -> u32 {
        if time_remaining <= 0 {
            return flow;
        }

        let mut best = flow;

        if let Some(neighbours) = adjacency.get(&current_node) {
            for (neighbour, distance) in neighbours {
                if visited.contains(&neighbour) {
                    continue;
                }

                let new_time = time_remaining - *distance as i32 - 1;
                let new_flow = flow + (neighbour.flow_rate * (max(new_time, 0)) as u32);

                visited.push(neighbour.clone());

                best = max(best, self.search_part_1_rec(neighbour.clone(), visited, new_flow, new_time, adjacency));

                visited.remove(visited.len() - 1);
            }
        }


        best
    }

    fn shortest_paths(nodes: &Vec<Rc<Node>>) -> HashMap<Rc<Node>, HashMap<Rc<Node>, u32>> {
        let mut dist: HashMap<(Rc<Node>, Rc<Node>), u32> = HashMap::new();

        for u in nodes {
            dist.insert((u.clone(), u.clone()), 0);
            let n = u.neighbours.borrow().len();
            for i in 0..n {
                let v = u.neighbours.borrow()[i].clone();
                dist.insert((u.clone(), v), 1);
            }
        }

        for k in nodes {
            for i in nodes {
                for j in nodes {
                    let a = dist.get(&(i.clone(), k.clone())).unwrap_or(&100000);
                    let b = dist.get(&(k.clone(), j.clone())).unwrap_or(&100000);
                    let candidate = a + b;
                    let index = (i.clone(), j.clone());
                    if *dist.get(&index).unwrap_or(&100000) > candidate {
                        dist.insert(index, candidate);
                    }
                }
            }
        }

        let result: HashMap<Rc<Node>, HashMap<Rc<Node>, u32>> = nodes.iter().filter(|n| n.name == "AA" || n.flow_rate > 0).map(|n| {
            let mut map = HashMap::new();

            for ((u, v), d) in &dist {
                if u == n && v.flow_rate != 0 && u != v {
                    map.insert(v.clone(), *d);
                }
            }

            (n.clone(), map)
        }).collect();


        // for x in &result {
        //     println!("{:?}", x);
        // }

        result
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        1707
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let (start, nodes) = parse_nodes(input);

        Search2::new(start, nodes).search_part_2()
    }
}

fn parse_nodes(input: &Vec<String>) -> (Rc<Node>, Vec<Rc<Node>>) {
    let mut neighbours_map = HashMap::new();

    let nodes = input.iter().filter(|line| !line.is_empty()).map(|line| {
        let mut split = line.split(" ");
        let name = split.nth(1).unwrap().to_string();
        let flow_rate = split.nth(2).unwrap().replace("rate=", "").replace(";", "").parse::<u32>().unwrap();
        let neighbours = split.skip(4).map(|e| e.replace(",", "")).collect::<Vec<_>>();
        neighbours_map.insert(name.clone(), neighbours);


        (name.clone(), Rc::new(Node::new(name, flow_rate)))
    }).collect::<HashMap<String, Rc<Node>>>();


    for (name, neighbours) in neighbours_map {
        let current: &Node = nodes.get(&name).unwrap().borrow();

        let vec1 = neighbours.iter().map(|s| nodes.get(s).unwrap().clone()).collect::<Vec<_>>();

        current.neighbours.replace(vec1);
    }

    let mut result = nodes.into_values().collect::<Vec<_>>();
    result.sort_by_key(|n| n.name.clone());
    (result.iter().find(|n| n.name == "AA").unwrap().clone(), result)
}

struct Node {
    name: String,
    neighbours: RefCell<Vec<Rc<Node>>>,
    flow_rate: u32,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Node {}

impl Node {
    pub fn new(name: String, flow_rate: u32) -> Self {
        Self { name, neighbours: RefCell::new(vec![]), flow_rate }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write!(f,
        //        "Node {{ name: {}, flow_rate: {}, neighbours: {:?} }}",
        //        self.name,
        //        self.flow_rate,
        //        self.neighbours.borrow().iter().map(|n| &n.name).collect::<Vec<_>>(),
        // )

        write!(f,
               "{}",
               self.name,
        )
    }
}
