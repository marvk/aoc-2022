//! yeah don't look at this hot mess i don't wanna fix it any more

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use crate::harness::{Day, Part};

pub fn day12() -> Day<u32, u32> {
    Day::new(12, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        31
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let map = Map::from(input);

        Pathfinder::new(map).shortest_path()
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        29
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let map = Map::from(input);

        (0..map.raw.len()).map(|y| p(0, y as i32)).map(|p| {
            let mut m = map.clone();
            m.raw.iter().flat_map(|row| row.iter()).for_each(|node| { node.predecessor.replace(None); });
            m.start = p;
            let r = Pathfinder::new(m).shortest_path();
            r
        }).min().unwrap()
    }
}


#[derive(Eq, PartialEq, Debug)]
enum Weight {
    Start,
    End,
    Height(u32),
}

impl Display for Weight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Weight::Start => "S".to_string(),
            Weight::End => "E".to_string(),
            Weight::Height(height) => height.to_string(),
        };

        write!(f,
               "{: >3}",
               x
        )
    }
}

impl Weight {
    fn height(&self) -> u32 {
        match self {
            Weight::Start => char_to_height('a'),
            Weight::End => char_to_height('z'),
            Weight::Height(height) => *height,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Node {
    weight: Weight,
    position: Point,
    predecessor: RefCell<Option<Rc<Node>>>,
    f: RefCell<i32>,
    g: RefCell<i32>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl Node {
    pub fn new(weight: Weight, position: Point) -> Self {
        Self { weight, position, predecessor: RefCell::new(None), f: RefCell::new(i32::MAX), g: RefCell::new(i32::MAX) }
    }

    fn height(&self) -> u32 {
        self.weight.height()
    }

    fn set_f(&self, value: i32) {
        self.f.replace(value);
    }

    fn set_g(&self, value: i32) {
        self.g.replace(value);
    }

    fn g(&self) -> i32 {
        self.g.clone().take()
    }

    fn f(&self) -> i32 {
        self.f.clone().take()
    }

    fn set_predcessor(&self, other: Rc<Node>) {
        self.predecessor.replace(Some(other.clone()));
    }
}


fn path(node: Rc<Node>) -> Vec<Rc<Node>> {
    let mut visited = HashSet::new();
    let mut path = vec![node.clone()];

    loop {
        let rc = path.last().unwrap().predecessor.clone().take();

        if let Some(x) = rc {
            if !visited.insert(x.clone()) {
                panic!("tail: {:?} --- insert: {:?} --- visited: {:?} --- path: {:?}", node.position, x.position, visited.iter().map(|it| it.position).collect::<Vec<_>>(), path.iter().map(|it| it.position).collect::<Vec<_>>());
            }

            path.push(x)
        } else {
            return path;
        }
    }
}

struct Pathfinder {
    map: Map,
    open_list: Vec<Rc<Node>>,
    closed_list: HashSet<Rc<Node>>,
}

impl Pathfinder {
    pub fn new(map: Map) -> Self {
        Self { map, open_list: Vec::new(), closed_list: HashSet::new() }
    }

    fn g(&self, node: Rc<Node>) -> i32 {
        node.g.clone().take()
    }

    fn h(&self, node: Rc<Node>) -> i32 {
        node.position.manhattan(self.map.end)
    }

    fn c(&self, current_node: &Node, successor: &Node) -> i32 {
        if current_node.height() + 1 < successor.height() { 1000000 } else { 1 }
    }

    fn expand_node(&mut self, current_node: Rc<Node>) {
        for d in ORTHOGONAL_DIRECTIONS {
            let successor = self.map.get(current_node.position.add(d));
            if let None = successor {
                continue;
            }
            let successor = successor.unwrap();
            if self.closed_list.contains(&successor) {
                continue;
            }

            let tentative_g = self.g(current_node.clone()) + self.c(&current_node, &successor);

            if self.open_list.contains(&successor) && tentative_g >= self.g(successor.clone()) {
                continue;
            }

            successor.set_predcessor(current_node.clone());
            successor.set_g(tentative_g);
            successor.set_f(tentative_g + self.h(successor.clone()));
            if !self.open_list.contains(&successor) {
                self.open_list.push(successor);
            }
        }
    }


    fn shortest_path(&mut self) -> u32 {
        self.open_list.push(self.map.start());

        while !self.open_list.is_empty() {
            self.open_list.sort_by_key(|n| Reverse(n.f.clone().take()));

            let current_node = self.open_list[self.open_list.len() - 1].clone();
            self.open_list.remove(self.open_list.len() - 1);

            if current_node == self.map.end() {
                break;
            }

            self.closed_list.insert(current_node.clone());

            self.expand_node(current_node.clone());
        }

        let mut count = 0;

        let mut current = Some(self.map.end().clone());

        let mut arr = [[0; 173]; 41];

        while let Some(node) = current {
            count += 1;
            let p = &node.position;
            // println!("{:?}", p);

            arr[p.y as usize][p.x as usize] = 1;

            current = node.predecessor.clone().take();
        }

        // for x in arr {
        //     println!("{}", x.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(""));
        // };

        count - 1
    }
}


#[derive(Clone)]
struct Map {
    raw: Vec<Vec<Rc<Node>>>,
    start: Point,
    end: Point,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&self, other: Self) -> Point {
        p(self.x + other.x, self.y + other.y)
    }

    fn manhattan(&self, other: Self) -> i32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i32
    }
}

const fn p(x: i32, y: i32) -> Point {
    Point { x, y }
}

const ORTHOGONAL_DIRECTIONS: [Point; 4] = [p(0, 1), p(1, 0), p(0, -1), p(-1, 0)];

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<&Vec<String>> for Map {
    fn from(input: &Vec<String>) -> Self {
        let mut start: Option<Point> = None;
        let mut end: Option<Point> = None;

        let x = input.iter().filter(|line| !line.is_empty()).enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)|
                {
                    let position = p(x as i32, y as i32);
                    let weight = match c {
                        'S' => {
                            start = Some(position);
                            Weight::Start
                        }
                        'E' => {
                            end = Some(position);
                            Weight::End
                        }
                        c => Weight::Height(char_to_height(c)),
                    };
                    Rc::new(Node::new(weight, position))
                }).collect()
        }).collect();

        Map { raw: x, start: start.unwrap(), end: end.unwrap() }
    }
}

impl Map {
    fn get(&self, p: Point) -> Option<Rc<Node>> {
        let line: Option<&Vec<Rc<Node>>> = self.raw.get(p.y as usize);
        let node: Option<Rc<Node>> = line.map(|line| line.get(p.x as usize).cloned()).flatten();
        node
    }

    fn start(&self) -> Rc<Node> {
        self.get(self.start).unwrap().clone()
    }

    fn end(&self) -> Rc<Node> {
        self.get(self.end).unwrap().clone()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x = self.raw.iter().map(|l| l.iter().map(|n| format!("{}", n.f.clone().take())).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>().join("\n");

        write!(f,
               "S: {}     E:{}\n{}",
               self.start,
               self.end,
               x
        )
    }
}

fn char_to_height(c: char) -> u32 {
    (c as u32) - 'a' as u32
}
