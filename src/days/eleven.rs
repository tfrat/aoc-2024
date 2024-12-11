use crate::days::Day;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Default)]
pub struct DayEleven {}

impl DayEleven {
    fn parse_stones(input: &str) -> HashMap<u64, u64> {
        input.split(" ").fold(HashMap::new(), |mut map, stone| {
            map.entry(stone.parse().unwrap())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            map
        })
    }

    fn blink(stone_lists: &HashMap<u64, u64>) -> HashMap<u64, u64> {
        stone_lists
            .iter()
            .fold(HashMap::new(), |mut new_map, (stone, count)| {
                let stone_string = stone.to_string();
                let new_values = match (stone, stone_string.len()) {
                    (0, _) => vec![1],
                    (_, len) if len % 2 == 0 => {
                        let (left, right) = stone_string.split_at(stone_string.len() / 2);
                        vec![left.parse().unwrap(), right.parse().unwrap()]
                    }
                    _ => vec![stone * 2024],
                };

                // Count up the new values
                new_values.iter().for_each(|value| {
                    new_map
                        .entry(*value)
                        .and_modify(|new_count| *new_count += count)
                        .or_insert(*count);
                });
                new_map
            })
    }

    fn count_stones(stones: &HashMap<u64, u64>, blinks: u32) -> u64 {
        let start = Instant::now();
        (0..blinks)
            .inspect(|blink| println!("Blink {blink}: {:?}", start.elapsed()))
            .fold(stones.clone(), |next, _| DayEleven::blink(&next))
            .values()
            .sum()
    }
}

impl Day for DayEleven {
    fn part_one(&self, input: &str) -> String {
        let stones = DayEleven::parse_stones(input);
        DayEleven::count_stones(&stones, 25).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let stones = DayEleven::parse_stones(input);
        DayEleven::count_stones(&stones, 75).to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_blink() {
        let cases = vec![
            ("2024", 2),
            ("20 24", 4),
            ("20 24 20 24 20 24 20 24", 16),
            ("20 24 20 24", 8),
            ("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32", 22),
        ];
        for (input, expected) in cases {
            let stones = DayEleven::parse_stones(input);
            assert_eq!(DayEleven::blink(&stones).values().sum::<u64>(), expected)
        }
    }

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
