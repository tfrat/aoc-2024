use crate::days::Day;

#[derive(Default)]
pub struct DaySeven {}

impl DaySeven {}

impl Day for DaySeven {
    fn part_one(&self, input: &str) -> String {
        input.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        input.to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DaySeven::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DaySeven::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
