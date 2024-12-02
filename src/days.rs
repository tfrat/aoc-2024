use std::fmt::Display;

pub mod day_1;

pub trait Day {
    fn part_one(&self, input: &str) -> impl Display;
    fn part_two(&self, input: &str) -> impl Display;
}