use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

use FaceDirection::{Back, Down, Front, Left, Right, Up};
use Instruction::{Turn, Walk};
use Rotation::{Anticlockwise, Clockwise};

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

        for inst in &instructions {
            solver.execute(inst);
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

        for inst in &instructions {
            solver.execute(inst);
        }

        solver.score()
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let vec = input.split(|line| line.is_empty()).collect::<Vec<_>>();
    let max_len = vec[0].iter().map(|s| s.len()).max().unwrap();
    let raw = vec[0].into_iter()
        .map(|line| format!("{}{}", line, " ".repeat(max_len - line.len())))
        .map(|line| line.chars().collect())
        .collect();

    let instructions = parse_instructions(&vec[1][0]);

    (raw, instructions)
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut current = String::new();
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
            result.push(Walk(current.parse().unwrap()));
            result.push(Turn(direction));
            current = String::new();
        }
    }

    if current.len() > 0 {
        result.push(Walk(current.parse().unwrap()));
    }

    result
}

trait Map {
    fn height(&self) -> usize;
    fn width(&self) -> usize;
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
            .copied()
    }
}

struct CubeMap {
    raw: Vec<Vec<char>>,
    faces: Vec<Face>,
    edge_length: usize,
}

impl CubeMap {
    pub fn new(raw: Vec<Vec<char>>) -> Self {
        let (face_origins, edge_length) = Self::build_raw_faces(&raw);

        let faces = Self::build_faces(face_origins, edge_length);

        Self { raw, faces, edge_length }
    }

    fn build_raw_faces(raw: &Vec<Vec<char>>) -> (Vec<Point>, usize) {
        let total_area = raw.iter().flatten().filter(|&&c| c != ' ').count();
        let face_area = total_area / 6;
        let edge_length = (face_area as f64).sqrt() as usize;

        let raw_width = raw[0].len();
        let raw_height = raw.len();

        let map =
            (0..raw_height).step_by(edge_length)
                .flat_map(|y| (0..raw_width).step_by(edge_length).map(move |x| (x, y)))
                .filter(|&(x, y)| raw[y][x] != ' ')
                .map(|(x, y)| p(x as i32, y as i32) / edge_length as i32)
                .collect();

        (map, edge_length)
    }

    fn build_faces(face_origins: Vec<Point>, edge_length: usize) -> Vec<Face> {
        let mut faces: Vec<Face> = vec![];

        let mut open_list = VecDeque::new();
        open_list.push_back(*face_origins.iter().next().unwrap());

        while let Some(face_grid_position) = open_list.pop_back() {
            if faces.iter().any(|f| f.face_grid_position == face_grid_position) {
                continue;
            }

            let face = if faces.is_empty() {
                // Handling first face
                let raw_origin = face_grid_position * edge_length as i32;
                let face_direction = Up;
                let neighbours = face_direction.clockwise_neighbours();
                let neighbour_faces = ORTHOGONAL_DIRECTIONS.iter().enumerate().map(|(i, &d)| (d, neighbours[i])).collect();
                Face::new(raw_origin, face_grid_position, face_direction, neighbour_faces)
            } else {
                if face_origins.contains(&face_grid_position) {
                    let (face, neighbours) = Self::orient_face_and_build_neighbours(&mut faces, face_grid_position);

                    Face::new(face_grid_position * edge_length as i32, face_grid_position, face, neighbours)
                } else {
                    continue;
                }
            };

            open_list.extend(ORTHOGONAL_DIRECTIONS.iter().map(|&d| face.face_grid_position + d));
            faces.push(face);
        }

        faces
    }

    fn orient_face_and_build_neighbours(faces: &Vec<Face>, face_grid_position: Point) -> (FaceDirection, HashMap<Point, FaceDirection>) {
        let (neighbour_direction, face_neighbour) =
            ORTHOGONAL_DIRECTIONS.iter()
                .find_map(|&d|
                    faces.iter()
                        .find(|f| f.face_grid_position == face_grid_position + d)
                        .map(|f| (d, f))
                )
                .unwrap();

        let face = face_neighbour.neighbour_faces[&-neighbour_direction];

        let neighbours = face.clockwise_neighbours();
        let start_index = neighbours.iter().enumerate().find(|(_, &d)| d == face_neighbour.face_direction).map(|(i, _)| i).unwrap();

        let map =
            (start_index..(start_index + 4))
                .map(|i| {
                    let mut direction = neighbour_direction;
                    for _ in start_index..i {
                        direction = Clockwise.apply(direction);
                    }
                    let face_direction = neighbours[i % 4];
                    (direction, face_direction)
                })
                .collect::<HashMap<_, _>>();

        (face, map)
    }

    fn get_face_by_point(&self, point: Point) -> &Face {
        self.faces.iter()
            .find(|face| face.raw_origin == (point / self.edge_length as i32) * self.edge_length as i32)
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
        let destination = start + direction;
        if let Some(result) = self.get(destination) {
            (destination, result, direction)
        } else {
            let from_face = self.get_face_by_point(start);
            let to_face = self.get_face(from_face.neighbour_faces[&direction]);
            let required_direction_opposite = to_face.neighbour_faces.iter().find(|(_, &f)| f == from_face.face_direction).map(|(&d, _)| d).unwrap();
            let required_direction = -required_direction_opposite;

            let start_on_face = start - from_face.raw_origin;
            let mut current_direction = direction;
            let mut current_position = start_on_face;

            let offset_length = self.edge_length as i32 - 1;

            while current_direction != required_direction {
                current_direction = Anticlockwise.apply(current_direction);
                current_position = Anticlockwise.apply(current_position) + Point::SOUTH * offset_length;
            }

            let final_destination = current_position + to_face.raw_origin - required_direction * offset_length;
            let final_direction = required_direction;

            (final_destination, self.get(final_destination).unwrap(), final_direction)
        }
    }

    fn get(&self, point: Point) -> Option<char> {
        self.raw
            .get(point.y as usize)
            .map(|row| row.get(point.x as usize))
            .flatten()
            .filter(|&&tile| tile != ' ')
            .copied()
    }
}

struct Face {
    raw_origin: Point,
    face_grid_position: Point,
    face_direction: FaceDirection,
    neighbour_faces: HashMap<Point, FaceDirection>,
}

impl Face {
    pub fn new(raw_origin: Point, face_grid_position: Point, face_direction: FaceDirection, neighbour_faces: HashMap<Point, FaceDirection>) -> Self {
        Self { raw_origin, face_grid_position, face_direction, neighbour_faces }
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
    const fn clockwise_neighbours(&self) -> [FaceDirection; 4] {
        match self {
            Up => [Back, Right, Front, Left],
            Down => [Front, Right, Back, Left],
            Right => [Back, Down, Front, Up],
            Left => [Front, Down, Back, Up],
            Front => [Up, Right, Down, Left],
            Back => [Left, Down, Right, Up],
        }
    }
}

struct Solver {
    map: Box<dyn Map>,
    position: RefCell<Point>,
    direction: RefCell<Point>,
}

impl Solver {
    pub fn new(map: Box<dyn Map>) -> Self {
        let (position, _) = map.find_first_map_position(Point::ZERO, Point::EAST);

        Self { map, position: RefCell::new(position), direction: RefCell::new(Point::EAST) }
    }

    fn execute(&self, instruction: &Instruction) {
        match instruction {
            Turn(rotation) => self.rotate(*rotation),
            Walk(length) => self.walk(*length),
        }
    }

    fn walk(&self, length: usize) {
        for _ in 0..length {
            let (new_point, tile, new_direction) = self.map.find_neighbour(*self.position.borrow(), *self.direction.borrow());
            if tile != '#' {
                self.position.replace(new_point);
                self.direction.replace(new_direction);
            }
        }
    }

    fn rotate(&self, rotation: Rotation) {
        let direction = *self.direction.borrow();
        self.direction.replace(rotation.apply(direction));
    }

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

impl Point {
    const ZERO: Self = p(0, 0);
    const NORTH: Self = p(0, -1);
    const EAST: Self = p(1, 0);
    const SOUTH: Self = p(0, 1);
    const WEST: Self = p(-1, 0);
}

#[derive(Debug, Copy, Clone)]
enum Rotation {
    Clockwise,
    Anticlockwise,
}

impl Rotation {
    fn apply(&self, point: Point) -> Point {
        match self {
            Clockwise => p(-point.y, point.x),
            Anticlockwise => p(point.y, -point.x),
        }
    }
}
