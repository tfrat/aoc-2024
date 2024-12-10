use crate::days::Day;
use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn plus_x(&self, offset: i32) -> Coord {
        Coord {
            x: self.x + offset,
            y: self.y,
        }
    }

    fn plus_y(&self, offset: i32) -> Coord {
        Coord {
            x: self.x,
            y: self.y + offset,
        }
    }
}

#[derive(Clone)]
struct Grid {
    grid: HashMap<(i32, i32), i32>,
    trailheads: Vec<Coord>,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut grid = HashMap::new();
        let mut trailheads: Vec<Coord> = vec![];
        for (y, line) in input.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                if value == '0' {
                    trailheads.push(Coord {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                grid.insert((x as i32, y as i32), value.to_string().parse().unwrap());
            }
        }
        Grid { grid, trailheads }
    }

    fn get(&self, coord: &Coord) -> Option<&i32> {
        self.grid.get(&(coord.x, coord.y))
    }
}
#[derive(Default)]
pub struct DayTen {}

impl DayTen {
    fn count_hiking_trails(pos: &Coord, map: &Grid) -> HashSet<Coord> {
        let elevation = match map.get(pos) {
            Some(elevation) => elevation,
            None => return HashSet::new(),
        };

        if *elevation == 9 {
            return HashSet::from([pos.clone()]);
        }
        [pos.plus_x(1), pos.plus_x(-1), pos.plus_y(1), pos.plus_y(-1)]
            .iter()
            .filter(|next_pos| {
                map.get(next_pos)
                    .is_some_and(|value| *value == *elevation + 1)
            })
            .map(|next_pos| DayTen::count_hiking_trails(next_pos, map))
            .fold(HashSet::new(), |mut acc, next| {
                acc.extend(next);
                acc
            })
    }

    fn count_hiking_trail_rating(pos: &Coord, map: &Grid) -> u32 {
        let elevation = match map.get(pos) {
            Some(elevation) => elevation,
            None => return 0,
        };

        if *elevation == 9 {
            return 1;
        }

        [pos.plus_x(1), pos.plus_x(-1), pos.plus_y(1), pos.plus_y(-1)]
            .iter()
            .filter(|next_pos| {
                map.get(next_pos)
                    .is_some_and(|value| *value == *elevation + 1)
            })
            .map(|next_pos| DayTen::count_hiking_trail_rating(next_pos, map))
            .sum()
    }

    fn count_good_trailheads(map: &Grid) -> u32 {
        map.trailheads
            .iter()
            .map(|trailhead| Self::count_hiking_trails(trailhead, map).len() as u32)
            .sum()
    }

    fn count_good_trailheads_rating(map: &Grid) -> u32 {
        map.trailheads
            .iter()
            .map(|trailhead| Self::count_hiking_trail_rating(trailhead, map))
            .sum()
    }
}

impl Day for DayTen {
    fn part_one(&self, input: &str) -> String {
        let map = Grid::new(input);

        DayTen::count_good_trailheads(&map).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let map = Grid::new(input);

        DayTen::count_good_trailheads_rating(&map).to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DayTen::default();
        let cases = vec![
            (
                r#"0123
1234
8765
9876"#,
                1,
            ),
            (
                r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#,
                36,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayTen::default();
        let cases = vec![
            (
                r#"0123
1234
8765
9876"#,
                16,
            ),
            (
                r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#,
                81,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
