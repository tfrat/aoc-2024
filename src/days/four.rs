use crate::days::Day;
use std::collections::HashMap;

struct Grid {
    grid: HashMap<(i32, i32), char>,
    pub width: u32,
    pub height: u32,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut grid = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                grid.insert((x as i32, y as i32), value);
            }
        }
        let height = input.lines().collect::<Vec<_>>().len() as u32;
        let width: u32 = input.lines().collect::<Vec<_>>()[0].len() as u32;
        Grid {
            grid,
            height,
            width,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&char> {
        self.grid.get(&(x, y))
    }
}

#[derive(Default)]
pub struct DayFour {}

impl DayFour {
    fn check_all_directions(grid: &Grid, x: i32, y: i32, xmas: &str) -> u32 {
        (-1..=1)
            .map(|x_offset| {
                (-1..=1)
                    .map(|y_offset| match (x_offset, y_offset) {
                        (0, 0) => 0,
                        _ => Self::count_xmas(
                            grid,
                            x + x_offset,
                            y + y_offset,
                            x_offset,
                            y_offset,
                            xmas,
                        ),
                    })
                    .sum::<u32>()
            })
            .sum()
    }

    fn count_xmas(grid: &Grid, x: i32, y: i32, x_offset: i32, y_offset: i32, xmas: &str) -> u32 {
        match (grid.get(x, y), xmas) {
            (Some('X'), "") => Self::check_all_directions(grid, x, y, "X"),
            (Some('M'), "X") => {
                Self::count_xmas(grid, x + x_offset, y + y_offset, x_offset, y_offset, "XM")
            }
            (Some('A'), "XM") => {
                Self::count_xmas(grid, x + x_offset, y + y_offset, x_offset, y_offset, "XMA")
            }
            (Some('S'), "XMA") => 1,
            _ => 0,
        }
    }
}

impl Day for DayFour {
    fn part_one(&self, input: &str) -> String {
        let grid = Grid::new(input);
        (0..grid.width)
            .map(|x| {
                (0..grid.height)
                    .map(|y| Self::count_xmas(&grid, x as i32, y as i32, 0, 0, ""))
                    .sum::<u32>()
            })
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
        let cases = vec![
            (
                r#"..X...
.SAMX.
.A..A.
XMAS.S
.X...."#,
                4,
            ),
            (
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
            ),
        ];
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
