use crate::days::Day;
use std::fmt::Display;

pub struct DayTwo {
    pub min_step: i32,
    pub max_step: i32,
    pub dampener: u32,
}

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

    fn are_levels_safe(&self, first: &i32, second: &i32, direction: &i32) -> bool {
        let diff = second - first;
        (diff.abs() < self.min_step || diff.abs() > self.max_step)
            || (diff < 0 && *direction > 0)
            || (diff > 0 && *direction < 0)
    }

    fn is_report_safe_recurs(
        &self,
        report: &[u32],
        first_index: &usize,
        second_index: &usize,
        direction: &i32,
        bad_levels: &u32,
    ) -> bool {
        if *first_index == *second_index {
            return false;
        }
        if *bad_levels > self.dampener {
            return false;
        }
        if *first_index >= report.len() || *second_index >= report.len() {
            return true;
        }
        let level = report[*first_index] as i32;
        let next_level = report[*second_index] as i32;
        let diff = next_level - level;
        if *direction == 0 {
            return self.is_report_safe_recurs(
                report,
                first_index,
                second_index,
                &diff,
                bad_levels,
            );
        }

        if self.are_levels_safe(&level, &next_level, direction) {
            let new_bad_levels = *bad_levels + 1;
            return (*first_index > 0
                && self.is_report_safe_recurs(
                    report,
                    &(*first_index - 1),
                    second_index,
                    direction,
                    &new_bad_levels,
                ))
                || self.is_report_safe_recurs(
                    report,
                    first_index,
                    &(*second_index + 1),
                    direction,
                    &new_bad_levels,
                )
                || (*first_index == 0
                    && self.is_report_safe_recurs(
                        report,
                        &(*first_index + 1),
                        &(*second_index + 1),
                        &0,
                        &new_bad_levels,
                    ));
        }
        self.is_report_safe_recurs(
            report,
            &(*first_index + 1),
            &(*second_index + 1),
            direction,
            bad_levels,
        )
    }

    fn is_report_safe(&self, report: &[u32]) -> bool {
        self.is_report_safe_recurs(report, &0, &1, &0, &0)
    }
}

impl Default for DayTwo {
    fn default() -> Self {
        Self {
            min_step: 1,
            max_step: 3,
            dampener: 1,
        }
    }
}

impl Day for DayTwo {
    fn part_one(&self, input: &str) -> Box<dyn Display> {
        let reports = DayTwo::parse_reports(input);
        let count = reports
            .iter()
            .map(|report| self.is_report_safe(report))
            .filter(|is_safe| *is_safe)
            .count();

        Box::new(count)
    }

    fn part_two(&self, input: &str) -> Box<dyn Display> {
        let reports = DayTwo::parse_reports(input);
        let count = reports
            .iter()
            .map(|report| self.is_report_safe(report))
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
        let day = DayTwo {
            dampener: 0,
            ..Default::default()
        };

        let cases = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], false),
            (vec![8, 6, 4, 4, 1], false),
            (vec![1, 3, 6, 7, 9], true),
        ];
        for (report, expected) in cases {
            assert_eq!(day.is_report_safe(&report), expected);
        }
    }

    #[test]
    fn test_example_part_two() {
        let day = DayTwo::default();
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
            assert_eq!(day.is_report_safe(&report), expected, "{}", readable);
        }
    }
}
