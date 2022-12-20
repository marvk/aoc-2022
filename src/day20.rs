use std::fmt::{Debug, Formatter};
use std::fs::read_link;

use crate::harness::{Day, Part};

pub fn day20() -> Day<i64, i64> {
    Day::new(20, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i64> for Part1 {
    fn expect_test(&self) -> i64 {
        3
    }

    fn solve(&self, input: &Vec<String>) -> i64 {
        let mut enc_file = EncFile::from(input);

        enc_file.mix();

        enc_file.decode()
    }
}

pub struct Part2;

impl Part<i64> for Part2 {
    fn expect_test(&self) -> i64 {
        1623178306
    }

    fn solve(&self, input: &Vec<String>) -> i64 {
        let mut enc_file = EncFile::from(input);

        enc_file.decrypt(811589153);

        for _ in 0..10 {
            enc_file.mix();
        }

        enc_file.decode()
    }
}

struct EncFile {
    raw: Vec<EncNumber>,
}

impl EncFile {
    fn mix(&mut self) {
        let n = self.raw.len();
        let n_low = (n - 1) as i64;

        for i in 0..n {
            let (old_position, _) =
                self.raw.iter()
                    .enumerate()
                    .find(|(_, item)| item.position == i)
                    .unwrap();

            let enc_number = self.raw.remove(old_position);
            let new_position = (((old_position as i64 + enc_number.shift) % n_low + n_low) % n_low) as usize;

            self.raw.insert(new_position, enc_number);
        }
    }

    fn decrypt(&mut self, key: i64) {
        self.raw.iter_mut()
            .for_each(|n| n.shift *= key);
    }

    fn decode(&self) -> i64 {
        let (zero_index, _) =
            self.raw.iter()
                .enumerate()
                .find(|(_, n)| n.shift == 0)
                .unwrap();

        let n = self.raw.len();

        [1000, 2000, 3000].into_iter()
            .map(|i| (zero_index + i) % n)
            .map(|i| self.raw[i].shift)
            .sum()
    }
}

impl From<&Vec<String>> for EncFile {
    fn from(value: &Vec<String>) -> Self {
        let raw =
            value.iter()
                .filter(|line| !line.is_empty())
                .map(|line| line.parse::<i64>().unwrap())
                .enumerate()
                .map(|(i, v)| EncNumber::new(i, v))
                .collect();

        EncFile { raw }
    }
}

struct EncNumber {
    position: usize,
    shift: i64,
}

impl EncNumber {
    pub fn new(position: usize, shift: i64) -> Self {
        Self { position, shift }
    }
}
