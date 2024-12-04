use crate::days::Day;
use regex::Regex;

#[derive(Default)]
pub struct DayThree {}

impl DayThree {
    fn execute_instructions(line: &str, can_disable: bool) -> u32 {
        let find_instructions = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
        let find_numbers = Regex::new(r"\d{1,3}").unwrap();
        let mut on = true;
        find_instructions
            .find_iter(line)
            .map(|instruction| match (instruction.as_str(), on) {
                ("do()", _) => {
                    on = true;
                    0
                }
                ("don't()", _) => {
                    on = !can_disable;
                    0
                }
                (_, true) => find_numbers
                    .find_iter(instruction.as_str())
                    .map(|x| {
                        x.as_str()
                            .parse::<u32>()
                            .expect("Couldn't parse mul expression to integer.")
                    })
                    .product::<u32>(),
                _ => 0,
            })
            .sum()
    }
}

impl Day for DayThree {
    fn part_one(&self, input: &str) -> String {
        DayThree::execute_instructions(input, false).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        DayThree::execute_instructions(input, true).to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            DayThree::default().part_one(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ),
            161.to_string()
        )
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            DayThree::default().part_two(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            ),
            48.to_string()
        )
    }
}
