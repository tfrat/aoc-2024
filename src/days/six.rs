use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use crate::days::Day;


#[derive(Eq, Hash, PartialEq, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(coords : (i32, i32)) -> Coord {
        Coord{x: coords.0, y: coords.1}
    }

    fn step(&self, direction: char) -> Coord {
        match direction {
            '^' => Coord::new((self.x , self.y - 1)),
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

struct Grid {
    grid: HashMap<(i32, i32), char>,
    pub width: u32,
    pub height: u32,
    pub starting_pos: Coord
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
            starting_pos: Coord::new(starting_pos.unwrap())
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&char> {
        self.grid.get(&(x, y))
    }

    fn get_pos(&self, coord: &Coord) -> Option<&char> {
        self.get(coord.x, coord.y)
    }

    #[allow(dead_code)]
    fn print(&self) -> String {
        (0..self.width)
            .map(|x| {
                let line = (0..self.height)
                    .map(|y| self.get(x as i32, y as i32).unwrap_or(&' '))
                    .collect::<String>();
                line + "\n"
            })
            .collect()
    }

    #[allow(dead_code)]
    fn print_around(&self, x: i32, y: i32) -> String {
        (-1..=1)
            .map(|x_offset| {
                let line = (-1..=1)
                    .map(|y_offset| self.get(x + x_offset, y + y_offset).unwrap_or(&' '))
                    .collect::<String>();
                line + "\n"
            })
            .collect()
    }
}


#[derive(Default)]
pub struct DaySix {}

impl DaySix {
    fn count_guard_steps(grid: &Grid) -> u32 {
        let mut guard_pos = grid.starting_pos.clone();
        let mut positions: HashSet<Coord> = HashSet::new();
        let directions = ['^', '>', 'v', '<'];
        let mut direction = directions.iter().position(|dir| dir == grid.get_pos(&grid.starting_pos).unwrap()).unwrap();
        let mut next_pos = guard_pos.step(directions[direction]);
        while let Some(place) = grid.get_pos(&next_pos) {
            positions.insert(guard_pos.clone());
            match place {
                '#' => direction = (direction + 1) % directions.len(),
                _ => guard_pos = next_pos.clone()
            }
            next_pos = guard_pos.step(directions[direction])
        }
        positions.insert(guard_pos);
        positions.len() as u32
    }
}

impl Day for DaySix {
    fn part_one(&self, input: &str) -> String {
        let grid = Grid::new(input);
        DaySix::count_guard_steps(&grid).to_string()
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
        let day = DaySix::default();
        let cases = vec![(r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#, 41)];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DaySix::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
