use crate::days::Day;
use crate::utils::{Coord, Diagonal};
use regex::Regex;
use std::collections::HashMap;
use std::iter::successors;

#[derive(Debug, Clone)]
struct Robot {
    position: Coord,
    velocity: Coord,
}

impl Robot {
    fn step(&self, height: i64, width: i64) -> Self {
        let next_x = self.position.x + self.velocity.x;
        let next_y = self.position.y + self.velocity.y;

        let adjusted_x = match next_x {
            next_x if next_x < 0 => width + next_x,
            next_x if next_x >= width => next_x - width,
            _ => next_x,
        };

        let adjusted_y = match next_y {
            next_y if next_y < 0 => height + next_y,
            next_y if next_y >= height => next_y - height,
            _ => next_y,
        };

        Robot {
            position: Coord::new(adjusted_x, adjusted_y),
            velocity: self.velocity,
        }
    }

    fn quadrant(&self, height: i64, width: i64) -> Option<Diagonal> {
        let top_left = (0..width / 2, 0..height / 2);
        let top_right = (1 + width / 2..width, 0..height / 2);
        let bottom_left = (0..width / 2, 1 + height / 2..height);
        let bottom_right = (1 + width / 2..width, 1 + height / 2..height);

        match self.position {
            pos if top_left.0.contains(&pos.x) && top_left.1.contains(&pos.y) => Some(Diagonal::TL),
            pos if top_right.0.contains(&pos.x) && top_right.1.contains(&pos.y) => {
                Some(Diagonal::TR)
            }
            pos if bottom_left.0.contains(&pos.x) && bottom_left.1.contains(&pos.y) => {
                Some(Diagonal::BL)
            }
            pos if bottom_right.0.contains(&pos.x) && bottom_right.1.contains(&pos.y) => {
                Some(Diagonal::BR)
            }
            _ => None,
        }
    }
}

pub struct DayFourteen {
    width: i64,
    height: i64,
}

impl Default for DayFourteen {
    fn default() -> Self {
        DayFourteen {
            width: 101,
            height: 103,
        }
    }
}

impl DayFourteen {
    fn parse_robots(input: &str) -> Vec<Robot> {
        let find_numbers = Regex::new(r"(\d+),(\d+).+?(-*\d+),(-*\d+)").unwrap();
        input
            .lines()
            .map(|line| {
                let cap = find_numbers.captures(line).unwrap();
                Robot {
                    position: Coord::new(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                    velocity: Coord::new(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
                }
            })
            .collect()
    }

    fn wait(robots: &[Robot], seconds: u32, width: i64, height: i64) -> u32 {
        successors(Some((robots.to_owned(), 1)), |(current_robots, second)| {
            if *second > seconds {
                return None;
            }
            let next_robots = current_robots
                .iter()
                .map(|robot| robot.step(height, width))
                .collect::<Vec<Robot>>();
            Some((next_robots, second + 1))
        })
        .map(|(end_robots, _)| end_robots)
        .last()
        .unwrap()
        .iter()
        .fold(HashMap::new(), |mut counter, robot| {
            if let Some(quadrant) = robot.quadrant(height, width) {
                counter
                    .entry(quadrant)
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
            }
            counter
        })
        .values()
        .product()
    }
}

impl Day for DayFourteen {
    fn part_one(&self, input: &str) -> String {
        let robots = DayFourteen::parse_robots(input);
        DayFourteen::wait(&robots, 100, self.width, self.height).to_string()
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
        let day = DayFourteen {
            width: 11,
            height: 7,
        };
        let cases = vec![
            ("p=2,4 v=2,-3", 1),
            (
                r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#,
                12,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayFourteen::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
