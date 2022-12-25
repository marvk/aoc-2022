use crate::harness::{Day, EmptyPart, Part};

pub fn day25() -> Day<String, String> {
    Day::new(25, Box::new(Part1 {}), Box::new(EmptyPart {}))
}

pub struct Part1;

const RADIX: i64 = 5;

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
                    let f1 = RADIX.pow(i as u32);
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
            let current_pow = RADIX.pow(i);
            let next_pow = RADIX.pow(i + 1);

            let remainder = sum % next_pow;
            let multiple = remainder / current_pow;

            let digit = match multiple {
                0 | 1 | 2 => multiple,
                3 => -2,
                4 => -1,
                _ => panic!(),
            };

            let char = match digit {
                0 | 1 | 2 => char::from_digit(digit as u32, 10).unwrap(),
                -2 => '=',
                -1 => '-',
                _ => panic!(),
            };

            result.insert(0, char);
            sum -= digit * current_pow;

            if sum == 0 {
                break;
            }
        }

        result
    }
}
