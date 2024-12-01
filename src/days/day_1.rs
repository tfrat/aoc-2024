use std::collections::HashMap;

fn number_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut left: Vec<i32> = Vec::with_capacity(lines.len());
    let mut right: Vec<i32> = Vec::with_capacity(lines.len());
    for line in lines {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        left.push(numbers[0].parse().expect(""));
        right.push(numbers[1].parse().expect(""));
    }
    (left, right)
}

pub fn part_one(input: &str) -> i32 {
    let (mut left, mut right) = number_lists(input);
    left.sort();
    right.sort();
    let mut sum = 0;
    for (left_item, right_item) in left.into_iter().zip(right.into_iter()) {
        sum += (left_item - right_item).abs()
    }
    sum
}

pub fn part_two(input: &str) -> i32 {
    let (left, right) = number_lists(input);
    let mut similarity_score = 0;
    let mut right_count: HashMap<i32, i32> = HashMap::new();

    for right_item in right {
        *right_count.entry(right_item).or_insert(0) += 1;
    }
    for left_item in left {
        let count = *right_count.get(&left_item).get_or_insert(&0);
        similarity_score += left_item * count;
    }
    similarity_score
}