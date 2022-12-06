use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type DataLine = (u16, u16, u16, u16);

#[aoc_generator(day4, numparse)]
pub fn input_generator(input: &str) -> Vec<DataLine> {
    let mut results: Vec<DataLine> = Vec::new();
    for line in input.lines() {
        let (s1, rest) = line.split_once('-').unwrap();
        let (e1, rest) = rest.split_once(',').unwrap();
        let (s2, e2) = rest.split_once('-').unwrap();
        results.push((
            s1.parse().unwrap(),
            e1.parse().unwrap(),
            s2.parse().unwrap(),
            e2.parse().unwrap(),
        ))
    }
    results
}

fn range_either_contains(dl: &DataLine) -> i32 {
    let (a_s, a_e, b_s, b_e) = dl;
    let cmp1 = a_s.cmp(b_s);
    let cmp2 = a_e.cmp(b_e);
    let res = cmp1 != cmp2 || cmp1 == Ordering::Equal;
    res as i32
}

fn range_overlaps(dl: &DataLine) -> i32 {
    let (a_s, a_e, b_s, b_e) = dl;
    let res = !(a_e < b_s || b_e < a_s);
    res as i32
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[DataLine]) -> i32 {
    input.iter().map(range_either_contains).sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[DataLine]) -> i32 {
    input.iter().map(range_overlaps).sum()
}
