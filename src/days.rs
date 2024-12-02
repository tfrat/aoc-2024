use std::fmt::Display;
use crate::days;

pub mod one;

pub trait Day {
    fn part_one(&self, input: &str) -> Box<dyn Display>;
    fn part_two(&self, input: &str) -> Box<dyn Display>;
}

pub fn get_day(day: &u8) -> Result<Box<dyn Day>, String> {
    match day {
        1 => Ok(Box::new(days::one::DayOne::default())),
        _ => Err(format!("Day {} not supported.", day))
    }
}