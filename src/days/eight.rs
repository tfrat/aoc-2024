use crate::days::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn plus(&self, other: &Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn minus(&self, other: &Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Default)]
pub struct DayEight {}

impl DayEight {
    fn find_antinodes(antenna_one: &Coord, antenna_two: &Coord, br: &Coord) -> HashSet<Coord> {
        let mut antinodes = HashSet::new();

        let delta_coord = Coord {
            x: antenna_two.x - antenna_one.x,
            y: antenna_two.y - antenna_one.y,
        };
        let mut next_coord = antenna_one.minus(&delta_coord);
        while next_coord.x < br.x && next_coord.x >= 0 && next_coord.y < br.y && next_coord.y >= 0 {
            antinodes.insert(next_coord.clone());
            next_coord = next_coord.minus(&delta_coord);
        }
        next_coord = antenna_two.plus(&delta_coord);
        while next_coord.x < br.x && next_coord.x >= 0 && next_coord.y < br.y && next_coord.y >= 0 {
            antinodes.insert(next_coord.clone());
            next_coord = next_coord.plus(&delta_coord);
        }
        antinodes
    }
    fn find_antenna(input: &str) -> HashMap<char, HashSet<Coord>> {
        let mut antennae_positions: HashMap<char, HashSet<Coord>> = HashMap::new();
        for (y, line) in input
            .lines()
            .enumerate()
            .map(|(index, item)| (index as i32, item))
        {
            for (x, space) in line
                .chars()
                .enumerate()
                .map(|(index, item)| (index as i32, item))
            {
                match space {
                    '.' => (),
                    _ => {
                        antennae_positions
                            .entry(space)
                            .or_default()
                            .insert(Coord { x, y });
                    }
                }
            }
        }
        antennae_positions
    }

    fn find_all_antinodes(map_size: &Coord, antennae: &HashMap<char, HashSet<Coord>>) -> u32 {
        antennae
            .values()
            .flat_map(|positions| {
                positions
                    .iter()
                    .permutations(2)
                    .flat_map(|pair| DayEight::find_antinodes(pair[0], pair[1], map_size))
            })
            .filter(|coord| {
                coord.x < map_size.x && coord.x >= 0 && coord.y < map_size.y && coord.y >= 0
            })
            .collect::<HashSet<Coord>>()
            .len() as u32
    }
}

impl Day for DayEight {
    fn part_one(&self, input: &str) -> String {
        let positions = DayEight::find_antenna(input);
        let (x, y) = (
            input.lines().collect::<Vec<_>>()[0].len(),
            input.lines().count(),
        );
        DayEight::find_all_antinodes(
            &Coord {
                x: x as i32,
                y: y as i32,
            },
            &positions,
        )
        .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let positions = DayEight::find_antenna(input);
        let (x, y) = (
            input.lines().collect::<Vec<_>>()[0].len(),
            input.lines().count(),
        );
        DayEight::find_all_antinodes(
            &Coord {
                x: x as i32,
                y: y as i32,
            },
            &positions,
        )
        .to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_find_antinode() {
        let cases = vec![(
            Coord { x: 0, y: 0 },
            Coord { x: 1, y: 1 },
            HashSet::from_iter(
                [Coord { x: -1, y: -1 }, Coord { x: 2, y: 2 }]
                    .iter()
                    .cloned(),
            ),
        )];
        for (left, right, expected) in cases {
            assert_eq!(
                DayEight::find_antinodes(&left, &right, &Coord { x: 3, y: 3 }),
                expected
            )
        }
    }

    #[test]
    fn test_part_one() {
        let day = DayEight::default();
        let cases = vec![(
            r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#,
            14,
        )];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayEight::default();
        let cases = vec![(
            r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#,
            34,
        )];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
