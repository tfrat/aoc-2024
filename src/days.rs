use std::fmt::Display;

pub mod one;
mod two;

pub trait Day {
    fn part_one(&self, input: &str) -> Box<dyn Display>;
    fn part_two(&self, input: &str) -> Box<dyn Display>;
}

pub fn get_day(day: &u8) -> Result<Box<dyn Day>, String> {
    match day {
        1 => Ok(Box::new(one::DayOne::default())),
        2 => Ok(Box::new(two::DayTwo::default())),
        _ => Err(format!("Day {} not supported.", day))
    }
}