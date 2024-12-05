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

    fn add_page(
        page: u32,
        page_order: &HashMap<u32, HashSet<u32>>,
        pages: &HashSet<u32>,
        visited_pages: &mut HashSet<u32>,
        out_of_order: &mut HashSet<u32>,
        new_order: &mut Vec<u32>,
    ) -> bool {
        let mut is_good = true;
        let priors = page_order.get(&page).cloned().unwrap_or(HashSet::new());
        let included_priors: HashSet<u32> = priors.intersection(pages).cloned().collect();
        if included_priors.intersection(visited_pages).count() == included_priors.len() {
            visited_pages.insert(page);
            if !new_order.contains(&page) {
                new_order.push(page)
            }
            out_of_order.remove(&page);
            for p in out_of_order.clone().iter() {
                Self::add_page(
                    *p,
                    page_order,
                    pages,
                    visited_pages,
                    out_of_order,
                    new_order,
                );
            }
        } else {
            is_good = false;
            out_of_order.insert(page);
        }
        is_good
    }

    fn sum_middle_pages(page_order: HashMap<u32, HashSet<u32>>, updates: &str) -> (u32, u32) {
        let mut correct_sum = 0;
        let mut incorrect_correct_sum = 0;
        for line in updates.lines() {
            let pages: Vec<u32> = line
                .split(",")
                .map(|page| page.parse::<u32>().unwrap())
                .collect();
            let pages_set: HashSet<u32> = pages.clone().into_iter().collect();
            let mut visited_pages: HashSet<u32> = HashSet::new();
            let mut is_good: bool = true;
            let mut out_of_order = HashSet::new();
            let mut new_order: Vec<u32> = Vec::new();

            for page in &pages {
                if !Self::add_page(
                    *page,
                    &page_order,
                    &pages_set,
                    &mut visited_pages,
                    &mut out_of_order,
                    &mut new_order,
                ) {
                    is_good = false;
                }
            }
            if is_good {
                correct_sum += new_order[new_order.len() / 2]
            } else {
                incorrect_correct_sum += new_order[new_order.len() / 2]
            }
        }
        (correct_sum, incorrect_correct_sum)
    }
}

impl Day for DayFive {
    fn part_one(&self, input: &str) -> String {
        let (instructions, pages) = input.split_once("\n\n").unwrap();
        let page_order = DayFive::get_page_order(instructions);
        DayFive::sum_middle_pages(page_order, pages).0.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (instructions, pages) = input.split_once("\n\n").unwrap();
        let page_order = DayFive::get_page_order(instructions);
        DayFive::sum_middle_pages(page_order, pages).1.to_string()
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
            123,
        )];
        for (input, expected) in cases {
            assert_eq!(day.part_two(input), expected.to_string())
        }
    }
}
