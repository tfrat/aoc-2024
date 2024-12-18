use crate::days::Day;
use std::cmp::min;

#[derive(Default)]
pub struct DayTwo {}

impl DayTwo {
    fn parse_reports(input: &str) -> Vec<Vec<i32>> {
        let mut reports: Vec<Vec<i32>> = Vec::new();
        for line in input.lines() {
            let report: Vec<i32> = line
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

    fn check_report(&self, report: &[i32]) -> bool {
        let out_of_bounds = report
            .windows(2)
            .filter(|window| (window[1] - window[0]).abs() < 1 || (window[1] - window[0]).abs() > 3)
            .count();
        let sorted: &mut Vec<i32> = &mut report.to_vec();
        sorted.sort();
        let reverse_sorted: &mut Vec<i32> = &mut report.to_vec();
        reverse_sorted.sort_by_key(|&x| std::cmp::Reverse(x));

        let count = report.iter().zip(sorted).filter(|(x, y)| x != y).count();
        let reverse_count = report
            .iter()
            .zip(reverse_sorted)
            .filter(|(x, y)| x != y)
            .count();

        out_of_bounds + min(count, reverse_count) == 0
    }

    fn is_report_safe(&self, report: &[i32], enable_dampener: &bool) -> bool {
        if self.check_report(report) {
            return true;
        }
        if *enable_dampener {
            for i in 0..report.len() {
                let vec: Vec<i32> = report[0..i]
                    .iter()
                    .chain(&report[i + 1..])
                    .copied()
                    .collect();
                if self.check_report(&vec) {
                    return true;
                }
            }
        }

        false
    }
}

impl Day for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let reports = DayTwo::parse_reports(input);
        let count = reports
            .iter()
            .map(|report| self.is_report_safe(report, &false))
            .filter(|is_safe| *is_safe)
            .count();

        count.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let reports = DayTwo::parse_reports(input);
        let count = reports
            .iter()
            .map(|report| self.is_report_safe(report, &true))
            .filter(|is_safe| *is_safe)
            .count();
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_one() {
        let day = DayTwo::default();

        let cases = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], false),
            (vec![8, 6, 4, 4, 1], false),
            (vec![1, 3, 6, 7, 9], true),
        ];
        for (report, expected) in cases {
            assert_eq!(day.is_report_safe(&report, &false), expected);
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
            (vec![1, 4, 3, 4, 5], true),
            (vec![2, 2, 4, 5, 7, 9], true),
            (vec![43, 40, 41, 44, 45, 46, 48, 51], true),
        ];
        for (report, expected) in cases {
            let readable: String = report
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            assert_eq!(day.is_report_safe(&report, &true), expected, "{}", readable);
        }
    }
}
