use aoc_zen_runner_macros::{aoc, solution};
use std::collections::HashSet;

#[aoc(2023, day4)]
pub mod solutions {
    use std::cmp::min;

    use super::*;

    // ----------------------- Part 1 -----------------------
    #[solution(part1, draft_soln)]
    pub fn part1_draft(input: &str) -> u32 {
        let mut tally = 0;
        for line in input.lines() {
            let (_, ln) = line.split_once(':').expect("No colon found.");
            let (a, b) = ln.split_once('|').expect("line did not contain a pipe.");
            let a_set = a
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|s| s.parse::<u8>().expect(&format!("Could not parse: '{}'", s)))
                .collect::<HashSet<u8>>();
            let b_set = b
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<HashSet<u8>>();
            let w_set_count = a_set.intersection(&b_set).count() as u32;
            if w_set_count == 0 {
                continue;
            }
            let w_points = 2i32.pow(w_set_count - 1);

            tally += w_points as u32;
        }

        tally
    }

    #[solution(part1, bitmangle)]
    pub fn part1_bitmangle(input: &str) -> u32 {
        let mut tally = 0;
        for line in input.lines() {
            let (_, ln) = line.split_once(':').expect("No colon found.");
            let (a, b) = ln.split_once('|').expect("line did not contain a pipe.");
            let a_set = a
                .split_whitespace()
                .filter(|s| s.len() > 0)
                .map(|s| match s.as_bytes() {
                    [a, b] => { (a & 0x0f) * 10 + (b & 0x0f) }
                    [ a ] => { a & 0x0f }
                    _ => { unreachable!() }
                })
                .fold(0u128, |bitset, val| bitset | (1 << val));
            let b_set = b
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|s| match s.as_bytes() {
                    [a, b] => { (a & 0x0f) * 10 + (b & 0x0f) }
                    [ a ] => { a & 0x0f }
                    _ => { unreachable!() }
                })
                .fold(0u128, |bitset, val| bitset | (1 << val));
            let w_set_count = (a_set & b_set).count_ones() as u32;
            if w_set_count == 0 {
                continue;
            }
            let w_points = 2i32.pow(w_set_count - 1);

            tally += w_points as u32;
        }

        tally
    }

    /*
        TODO:
            - Swap Hashset for a u128
                - Each "number" takes up 2 characters. Use the lower 4 bits of each number (multiply by 10, don't bitshift).
                - With each number now a single byte, cast to usize and index into bitset.
            - SIMD ????
     */

    // ----------------------- Part 2 -----------------------

    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> u32 {
        let mut tally = 0;
        let lines: Vec<&str> = input.lines().collect();
        let mut card_count = vec![1; lines.len()];
        

        for (idx, line) in lines.iter().enumerate() {
            let (_, ln) = line.split_once(':').expect("No colon found.");
            let (a, b) = ln.split_once('|').expect("line did not contain a pipe.");
            let a_set = a
                .split_whitespace()
                .filter(|s| s.len() > 0)
                .map(|s| s.parse::<u8>().expect(&format!("Could not parse: '{}'", s)))
                .collect::<HashSet<u8>>();
            let b_set = b
                .split_whitespace()
                .filter(|s| s.len() > 0)
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<HashSet<u8>>();
            let w_set_count = a_set.intersection(&b_set).count();

            if w_set_count > 0 {
                let stop = min(idx + w_set_count, card_count.len() - 1);

                for _ in 0..card_count[idx] {
                    for i in idx + 1..=stop {
                        card_count[i] += 1;
                    }
                }
            }

            tally += card_count[idx];
        }

        tally
    }

    #[solution(part2, bitmangle)]
    pub fn part2_bitmangle(input: &str) -> u32 {
        let mut tally = 0;
        let lines: Vec<&str> = input.lines().collect();
        let mut card_count = [1; 300];
        let skip = lines[0].find(':').expect("No colon found.") + 2;

        for (idx, line) in lines.iter().enumerate() {
            let (a, b) = line[skip..].split_once('|').expect("line did not contain a pipe.");
            let a_set = a
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|s| match s.as_bytes() {
                    [a, b] => { (a & 0x0f) * 10 + (b & 0x0f) }
                    [ a ] => { a & 0x0f }
                    _ => { unreachable!() }
                })
                .fold(0u128, |bitset, val| bitset | (1 << val));
            let b_set = b
                .split(' ')
                .filter(|s| s.len() > 0)
                .map(|s| match s.as_bytes() {
                    [a, b] => { (a & 0x0f) * 10 + (b & 0x0f) }
                    [ a ] => { a & 0x0f }
                    _ => { unreachable!() }
                })
                .fold(0u128, |bitset, val| bitset | (1 << val));
            let w_set_count = (a_set & b_set).count_ones() as usize;

            if w_set_count > 0 {
                let stop = min(idx + w_set_count, lines.len() - 1);

                for _ in 0..card_count[idx] {
                    for i in idx + 1..=stop {
                        card_count[i] += 1;
                    }
                }
            }

            tally += card_count[idx];
        }

        tally
    }
}

#[cfg(test)]
mod tests {
    use super::solutions::*;
    use super::*;
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(13, 30)]
    const input1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
}
