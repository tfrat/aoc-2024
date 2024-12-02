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


    fn is_report_safe(report: &Vec<u32>) -> bool {
        let mut direction: i32 = 0;
        for index in 0..report.len() - 1 {
            let first = report[index] as i32;
            let next = report[index + 1] as i32;
            let diff: i32 = next - first;

            if diff.abs() < 1 || diff.abs() > 3 {
                return false
            }
            if direction == 0 {
                direction = diff
            } else {
                if diff < 0 && direction > 0 {
                    return false
                }
                if diff > 0 && direction < 0 {
                    return false
                }
            }
        }
        true
    }
}

impl Day for DayTwo {
    fn part_one(&self, input: &str) -> Box<dyn Display> {
        let reports = DayTwo::parse_reports(input);
        let count = reports
            .iter()
            .map(|report| DayTwo::is_report_safe(report))
            .filter(|is_safe| *is_safe)
            .count();

        Box::new(count)
    }

    fn part_two(&self, _input: &str) -> Box<dyn Display> {
        Box::new("TODO")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_levels() {
        assert_eq!(DayTwo::is_report_safe(&vec![7, 6, 4, 2, 1]), true); // Check if the result is correct
    }
}
