use crate::days::Day;
use regex::Regex;
use std::fmt::Display;

#[derive(Default)]
pub struct DayThree {}

impl DayThree {
    fn execute_instructions(line: &str) -> u32 {
        let find_instructions = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
        let find_numbers = Regex::new(r"\d{1,3}").unwrap();
        find_instructions
            .find_iter(line)
            .map(|instruction| {
                find_numbers
                    .find_iter(instruction.as_str())
                    .map(|x| x.as_str().parse::<u32>().expect(""))
                    .product::<u32>()
            })
            .sum()
    }
}

impl Day for DayThree {
    fn part_one(&self, input: &str) -> Box<dyn Display> {
        Box::new(DayThree::execute_instructions(input))
    }

    fn part_two(&self, _input: &str) -> Box<dyn Display> {
        todo!()
    }
}
