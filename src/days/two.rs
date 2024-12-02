use std::fmt::Display;
use crate::days::Day;

#[derive(Default)]
pub struct DayTwo {

}

impl DayTwo {
    fn parse_reports(input: &str) -> Vec<Vec<u32>> {
        let mut reports: Vec<Vec<u32>> = Vec::new();
        for line in input.lines() {
            let report: Vec<u32> = line
                .split_whitespace()
                .map(|num| num.parse().expect(format!("Found a non-number {}", num).as_str()))
                .collect();
            reports.push(report)
        }
        reports
    }


    fn is_report_safe(report: &Vec<u32>, dampener: &u32) -> bool {
        if report.len() < 2 {
            return true
        }
        let mut bad_levels: u32 = 0;
        let mut index: usize = 0;
        let mut next_index: usize = index + 1;
        let mut direction = 0;
        while next_index < report.len() {
            let first = report[index] as i32;
            let next = report[next_index] as i32;
            let diff: i32 = next - first;
            if direction == 0 {
                direction = diff;
            }
            if (diff.abs() < 1 || diff.abs() > 3) ||
                (diff < 0 && direction > 0) ||
                (diff > 0 && direction < 0) {
                bad_levels += 1;
                if bad_levels > *dampener {
                    return false
                }
            } else {
                index = next_index;
            }
            next_index += 1;
        }
        true
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
        ];
        for (report, expected) in cases {
            let readable: String = report
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            assert_eq!(
                DayTwo::is_report_safe(&report, &1),
                expected, "{}", readable
            );
        }
    }
}
