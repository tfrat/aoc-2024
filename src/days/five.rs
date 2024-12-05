use crate::days::Day;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct DayFive {}

impl DayFive {
    fn get_page_order(input: &str) -> HashMap<u32, HashSet<u32>> {
        let re = Regex::new(r"(\d+)\|(\d+)");
        re.unwrap()
            .captures_iter(input)
            .filter(|caps| caps.len() > 0)
            .map(|caps| {
                (
                    caps[1].parse::<u32>().unwrap(),
                    caps[2].parse::<u32>().unwrap(),
                )
            })
            .fold(HashMap::new(), |mut map, (before, after)| {
                let set = map.entry(after).or_default();
                set.insert(before);
                map
            })
    }

    fn sum_middle_pages(page_order: &mut HashMap<u32, HashSet<u32>>, updates: &str) -> u32 {
        let mut sum = 0;
        for line in updates.lines() {
            let pages: Vec<u32> = line
                .split(",")
                .map(|page| page.parse::<u32>().unwrap())
                .collect();
            let pages_set: HashSet<u32> = pages.clone().into_iter().collect();
            let mut visited_pages: HashSet<u32> = HashSet::new();
            let mut is_good: bool = true;
            for page in &pages {
                let priors = page_order.get(page).cloned().unwrap_or(HashSet::new());
                let included_priors: HashSet<u32> =
                    priors.intersection(&pages_set).cloned().collect();
                if included_priors.intersection(&visited_pages).count() == included_priors.len() {
                    visited_pages.insert(*page);
                } else {
                    is_good = false;
                    break;
                }
            }
            if is_good {
                sum += pages[pages.len() / 2]
            }
        }
        sum
    }
}

impl Day for DayFive {
    fn part_one(&self, input: &str) -> String {
        let (instructions, pages) = input.split_once("\n\n").unwrap();
        let page_order = DayFive::get_page_order(instructions);
        DayFive::sum_middle_pages(&mut page_order.clone(), pages).to_string()
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
        let day = DayFive::default();
        let cases = vec![(
            r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#,
            143,
        )];
        for (input, expected) in cases {
            assert_eq!(day.part_one(input), expected.to_string())
        }
    }

    #[test]
    fn test_part_two() {
        let day = DayFive::default();
        let cases = vec![("", 0)];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
