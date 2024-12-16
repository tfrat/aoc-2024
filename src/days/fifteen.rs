use crate::days::Day;
use crate::utils::{Coord, Direction, Grid};
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Object {
    Robot,
    Box,
    Wall,
    Empty,
}

impl Object {
    fn new(letter: &char) -> Self {
        match letter {
            '@' => Self::Robot,
            'O' => Self::Box,
            '#' => Self::Wall,
            _ => Self::Empty,
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Empty => write!(f, "."),
            Object::Wall => write!(f, "#"),
            Object::Box => write!(f, "O"),
            Object::Robot => write!(f, "@"),
        }
    }
}

#[derive(Default)]
pub struct DayFifteen {}

impl DayFifteen {
    fn parse_factory(input: &str, width: u8) -> (Coord, Grid<Object>, Vec<Direction>) {
        let (factory_input, moves_input) = input.split_once("\n\n").unwrap();

        let factory =
            factory_input
                .lines()
                .enumerate()
                .fold(Grid::new(), |mut factory, (y, line)| {
                    line.chars().enumerate().for_each(|(x, letter)| {
                        let new_x = x * width as usize;
                        factory.set(Coord::new(new_x as i64, y as i64), Object::new(&letter));
                        for offset in 1..width {
                            if letter == '@' {
                                factory.set(
                                    Coord::new((new_x + offset as usize) as i64, y as i64),
                                    Object::Empty,
                                );
                            } else {
                                factory.set(
                                    Coord::new((new_x + offset as usize) as i64, y as i64),
                                    Object::new(&letter),
                                );
                            }
                        }
                    });
                    factory
                });

        let robot = {
            factory
                .iter()
                .find(|(_, obj)| **obj == Object::Robot)
                .unwrap()
                .0
        };

        let moves = moves_input
            .lines()
            .flat_map(|line| line.chars())
            .flat_map(|letter| Direction::new(&letter))
            .collect();

        (*robot, factory, moves)
    }

    fn score_factory(factory: &Grid<Object>) -> i64 {
        factory
            .iter()
            .filter(|(_, object)| **object == Object::Box)
            .map(|(position, _)| position.x + position.y * 100)
            .sum()
    }

    fn find_moves(
        position: &Coord,
        factory: &mut Grid<Object>,
        direction: &Direction,
    ) -> Option<Vec<(Coord, Object)>> {
        let next = match direction {
            Direction::Up => position.plus_y(-1),
            Direction::Down => position.plus_y(1),
            Direction::Left => position.plus_x(-1),
            Direction::Right => position.plus_x(1),
        };
        let next_obj = factory.get(&next)?;

        if *next_obj == Object::Wall {
            return None;
        }

        let current_obj = *factory.get(position)?;

        if *next_obj == Object::Empty {
            return Some(vec![(next, current_obj), (*position, Object::Empty)]);
        }

        match Self::find_moves(&next, factory, direction) {
            None => None,
            Some(moves) => {
                let mut total_moves = moves.clone();
                total_moves.extend(vec![(next, current_obj), (*position, Object::Empty)]);
                Some(total_moves)
            }
        }
    }

    fn execute_moves(robot: &Coord, factory: &mut Grid<Object>, moves: &[Direction]) -> i64 {
        let mut current = *robot;
        for direction in moves {
            if let Some(next) = Self::find_moves(&current, factory, direction) {
                for (coord, obj) in next {
                    factory.set(coord, obj);
                    if obj == Object::Robot {
                        current = coord;
                    }
                }
            }
        }

        DayFifteen::score_factory(factory)
    }
}

impl Day for DayFifteen {
    fn part_one(&self, input: &str) -> String {
        let (robot, mut factory, moves) = DayFifteen::parse_factory(input, 1);
        println!("{factory:?}");
        DayFifteen::execute_moves(&robot, &mut factory, &moves).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (_robot, factory, _moves) = DayFifteen::parse_factory(input, 2);
        println!("{factory:?}");
        input.to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DayFifteen::default();
        let cases = vec![
            (
                r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#,
                2028,
            ),
            (
                r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#,
                10092,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayFifteen::default();
        let cases = vec![
            (
                r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#,
                618,
            ),
            (
                r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#,
                9021,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
