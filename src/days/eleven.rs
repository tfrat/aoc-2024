use crate::days::Day;

#[derive(Default)]
pub struct DayEleven {}

impl DayEleven {
    fn parse_stones(input: &str) -> Vec<u64> {
        input
            .split(" ")
            .map(|stone| stone.parse().unwrap())
            .collect()
    }

    fn blink(stones: &[u64]) -> Vec<u64> {
        stones
            .iter()
            .flat_map(|stone| {
                let stone_string = stone.to_string();
                match (stone, stone_string.len()) {
                    (0, _) => vec![1],
                    (_, len) if len % 2 == 0 => {
                        let (left, right) = stone_string.split_at(stone_string.len() / 2);
                        vec![left.parse().unwrap(), right.parse().unwrap()]
                    }
                    _ => vec![stone * 2024],
                }
            })
            .collect()
    }

    fn count_stones(stones: &[u64], blinks: u32) -> u32 {
        (0..blinks)
            .fold(stones.to_owned(), |next, _| DayEleven::blink(&next))
            .len() as u32
    }
}

impl Day for DayEleven {
    fn part_one(&self, input: &str) -> String {
        let stones = DayEleven::parse_stones(input);
        DayEleven::count_stones(&stones, 25).to_string()
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
        let day = DayEleven::default();
        let cases = vec![("125 17", 55312)];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayEleven::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
