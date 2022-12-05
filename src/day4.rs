use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

type DataLine = u64;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<DataLine> {
    let mut results: Vec<DataLine> = Vec::new();
    for line in input.lines() {
        let mut byte_holder = 0u16;
        let mut res = 0u64;

        for b in line.as_bytes() {
            match b {
                b'-' | b',' => {
                    res = res << 16 | (byte_holder as u64);
                    byte_holder = 0;
                }
                _ => { byte_holder = byte_holder << 8 | (*b as u16); }
            }
        }

        if byte_holder != 0 {
            res = res << 16 | (byte_holder as u64);
        }

        results.push(res);
    }
    results
}

fn range_either_contains(dl: &DataLine) -> i32 {
    let a_s = (dl >> 24 & 0xFFFF) as u16;
    let a_e = (dl >> 16 & 0xFFFF) as u16;
    let b_s = (dl >> 8 & 0xFFFF) as u16;
    let b_e = (dl & 0xFFFF) as u16;
    let cmp1 = a_s.cmp(&b_s);
    let cmp2 = a_e.cmp(&b_e);
    let res = cmp1 != cmp2 || cmp1 == Ordering::Equal;
    res as i32
}

fn range_overlaps(dl: &DataLine) -> i32 {
    let a_s = (dl >> 24 & 0xFFFF) as u16;
    let a_e = (dl >> 16 & 0xFFFF) as u16;
    let b_s = (dl >> 8 & 0xFFFF) as u16;
    let b_e = (dl & 0xFFFF) as u16;
    let res = !(a_e < b_s || b_e < a_s);
    res as i32
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[DataLine]) -> i32 {
    input
        .iter()
        .map(range_either_contains)
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[DataLine]) -> i32 {
    input
        .iter()
        .map(range_overlaps)
        .sum()
}

#[test]
fn test_input_generator_1() {
    assert_eq!(vec![u64::from_be_bytes([0, b'2', 0, b'4', 0, b'6', 0, b'8'])], input_generator("2-4,6-8"));
}

#[test]
fn test_input_generator_2() {
    assert_eq!(vec![u64::from_be_bytes([b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8'])], input_generator("12-34,56-78"));
}

#[test]
fn test_input_generator_3() {
    let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
    let output = input_generator(input);

    let expected = vec![
        0x0032003400360038,
        0x0032003300340035,
        0x0035003700370039,
        0x0032003800330037,
        0x0036003600340036,
        0x0032003600340038,
    ];

    assert_eq!(output, expected);
}

#[test]
fn test_range_either_contains_1() {
    let range = 0x0032003400360038;
    let result = range_either_contains(&range);
    assert_eq!(0, result);
}

#[test]
fn test_range_either_contains_2() {
    let range = 0x0032003800330037;
    let result = range_either_contains(&range);
    assert_eq!(1, result);
}

#[test]
fn test_range_either_contains_3() {
    let range = 0x0032003600340038;
    let result = range_either_contains(&range);
    assert_eq!(1, result);
}

#[test]
fn test_range_overlaps_1() {
    let range = 0x0032003400360038;
    let result = range_either_contains(&range);
    assert_eq!(0, result);
}

#[test]
fn test_range_overlaps_2() {
    let range = 0x0032003800330037;
    let result = range_either_contains(&range);
    assert_eq!(1, result);
}

#[test]
fn test_range_overlaps_3() {
    let range = 0x0032003600340038;
    let result = range_either_contains(&range);
    assert_eq!(1, result);
}