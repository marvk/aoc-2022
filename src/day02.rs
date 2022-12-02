use crate::harness::{Day, Part};

pub fn day02() -> Day<u32, u32> {
    Day::new(2, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

static LOSS_SCORE: u32 = 0;
static DRAW_SCORE: u32 = 3;
static WIN_SCORE: u32 = 6;

static ROCK_SCORE: u32 = 1;
static PAPER_SCORE: u32 = 2;
static SCISSORS_SCORE: u32 = 3;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        15
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        sum(input, Self::score)
    }
}

impl Part1 {
    fn score(a: char, b: char) -> u32 {
        match (a, b) {
            ('C', 'A') => ROCK_SCORE + WIN_SCORE,
            ('A', 'A') => ROCK_SCORE + DRAW_SCORE,
            ('B', 'A') => ROCK_SCORE + LOSS_SCORE,
            ('A', 'B') => PAPER_SCORE + WIN_SCORE,
            ('B', 'B') => PAPER_SCORE + DRAW_SCORE,
            ('C', 'B') => PAPER_SCORE + LOSS_SCORE,
            ('B', 'C') => SCISSORS_SCORE + WIN_SCORE,
            ('C', 'C') => SCISSORS_SCORE + DRAW_SCORE,
            ('A', 'C') => SCISSORS_SCORE + LOSS_SCORE,
            (_, _) => panic!(),
        }
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        12
    }

    fn solve(&self, input: &Vec<String>) -> u32 {
        sum(input, Self::score)
    }
}

impl Part2 {
    fn score(a: char, b: char) -> u32 {
        match (a, b) {
            ('C', 'C') => ROCK_SCORE + WIN_SCORE,
            ('A', 'B') => ROCK_SCORE + DRAW_SCORE,
            ('B', 'A') => ROCK_SCORE + LOSS_SCORE,
            ('A', 'C') => PAPER_SCORE + WIN_SCORE,
            ('B', 'B') => PAPER_SCORE + DRAW_SCORE,
            ('C', 'A') => PAPER_SCORE + LOSS_SCORE,
            ('B', 'C') => SCISSORS_SCORE + WIN_SCORE,
            ('C', 'B') => SCISSORS_SCORE + DRAW_SCORE,
            ('A', 'A') => SCISSORS_SCORE + LOSS_SCORE,
            (_, _) => panic!(),
        }
    }
}

fn sum<FScore: Fn(char, char) -> u32>(input: &Vec<String>, calculate_score: FScore) -> u32 {
    input
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes())
        .map(|line| calculate_score(line[0] as char, (line[2] - 'X' as u8 + 'A' as u8) as char))
        .sum()
}
