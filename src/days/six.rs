use crate::days::Day;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::thread;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(coords: (i32, i32)) -> Coord {
        Coord {
            x: coords.0,
            y: coords.1,
        }
    }

    fn step(&self, direction: char) -> Coord {
        match direction {
            '^' => Coord::new((self.x, self.y - 1)),
            '>' => Coord::new((self.x + 1, self.y)),
            'v' => Coord::new((self.x, self.y + 1)),
            '<' => Coord::new((self.x - 1, self.y)),
            _ => Coord::new((self.x, self.y)),
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone)]
struct Grid {
    grid: HashMap<(i32, i32), char>,
    width: u32,
    height: u32,
    starting_pos: Coord,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut grid = HashMap::new();
        let mut starting_pos: Option<(i32, i32)> = None;
        for (y, line) in input.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                match value {
                    '^' | '>' | 'v' | '<' => starting_pos = Some((x as i32, y as i32)),
                    _ => (),
                }
                grid.insert((x as i32, y as i32), value);
            }
        }
        let height = input.lines().collect::<Vec<_>>().len() as u32;
        let width: u32 = input.lines().collect::<Vec<_>>()[0].len() as u32;
        Grid {
            grid,
            height,
            width,
            starting_pos: Coord::new(starting_pos.unwrap()),
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&char> {
        self.grid.get(&(x, y))
    }

    fn set(&mut self, coord: &Coord, letter: char) {
        self.grid.insert((coord.x, coord.y), letter);
    }

    fn get_pos(&self, coord: &Coord) -> Option<&char> {
        self.get(coord.x, coord.y)
    }

    #[allow(dead_code)]
    fn print(&self) -> String {
        (0..self.width)
            .map(|y| {
                let line = (0..self.height)
                    .map(|x| self.get(x as i32, y as i32).unwrap_or(&' '))
                    .collect::<String>();
                line + "\n"
            })
            .collect()
    }

    #[allow(dead_code)]
    fn print_around(&self, x: i32, y: i32, window_length: i32) -> String {
        (-window_length..=window_length)
            .map(|y_offset| {
                let line = (-window_length..=window_length)
                    .map(|x_offset| self.get(x + x_offset, y + y_offset).unwrap_or(&' '))
                    .collect::<String>();
                line + "\n"
            })
            .collect()
    }
}

#[derive(Default)]
pub struct DaySix {}

impl DaySix {
    fn count_guard_steps(grid: &Grid) -> Option<u32> {
        let mut guard_pos = grid.starting_pos.clone();
        let mut positions: HashSet<(Coord, char)> = HashSet::new();
        let directions = ['^', '>', 'v', '<'];
        let mut direction = directions
            .iter()
            .position(|dir| dir == grid.get_pos(&grid.starting_pos).unwrap())?;
        let mut next_pos = guard_pos.step(directions[direction]);
        while let Some(place) = grid.get_pos(&next_pos) {
            match place {
                '#' | '0' => direction = (direction + 1) % directions.len(),
                _ if positions.contains(&(guard_pos.clone(), directions[direction])) => {
                    return None;
                }
                _ => {
                    positions.insert((guard_pos.clone(), directions[direction]));
                    guard_pos = next_pos.clone()
                }
            }
            next_pos = guard_pos.step(directions[direction])
        }
        positions.insert((guard_pos, directions[direction]));
        Some(
            positions
                .iter()
                .map(|pos| pos.0.clone())
                .collect::<HashSet<Coord>>()
                .len() as u32,
        )
    }

    fn find_obstruction_count(grid: &Grid) -> u32 {
        let mut guard_pos = grid.starting_pos.clone();
        let directions = ['^', '>', 'v', '<'];
        let mut direction = directions
            .iter()
            .position(|dir| dir == grid.get_pos(&grid.starting_pos).unwrap())
            .unwrap();
        let mut next_pos = guard_pos.step(directions[direction]);
        let mut handles = vec![];
        while let Some(place) = grid.get_pos(&next_pos) {
            if place != &'#' {
                let mut new_grid = grid.clone();
                new_grid.set(&next_pos, '0');
                let new_obs_pos = next_pos.clone();
                handles.push(thread::spawn(move || {
                    match Self::count_guard_steps(&new_grid) {
                        Some(_) => None,
                        None => Some(new_obs_pos),
                    }
                }));
            }

            match place {
                '#' => direction = (direction + 1) % directions.len(),
                _ => guard_pos = next_pos.clone(),
            }
            next_pos = guard_pos.step(directions[direction])
        }

        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .filter(|coord| !coord.is_none())
            .map(|coord| coord.unwrap())
            .collect::<HashSet<Coord>>()
            .len() as u32
    }
}

impl Day for DaySix {
    fn part_one(&self, input: &str) -> String {
        let grid = Grid::new(input);
        DaySix::count_guard_steps(&grid).unwrap().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let grid = Grid::new(input);
        DaySix::find_obstruction_count(&grid).to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DaySix::default();
        let cases = vec![(
            r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#,
            41,
        )];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DaySix::default();
        let cases = vec![(
            r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#,
            6,
        )];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
