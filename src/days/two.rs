use crate::days::Day;
use std::fmt::Display;

#[derive(Default)]
pub struct DayTwo {}

impl DayTwo {
    fn parse_reports(input: &str) -> Vec<Vec<u32>> {
        let mut reports: Vec<Vec<u32>> = Vec::new();
        for line in input.lines() {
            let report: Vec<u32> = line
                .split_whitespace()
                .map(|num| {
                    num.parse()
                        .unwrap_or_else(|_| panic!("Found a non-number {}", num))
                })
                .collect();
            reports.push(report)
        }
        reports
    }

    fn are_levels_safe(first: &i32, second: &i32, direction: &i32) -> bool {
        let diff = second - first;
        (diff.abs() < 1 || diff.abs() > 3)
            || (diff < 0 && *direction > 0)
            || (diff > 0 && *direction < 0)
    }

    fn is_report_safe_recurs(
        report: &[u32],
        dampener: &u32,
        first_index: &usize,
        second_index: &usize,
        direction: &i32,
        bad_levels: &u32,
    ) -> bool {
        if *first_index == *second_index {
            return false;
        }
        if *bad_levels > *dampener {
            return false;
        }
        if *first_index >= report.len() || *second_index >= report.len() {
            return true;
        }
        let level = report[*first_index] as i32;
        let next_level = report[*second_index] as i32;
        let diff = next_level - level;
        if *direction == 0 {
            return DayTwo::is_report_safe_recurs(
                report,
                dampener,
                first_index,
                second_index,
                &diff,
                bad_levels,
            );
        }

        if DayTwo::are_levels_safe(&level, &next_level, direction) {
            let new_bad_levels = *bad_levels + 1;
            return (*first_index > 0
                && DayTwo::is_report_safe_recurs(
                    report,
                    dampener,
                    &(*first_index - 1),
                    second_index,
                    direction,
                    &new_bad_levels,
                ))
                || DayTwo::is_report_safe_recurs(
                    report,
                    dampener,
                    first_index,
                    &(*second_index + 1),
                    direction,
                    &new_bad_levels,
                )
                || (*first_index == 0
                    && DayTwo::is_report_safe_recurs(
                        report,
                        dampener,
                        &(*first_index + 1),
                        &(*second_index + 1),
                        &0,
                        &new_bad_levels,
                    ));
        }
        DayTwo::is_report_safe_recurs(
            report,
            dampener,
            &(*first_index + 1),
            &(*second_index + 1),
            direction,
            bad_levels,
        )
    }

    fn is_report_safe(report: &[u32], dampener: &u32) -> bool {
        DayTwo::is_report_safe_recurs(report, dampener, &0, &1, &0, &0)
    }
}

impl Day for DayTwo {
    fn part_one(&self, input: &str) -> Box<dyn Display> {
        let reports = DayTwo::parse_reports(input);
        let count = reports
            .iter()
            .map(|report| DayTwo::is_report_safe(report, &0))
            .filter(|is_safe| *is_safe)
            .count();

        Box::new(count)
    }

    fn part_two(&self, input: &str) -> Box<dyn Display> {
        let reports = DayTwo::parse_reports(input);
        let count = reports
            .iter()
            .map(|report| DayTwo::is_report_safe(report, &1))
            .filter(|is_safe| *is_safe)
            .count();
        Box::new(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_one() {
        let cases = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], false),
            (vec![8, 6, 4, 4, 1], false),
            (vec![1, 3, 6, 7, 9], true),
        ];
        for (report, expected) in cases {
            assert_eq!(DayTwo::is_report_safe(&report, &0), expected);
        }
    }

    #[test]
    fn test_example_part_two() {
        let cases = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], true),
            (vec![8, 6, 4, 4, 1], true),
            (vec![1, 3, 6, 7, 9], true),
            (vec![7, 2, 4, 7, 9], true),
        ];
        for (report, expected) in cases {
            let readable: String = report
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            assert_eq!(
                DayTwo::is_report_safe(&report, &1),
                expected,
                "{}",
                readable
            );
        }
    }
}
