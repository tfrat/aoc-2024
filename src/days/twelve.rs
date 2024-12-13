use crate::days::Day;
use crate::utils::{Coord, Grid};
use std::collections::{HashMap, HashSet};

enum Diagonal {
    TL,
    TR,
    BR,
    BL,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Default, Debug)]
struct Region {
    area: u32,
    perimeter: u32,
    corners: u32,
}

impl Region {
    fn new(area: u32, perimeter: u32, corners: u32) -> Region {
        Region {
            area,
            perimeter,
            corners,
        }
    }

    fn cost(&self) -> u32 {
        self.area * self.perimeter
    }

    fn side_cost(&self) -> u32 {
        self.area * self.corners
    }

    fn plus(&self, other: &Region) -> Region {
        Region {
            area: self.area + other.area,
            perimeter: self.perimeter + other.perimeter,
            corners: self.corners + other.corners,
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

    fn is_interior_corner(
        plant: &char,
        coord: &Coord,
        diagonal: &Diagonal,
        grid: &Grid<char>,
        neighbor_plant: Option<&char>,
    ) -> bool {
        let (one, two) = match diagonal {
            Diagonal::TL => (
                grid.get(&coord.plus_x(-1)).unwrap_or(&'+'),
                grid.get(&coord.plus_y(1)).unwrap_or(&'+'),
            ),
            Diagonal::TR => (
                grid.get(&coord.plus_x(1)).unwrap_or(&'+'),
                grid.get(&coord.plus_y(1)).unwrap_or(&'+'),
            ),
            Diagonal::BR => (
                grid.get(&coord.plus_x(1)).unwrap_or(&'+'),
                grid.get(&coord.plus_y(-1)).unwrap_or(&'+'),
            ),
            Diagonal::BL => (
                grid.get(&coord.plus_x(-1)).unwrap_or(&'+'),
                grid.get(&coord.plus_y(-1)).unwrap_or(&'+'),
            ),
        };
        one != plant
            && neighbor_plant.is_none_or(|neighbor| neighbor == one)
            && two != plant
            && neighbor_plant.is_none_or(|neighbor| neighbor == two)
    }

    fn is_anterior_corner(
        plant: &char,
        coord: &Coord,
        diagonal: &Diagonal,
        grid: &Grid<char>,
    ) -> bool {
        let (next, opposite) = match diagonal {
            Diagonal::TL => (coord.plus(-1, 1), Diagonal::BR),
            Diagonal::TR => (coord.plus(1, 1), Diagonal::BL),
            Diagonal::BR => (coord.plus(1, -1), Diagonal::TL),
            Diagonal::BL => (coord.plus(-1, -1), Diagonal::TR),
        };
        let next_plant = grid.get(&next);
        next_plant.is_none_or(|other| other != plant)
            && Self::is_interior_corner(
                next_plant.unwrap_or(&'+'),
                &next,
                &opposite,
                grid,
                Some(plant),
            )
    }

    fn visit(
        farm: &Grid<char>,
        plant: &char,
        coord: &Coord,
        visited: &mut HashSet<Coord>,
    ) -> Region {
        let region = match (farm.get(coord), visited.contains(coord)) {
            (Some(neighbor), false) if *neighbor == *plant => Region::new(1, 0, 0),
            (Some(neighbor), _) if *neighbor != *plant => {
                return Region::new(0, 1, 0);
            }
            (None, _) => {
                return Region::new(0, 1, 0);
            }
            _ => return Region::default(),
        };
        visited.insert(*coord);

        let corners = [Diagonal::TR, Diagonal::BL, Diagonal::BR, Diagonal::TL]
            .iter()
            .filter(|diagonal| {
                Self::is_interior_corner(plant, coord, diagonal, farm, None)
                    || Self::is_anterior_corner(plant, coord, diagonal, farm)
            })
            .count() as u32;

        region.plus(&Region::new(0, 0, corners)).plus(
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

    fn calculate_fence_amount(farm: &Grid<char>) -> (u32, u32) {
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
            .map(|region| (region.cost(), region.side_cost()))
            .reduce(|(acc_cost, acc_side_cost), (next_cost, next_side_cost)| {
                (acc_cost + next_cost, acc_side_cost + next_side_cost)
            })
            .unwrap()
    }
}

impl Day for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        let farm = DayTwelve::parse_farm(input);
        DayTwelve::calculate_fence_amount(&farm).0.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let farm = DayTwelve::parse_farm(input);
        DayTwelve::calculate_fence_amount(&farm).1.to_string()
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
                1206,
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayTwelve::default();
        let cases = vec![
            ("A", 4),
            (
                r#"AAAA
BBCD
BBCC
EEEC"#,
                80,
            ),
            (
                r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#,
                436,
            ),
            (
                r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#,
                236,
            ),
            (
                r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#,
                368,
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
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
