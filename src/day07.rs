use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::harness::{Day, Part};

pub fn day07() -> Day<u64, u64> {
    Day::new(7, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        95437
    }

    fn solve(&self, input: &Vec<String>) -> u64 {
        walk(parse_tree(input))
            .iter()
            .map(|d| d.size())
            .filter(|s| *s < 100000_u64)
            .sum()
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        24933642
    }

    fn solve(&self, input: &Vec<String>) -> u64 {
        let root = parse_tree(input);
        let vec = walk(root.clone());

        let current_usable = 70000000 - root.size();
        let needed_space = 30000000 - current_usable;

        let mut directories_with_sufficient_space = vec.into_iter().filter(|d| d.size() > needed_space).collect::<Vec<_>>();

        directories_with_sufficient_space.sort_by_cached_key(|d| d.size());

        directories_with_sufficient_space.first().unwrap().size()
    }
}

pub trait Sized {
    fn size(&self) -> u64;
}

#[derive(Debug)]
struct Directory {
    name: String,
    directory_children: RefCell<Vec<Rc<Directory>>>,
    file_children: RefCell<Vec<File>>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Self { name, directory_children: RefCell::new(Vec::new()), file_children: RefCell::new(Vec::new()) }
    }

    pub fn push_file(&self, file: File) {
        if self.file_children.borrow().iter().find(|f| f.name == file.name).is_some() {
            return;
        }

        self.file_children.borrow_mut().push(file);
    }

    pub fn push_directory(&self, directory: Directory) -> Rc<Directory> {
        let result = Rc::new(directory);
        self.directory_children.borrow_mut().push(result.clone());
        result
    }
}

impl Sized for Directory {
    fn size(&self) -> u64 {
        self.directory_children.borrow().iter().map(|d| d.size()).sum::<u64>() +
            self.file_children.borrow().iter().map(|d| d.size()).sum::<u64>()
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

impl File {
    pub fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }
}

impl Sized for File {
    fn size(&self) -> u64 {
        self.size
    }
}

fn parse_tree(input: &Vec<String>) -> Rc<Directory> {
    let mut stack: VecDeque<Rc<Directory>> = VecDeque::new();
    stack.push_back(Rc::new(Directory::new("/".to_string())));

    for cmd in input.iter().skip(1) {
        match &cmd.split(" ").collect::<Vec<_>>()[..] {
            ["$", "cd", "/"] => { stack.drain(1..); }
            ["$", "cd", ".."] => { stack.pop_back(); }
            ["$", "cd", directory] => { stack.push_back(stack.back().unwrap().push_directory(Directory::new(directory.to_string()))) },
            ["$", "ls"] | ["dir", _] => {}
            [size, filename] => { stack.back().unwrap().push_file(File::new(filename.to_string(), size.parse().unwrap())) },
            _ => {}
        }
    }

    stack.front().unwrap().clone()
}

fn walk(directory: Rc<Directory>) -> Vec<Rc<Directory>> {
    let mut result = Vec::new();
    result.push(directory.clone());
    for child in directory.directory_children.borrow().iter() {
        for walk_child in walk(child.clone()) {
            result.push(walk_child.clone())
        }
    }
    result
}
