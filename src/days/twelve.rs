use crate::days::Day;
use crate::utils::{Coord, Grid};
use std::collections::{HashMap, HashSet};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Default, Debug)]
struct Region {
    area: u32,
    perimeter: u32,
}

impl Region {
    fn new(area: u32, perimeter: u32) -> Region {
        Region { area, perimeter }
    }

    fn cost(&self) -> u32 {
        self.area * self.perimeter
    }

    fn plus(&self, other: &Region) -> Region {
        Region {
            area: self.area + other.area,
            perimeter: self.perimeter + other.perimeter,
        }
    }
}

#[derive(Default)]
pub struct DayTwelve {}

impl DayTwelve {
    fn parse_farm(input: &str) -> Grid<char> {
        input
            .lines()
            .enumerate()
            .fold(Grid::new(), |mut grid, (y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, plant)| {
                        (
                            Coord {
                                x: x as i32,
                                y: y as i32,
                            },
                            plant,
                        )
                    })
                    .for_each(|(coord, plant)| grid.set(coord, plant));
                grid
            })
    }

    fn visit(
        farm: &Grid<char>,
        plant: &char,
        coord: &Coord,
        visited: &mut HashSet<Coord>,
    ) -> Region {
        let region = match (farm.get(coord), visited.contains(coord)) {
            (Some(neighbor), false) if *neighbor == *plant => Region::new(1, 0),
            (Some(neighbor), _) if *neighbor != *plant => {
                return Region::new(0, 1);
            }
            (None, _) => {
                return Region::new(0, 1);
            }
            _ => return Region::default(),
        };
        visited.insert(*coord);
        region.plus(
            &([
                coord.plus_x(1),
                coord.plus_x(-1),
                coord.plus_y(1),
                coord.plus_y(-1),
            ]
            .iter()
            .map(|next| Self::visit(farm, plant, next, visited))
            .reduce(|current, next| current.plus(&next))
            .unwrap()),
        )
    }

    fn calculate_fence_amount(farm: &Grid<char>) -> u32 {
        // let mut visited_regions: HashMap<char, HashSet<Coord>> = HashMap::new();
        let mut visited: HashSet<Coord> = HashSet::new();
        farm.iter()
            .fold(HashMap::new(), |mut regions, (coord, plant)| {
                let region = DayTwelve::visit(farm, plant, coord, &mut visited);
                if region.cost() > 0 {
                    let plant_regions: &mut Vec<Region> = regions.entry(plant).or_default();
                    plant_regions.push(region);
                }
                regions
            })
            .values()
            .flatten()
            .map(|region| region.cost())
            .sum()
    }
}

impl Day for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        let farm = DayTwelve::parse_farm(input);
        DayTwelve::calculate_fence_amount(&farm).to_string()
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
        let day = DayTwelve::default();
        let cases = vec![
            ("A", 4),
            (
                /*
                AAAA
                BBCD
                BBCC
                EEEC
                 */
                r#"AAAA
BBCD
BBCC
EEEC"#,
                140,
            ),
            (
                r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#,
                772,
            ),
            (
                r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#,
                1930,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayTwelve::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
