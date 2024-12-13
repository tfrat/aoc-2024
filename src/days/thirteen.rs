use crate::days::Day;
use crate::utils::Coord;
use itertools::Itertools;
use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Debug)]
struct Button {
    cost: u32,
    moves: Coord,
}

#[derive(Debug)]
struct PrizeMachine {
    prize_location: Coord,
    a: Button,
    b: Button,
}

#[derive(Default)]
pub struct DayThirteen {}

impl DayThirteen {
    fn parse_machines(input: &str, prize_location_offset: i64) -> Vec<PrizeMachine> {
        /*
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400
         */

        let find_numbers = Regex::new(r"(\d+).+?(\d+)").unwrap();

        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let caps = find_numbers.captures(line).unwrap();
                Coord {
                    x: caps[1].parse().unwrap(),
                    y: caps[2].parse().unwrap(),
                }
            })
            .chunks(3)
            .into_iter()
            .map(|chunk| {
                let chunks: Vec<Coord> = chunk.collect();
                PrizeMachine {
                    a: Button {
                        cost: 3,
                        moves: chunks[0],
                    },
                    b: Button {
                        cost: 1,
                        moves: chunks[1],
                    },
                    prize_location: chunks[2].plus(prize_location_offset, prize_location_offset),
                }
            })
            .collect()
    }
    fn run(
        prize_machine: &PrizeMachine,
        location: &Coord,
        a_presses: u32,
        b_presses: u32,
        press_limit: &Option<u32>,
        memo: &mut HashMap<Coord, Option<u32>>,
    ) -> Option<u32> {
        if let Some(cost) = memo.get(location) {
            return *cost;
        }

        if press_limit.is_some_and(|limit| a_presses > limit && b_presses > limit)
            || location.x > prize_machine.prize_location.x
            || location.y > prize_machine.prize_location.y
        {
            return None;
        }

        if prize_machine.prize_location == *location {
            return Some(0);
        }

        let next_a = location.plus(prize_machine.a.moves.x, prize_machine.a.moves.y);
        let a_cost = Self::run(
            prize_machine,
            &next_a,
            a_presses + 1,
            b_presses,
            press_limit,
            memo,
        );
        let next_b = location.plus(prize_machine.b.moves.x, prize_machine.b.moves.y);
        let b_cost = Self::run(
            prize_machine,
            &next_b,
            a_presses,
            b_presses + 1,
            press_limit,
            memo,
        );

        let cost = match (a_cost, b_cost) {
            (Some(a), None) => Some(a + prize_machine.a.cost),
            (None, Some(b)) => Some(b + prize_machine.b.cost),
            (Some(a), Some(b)) => Some(min(a + prize_machine.a.cost, b + prize_machine.b.cost)),
            _ => None,
        };

        memo.insert(*location, cost);
        cost
    }
    fn get_prizes(machines: &[PrizeMachine], press_limit: &Option<u32>) -> u32 {
        machines
            .iter()
            .filter_map(|machine| {
                Self::run(
                    machine,
                    &Coord::new(0, 0),
                    0,
                    0,
                    press_limit,
                    &mut HashMap::new(),
                )
            })
            .sum()
    }
}

impl Day for DayThirteen {
    fn part_one(&self, input: &str) -> String {
        let machines = DayThirteen::parse_machines(input, 0);
        DayThirteen::get_prizes(&machines, &Some(100)).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let machines = DayThirteen::parse_machines(input, 10000000000000);
        DayThirteen::get_prizes(&machines, &None).to_string()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let day = DayThirteen::default();
        let cases = vec![(
            r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#,
            480,
        )];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayThirteen::default();
        let cases = vec![(
            r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#,
            480,
        )];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
