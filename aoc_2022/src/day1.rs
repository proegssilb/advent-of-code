use std::collections::BinaryHeap;
use aoc_runner_derive::{aoc_generator, aoc};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    let mut results: Vec<i32> = Vec::new();
    let mut current_sum: i32 = 0;
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            results.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += trimmed.parse::<i32>().unwrap();
        }
    }
    results
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    *input
        .iter()
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let mut results = BinaryHeap::new();
    for backpack in input {
        results.push(backpack);
        if results.len() > 3 {
            _ = results.pop();
        }
    }

    results.into_iter().sum()
}