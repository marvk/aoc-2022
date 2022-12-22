use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Neg, Sub};

use crate::day22::Rotation::{Anticlockwise, Clockwise};
use crate::harness::{Day, Part};

pub fn day22() -> Day<u32, u32> {
    Day::new(22, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        6032
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let (raw, instructions) = parse_input(input);

        let solver = Solver::new(Box::new(RegularMap::new(raw)));

        for x in &instructions {
            solver.execute(x);
        }

        solver.score()
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        5031
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        let (raw, instructions) = parse_input(input);

        let solver = Solver::new(Box::new(CubeMap::new(raw)));

        for x in &instructions {
            solver.execute(x);
        }

        solver.score()
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let vec = input.split(|line| line.is_empty()).collect::<Vec<_>>();
    let max_len = vec[0].iter().map(|s| s.len()).max().unwrap();
    let raw = vec[0].into_iter()
        .map(|line| format!("{}{}", line, " ".repeat(max_len - line.len())))
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let instructions = parse_instructions(&vec[1][0]);

    (raw, instructions)
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut current = "".to_string();
    let mut result = vec![];

    for char in line.chars() {
        if char.is_ascii_digit() {
            current.push(char);
        } else {
            let direction = match char {
                'R' => Clockwise,
                'L' => Anticlockwise,
                _ => panic!("unknown char {:?}", char),
            };
            result.push(Instruction::Walk(current.parse().unwrap()));
            result.push(Instruction::Turn(direction));
            current = "".to_string();
        }
    }

    if current.len() > 0 {
        result.push(Instruction::Walk(current.parse().unwrap()));
    }

    result
}

trait Map {
    fn find_neighbour(&self, start: Point, direction: Point) -> (Point, char, Point);
    fn get(&self, point: Point) -> Option<char>;
    fn find_first_map_position(&self, start: Point, direction: Point) -> (Point, char) {
        let mut current = start;

        loop {
            if let Some(tile) = self.get(current) {
                return (current, tile);
            } else {
                current = current + direction;
            }
        }
    }
    fn height(&self) -> usize;
    fn width(&self) -> usize;
}

struct CubeMap {
    raw: Vec<Vec<char>>,
    faces: Vec<Face>,
    face_area: usize,
    edge_length: usize,
}

impl CubeMap {
    pub fn new(raw: Vec<Vec<char>>) -> Self {
        let total_area = raw.iter().flat_map(|row| row.iter()).filter(|&&c| c != ' ').count();
        let face_area = total_area / 6;
        let edge_length = (face_area as f64).sqrt() as usize;

        let raw_width = raw[0].len();
        let raw_height = raw.len();

        let mut raw_faces: HashMap<Point, Vec<Vec<char>>> = HashMap::new();

        for y in (0..raw_height).step_by(edge_length) {
            for x in (0..raw_width).step_by(edge_length) {
                if raw[y][x] != ' ' {
                    let grid_position = p((x / edge_length) as i32, (y / edge_length) as i32);

                    let mut raw_cube = vec![];

                    for y_cube in 0..edge_length {
                        let mut current_cube = vec![];

                        for x_cube in 0..edge_length {
                            current_cube.push(raw[y + y_cube][x + x_cube]);
                        }

                        raw_cube.push(current_cube);
                    }

                    raw_faces.insert(grid_position, raw_cube);
                }
            }
        }

        let mut open_list = VecDeque::new();
        open_list.push_back(*raw_faces.keys().min_by_key(|p| p.x).unwrap());

        let mut faces: Vec<Face> = vec![];

        while let Some(face_grid_position) = open_list.pop_back() {
            if faces.iter().any(|f| f.face_grid_position == face_grid_position) {
                continue;
            }

            let face = if faces.is_empty() {
                let vec1 = raw_faces[&face_grid_position].clone();
                let map_origin = p(face_grid_position.x * edge_length as i32, face_grid_position.y * edge_length as i32);
                let direction = FaceDirection::Up;
                let neighbours = direction.neighbours();
                let neighbours_map = ORTHOGONAL_DIRECTIONS.iter().enumerate().map(|(i, d)| (*d, neighbours[i])).collect::<HashMap<_, _>>();
                Face::new(vec1, map_origin, face_grid_position, direction, neighbours_map)
            } else {
                if let Some(raw) = raw_faces.get(&face_grid_position) {
                    let (neighbour_direction, face_neighbour) =
                        ORTHOGONAL_DIRECTIONS.iter()
                            .map(|d| (*d, face_grid_position + *d))
                            .map(|(d, p)| (d, faces.iter().find(|face| face.face_grid_position == p)))
                            .find(|(_, f)| f.is_some())
                            .map(|(d, f)| (d, f.unwrap()))
                            .unwrap();

                    let x = face_neighbour.face_direction;

                    let face = face_neighbour.neighbour_faces[&-neighbour_direction];

                    let neighbours = face.neighbours();
                    let start_index = neighbours.iter().enumerate().find(|(_, d)| **d == x).map(|(i, _)| i).unwrap();


                    let map = (start_index..(start_index + 4)).map(|i| {
                        let mut direction = neighbour_direction;
                        for j in start_index..i {
                            direction = Rotation::Clockwise.apply(direction);
                        }
                        let face_direction = neighbours[i % 4];
                        (direction, face_direction)
                    }).collect::<HashMap<_, _>>();
                    let map_origin = p(face_grid_position.x * edge_length as i32, face_grid_position.y * edge_length as i32);

                    Face::new(raw.clone(), map_origin, face_grid_position, face, map)
                } else {
                    continue;
                }
            };

            ORTHOGONAL_DIRECTIONS.iter().map(|d| face.face_grid_position + *d).for_each(|p| open_list.push_back(p));
            faces.push(face);
        }

        Self { raw, faces, face_area, edge_length }
    }

    fn get_face_by_point(&self, point: Point) -> &Face {
        self.faces.iter()
            .find(|face|
                face.raw_origin == p(
                    (point.x / self.edge_length as i32) * self.edge_length as i32,
                    (point.y / self.edge_length as i32) * self.edge_length as i32,
                ))
            .unwrap()
    }

    fn get_face(&self, face_direction: FaceDirection) -> &Face {
        self.faces.iter()
            .find(|face| face.face_direction == face_direction)
            .unwrap()
    }
}

impl Map for CubeMap {
    fn height(&self) -> usize {
        self.raw.len()
    }

    fn width(&self) -> usize {
        self.raw[0].len()
    }

    fn find_neighbour(&self, start: Point, direction: Point) -> (Point, char, Point) {
        let point = start + direction;
        if let Some(result) = self.get(point) {
            (point, result, direction)
        } else {
            let from = self.get_face_by_point(start);
            let to = self.get_face(from.neighbour_faces[&direction]);
            let required_direction = *to.neighbour_faces.iter().find(|(_, v)| **v == from.face_direction).map(|(d, _)| d).unwrap();
            let required_direction = Clockwise.apply(Clockwise.apply(required_direction));

            let origin = start - from.raw_origin;
            let mut current_direction = direction;
            let mut current_destination = origin;

            println!("from {:?} to {:?}", from.face_direction, to.face_direction);
            println!("current direction {:?}", direction);
            println!("required direction {:?}", required_direction);
            println!("start {:?}", start);
            println!("origin on face {:?}", origin);

            while current_direction != required_direction {
                current_direction = Anticlockwise.apply(current_direction);
                current_destination = Anticlockwise.apply(current_destination);
                // println!("_ {:?}", current_direction);
                println!("a {:?}", current_destination);
                current_destination = current_destination + p(0, self.edge_length as i32 - 1);
                println!("b {:?}", current_destination);
            }

            // current_destination = current_destination + required_direction;

            println!("direction {:?}", current_direction);
            println!("destination on face {:?}", current_destination);

            let final_destination = to.raw_origin + current_destination - p(required_direction.x * (self.edge_length - 1) as i32, required_direction.y * (self.edge_length - 1) as i32);
            let final_direction = required_direction;

            println!("final_destination {:?}", final_destination);
            println!("final_direction {:?}", final_direction);

            println!();
            println!();

            (final_destination, self.get(final_destination).expect(&format!("{:?}", final_destination)), final_direction)
        }
    }

    fn get(&self, point: Point) -> Option<char> {
        self.raw
            .get(point.y as usize)
            .map(|row| row.get(point.x as usize))
            .flatten()
            .filter(|&&tile| tile != ' ')
            .map(|char| *char)
    }
}

struct Face {
    raw_origin: Point,
    face_grid_position: Point,
    face_direction: FaceDirection,
    neighbour_faces: HashMap<Point, FaceDirection>,
    raw: Vec<Vec<char>>,
}

impl Debug for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Face")
            .field("raw_origin", &self.raw_origin)
            .field("face_grid_position", &self.face_grid_position)
            .field("face_direction", &self.face_direction)
            .field("neighbour_faces", &self.neighbour_faces)
            .finish()
    }
}

impl Face {
    pub fn new(raw: Vec<Vec<char>>, raw_origin: Point, grid_position: Point, face_direction: FaceDirection, neighbours: HashMap<Point, FaceDirection>) -> Self {
        Self { raw, raw_origin, face_grid_position: grid_position, face_direction, neighbour_faces: neighbours }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FaceDirection {
    Up,
    Down,
    Right,
    Left,
    Front,
    Back,
}

impl FaceDirection {
    const fn neighbours(&self) -> [FaceDirection; 4] {
        match self {
            FaceDirection::Up => [FaceDirection::Back, FaceDirection::Right, FaceDirection::Front, FaceDirection::Left],
            FaceDirection::Down => [FaceDirection::Front, FaceDirection::Right, FaceDirection::Back, FaceDirection::Left],
            FaceDirection::Right => [FaceDirection::Back, FaceDirection::Down, FaceDirection::Front, FaceDirection::Up],
            FaceDirection::Left => [FaceDirection::Front, FaceDirection::Down, FaceDirection::Back, FaceDirection::Up],
            FaceDirection::Front => [FaceDirection::Up, FaceDirection::Right, FaceDirection::Down, FaceDirection::Left],
            FaceDirection::Back => [FaceDirection::Left, FaceDirection::Down, FaceDirection::Right, FaceDirection::Up],
        }
    }
}

struct RegularMap {
    raw: Vec<Vec<char>>,
}

impl RegularMap {
    pub fn new(raw: Vec<Vec<char>>) -> Self {
        Self { raw }
    }
}

impl Map for RegularMap {
    fn height(&self) -> usize {
        self.raw.len()
    }

    fn width(&self) -> usize {
        self.raw[0].len()
    }

    fn find_neighbour(&self, start: Point, direction: Point) -> (Point, char, Point) {
        let point = start + direction;
        if let Some(result) = self.get(point) {
            (point, result, direction)
        } else {
            let origin = match direction {
                Point::NORTH => p(start.x, self.height() as i32 - 1),
                Point::SOUTH => p(start.x, 0),
                Point::EAST => p(0, start.y),
                Point::WEST => p(self.width() as i32 - 1, start.y),
                _ => panic!(),
            };

            let r = self.find_first_map_position(origin, direction);
            (r.0, r.1, direction)
        }
    }

    fn get(&self, point: Point) -> Option<char> {
        self.raw
            .get(point.y as usize)
            .map(|row| row.get(point.x as usize))
            .flatten()
            .filter(|&&tile| tile != ' ')
            .map(|char| *char)
    }
}

struct Solver {
    map: Box<dyn Map>,
    position: RefCell<Point>,
    direction: RefCell<Point>,
}

impl Solver {
    fn score(&self) -> u32 {
        let position = self.position.borrow();
        let direction_score = match *self.direction.borrow() {
            Point::NORTH => 3,
            Point::EAST => 0,
            Point::SOUTH => 1,
            Point::WEST => 2,
            _ => panic!(),
        };

        ((position.x + 1) * 4 + (position.y + 1) * 1000 + direction_score) as u32
    }
}

impl Solver {
    pub fn new(map: Box<dyn Map>) -> Self {
        let (position, _) = map.find_first_map_position(Point::ZERO, Point::EAST);

        Self { map, position: RefCell::new(position), direction: RefCell::new(Point::EAST) }
    }

    fn execute(&self, instruction: &Instruction) {
        match instruction {
            Instruction::Turn(rotation) => {
                let direction = *self.direction.borrow();
                self.direction.replace(rotation.apply(direction));
            }
            Instruction::Walk(length) => {
                for _ in 0..*length {
                    let (new_point, tile, new_direction) = self.map.find_neighbour(*self.position.borrow(), *self.direction.borrow());
                    if tile != '#' {
                        self.position.replace(new_point);
                        self.direction.replace(new_direction);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Walk(usize),
    Turn(Rotation),
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

const ORTHOGONAL_DIRECTIONS: [Point; 4] = [Point::NORTH, Point::EAST, Point::SOUTH, Point::WEST];

impl Point {
    const ZERO: Self = p(0, 0);
    const NORTH: Self = p(0, -1);
    const EAST: Self = p(1, 0);
    const SOUTH: Self = p(0, 1);
    const WEST: Self = p(-1, 0);
}

#[derive(Debug)]
enum Rotation {
    Clockwise,
    Anticlockwise,
}

impl Rotation {
    fn apply(&self, point: Point) -> Point {
        match self {
            Rotation::Clockwise => p(-point.y, point.x),
            Rotation::Anticlockwise => p(point.y, -point.x),
        }
    }
}
