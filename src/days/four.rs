use crate::days::Day;
use regex::Regex;

#[derive(Default)]
pub struct DayFour {}

impl DayFour {
    fn count_xmas(line: &str) -> u32 {
        let re = Regex::new(r"XMAS|SAMX").unwrap();
        re.find_iter(line).count() as u32
    }

    fn get_lines(grid: &str) -> Vec<String> {
        let horizontal: Vec<String> = grid.lines().map(|line| line.to_string()).collect();
        let vertical: Vec<String> = (0..horizontal[0].len())
            .map(|i| {
                horizontal
                    .iter()
                    .map(|l| &l[i..i + 1])
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect();

        [horizontal, vertical].concat()
    }
}

impl Day for DayFour {
    fn part_one(&self, input: &str) -> String {
        let lines = Self::get_lines(input);
        lines
            .iter()
            .map(|line| Self::count_xmas(line))
            .sum::<u32>()
            .to_string()
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
        let day = DayFour::default();
        let cases = vec![(
            r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#,
            18,
        )];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayFour::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
