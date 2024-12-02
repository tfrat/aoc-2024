use crate::Part;
use std::fmt::Display;

pub mod one;
mod two;

pub trait Day {
    fn part_one(&self, input: &str) -> Box<dyn Display>;
    fn part_two(&self, input: &str) -> Box<dyn Display>;
}

pub fn get_day(day: &u8, part: &Part) -> Result<Box<dyn Day>, String> {
    match (day, part) {
        (1, _) => Ok(Box::new(one::DayOne {})),
        (2, Part::PartOne) => Ok(Box::new(two::DayTwo {
            dampener: 0,
            ..Default::default()
        })),
        (2, Part::PartTwo) => Ok(Box::new(two::DayTwo::default())),
        _ => Err(format!("Day {} not supported.", day)),
    }
}
