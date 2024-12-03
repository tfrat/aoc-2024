use crate::days::Day;
use std::fmt::Display;

#[derive(Default)]
pub struct DayXXXX {}

impl DayXXXX {}

impl Day for DayXXXX {
    fn part_one(&self, input: &str) -> Box<dyn Display> {
        Box::new(input)
    }

    fn part_two(&self, input: &str) -> Box<dyn Display> {
        Box::new(input)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DayXXXX::default();
    }

    #[test]
    fn test_part_two() {
        let day = DayXXXX::default();
    }
}
