use crate::days::Day;

#[derive(Default)]
pub struct DaySeven {}

impl DaySeven {
    fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
        input
            .lines()
            .map(|line| {
                let (result, numbers_strings) = line.split_once(":").unwrap();
                let numbers: Vec<u64> = numbers_strings
                    .split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect();
                (result.parse::<u64>().unwrap(), numbers)
            })
            .collect()
    }

    fn find_solvable_sum(operators: &Vec<char>, result: &u64, numbers: &[u64], progress: &u64) -> u64 {
        if progress == result {
            return *result
        }
        if numbers.len() == 0 {
            return 0
        }
        operators.iter().map(|operator| {
            match operator {
                '*' => Self::find_solvable_sum(operators, result, &numbers[1..], &(progress * numbers[0])),
                '+' => Self::find_solvable_sum(operators, result, &numbers[1..], &(progress + numbers[0])),
                '|' => Self::find_solvable_sum(operators, result, &numbers[1..], &(format!("{progress}{}", numbers[0]).parse().unwrap())),
                _ => 0,
            }
        }).filter(|result| *result > 0u64).next().unwrap_or(0)
    }
}

impl Day for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let problems = DaySeven::parse_input(input);
        problems
            .iter()
            .map(|(result, numbers)| {
                if DaySeven::find_solvable_sum(&vec!['*', '+'], result, &numbers[1..], &numbers[0]) > 0 {
                    return result
                }
                &0u64
            })
            .sum::<u64>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let problems = DaySeven::parse_input(input);
        problems
            .iter()
            .map(|(result, numbers)| {
                if DaySeven::find_solvable_sum(&vec!['*', '+', '|'], result, &numbers[1..], &numbers[0]) > 0 {
                    return result
                }
                &0u64
            })
            .sum::<u64>()
            .to_string()    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DaySeven::default();
        let cases = vec![(
            r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#,
            3749,
        )];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DaySeven::default();
        let cases = vec![(
            r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#,
            11387,
        )];        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
