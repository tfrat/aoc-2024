use crate::days::Day;
use crate::utils::{Coord, Direction, Grid};

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

#[derive(Default)]
pub struct DayFifteen {}

impl DayFifteen {
    fn parse_factory(input: &str) -> (Grid<Object>, Vec<Direction>) {
        let (factory_input, moves_input) = input.split_once("\n\n").unwrap();

        let factory =
            factory_input
                .lines()
                .enumerate()
                .fold(Grid::new(), |mut factory, (y, line)| {
                    line.chars().enumerate().for_each(|(x, letter)| {
                        factory.set(Coord::new(x as i64, y as i64), Object::new(&letter));
                    });
                    factory
                });

        let moves = moves_input
            .lines()
            .flat_map(|line| line.chars())
            .flat_map(|letter| Direction::new(&letter))
            .collect();

        (factory, moves)
    }

    fn execute_moves(factory: &mut Grid<Object>, moves: &[Direction]) -> u32 {
        // todo
        (factory.top_left.x + moves.len() as i64) as u32
    }
}

impl Day for DayFifteen {
    fn part_one(&self, input: &str) -> String {
        let (mut factory, moves) = DayFifteen::parse_factory(input);
        DayFifteen::execute_moves(&mut factory, &moves).to_string()
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
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
