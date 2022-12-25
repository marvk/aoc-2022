use crate::harness::{Day, Part};

pub fn day25() -> Day<String, ()> {
    Day::new(25, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<String> for Part1 {
    fn expect_test(&self) -> String {
        "2=-1=0".to_string()
    }

    fn solve(&self, input: &Vec<String>) -> String {
        let mut sum =
            input.iter()
                .filter(|line| !line.is_empty())
                .flat_map(|line| line.chars().rev().enumerate())
                .map(|(i, char)| {
                    let f1 = 5_i64.pow(i as u32);
                    let f2 = match char {
                        '2' => 2,
                        '1' => 1,
                        '0' => 0,
                        '-' => -1,
                        '=' => -2,
                        _ => panic!()
                    };
                    f1 * f2
                })
                .sum::<i64>();

        let mut result = String::new();

        for i in 0.. {
            let current_pow = 5_i64.pow(i);
            let next_pow = 5_i64.pow(i + 1);
            let remainder = sum % next_pow;
            let count = remainder / current_pow;
            let amount = match count {
                0 | 1 | 2 => count,
                3 => -2,
                4 => -1,
                _ => panic!(),
            };

            let char = match amount {
                0 | 1 | 2 => char::from_digit(amount as u32, 10).unwrap(),
                -2 => '=',
                -1 => '-',
                _ => panic!(),
            };

            result.push(char);

            sum -= amount * current_pow;

            if sum == 0 {
                break;
            }
        }

        result.chars().rev().collect()
    }
}

pub struct Part2;

impl Part<()> for Part2 {
    fn expect_test(&self) -> () {
        ()
    }

    fn solve(&self, _: &Vec<String>) -> () {
        ()
    }
}
