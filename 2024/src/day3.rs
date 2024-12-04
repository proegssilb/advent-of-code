use std::str::FromStr;

use aoc_zen_runner_macros::{aoc, solution, solver};
use regex::Regex;

#[aoc(2024, day3)]
pub mod solutions {
    use super::*;

    // #[generator(gen)]
    // pub fn input_generator(input: &str) -> Vec<i32> {
    //     Vec::new()
    // }

    // ----------------------- Part 1 -----------------------

    // #[solver(part1, draft_solvr)]
    // pub fn solve_part1(input: Vec<i32>) -> u32 {
    //     0
    // }

    #[solution(part1, draft_soln)]
    pub fn part1_draft(input: &str) -> i32 {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut tally = 0;

        for c in re.captures_iter(input) 
        {
            let (_, [a_str, b_str]) = c.extract();
            let a = i32::from_str(a_str).unwrap();
            let b = i32::from_str(b_str).unwrap();
            tally += a * b;
        }

        tally
    }

    // ----------------------- Part 2 -----------------------

    // #[solver(part2, draft_solvr)]
    // pub fn solve_part2(input: Vec<i32>) -> u32 {
    //     0
    // }

    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> i32 {
        let re = Regex::new(r"(mul|don't|do)\((?:(\d+),(\d+)|()())\)").unwrap();

        let mut tally = 0;
        let mut enable = true;

        for c in re.captures_iter(input) 
        {
            let (_, [cmd, a_str, b_str]) = c.extract();
            match cmd {
                "do" => {
                    enable = true;
                }
                "don't" => {
                    enable = false;
                }
                "mul" => {
                    if enable {
                        let a = i32::from_str(a_str).unwrap();
                        let b = i32::from_str(b_str).unwrap();
                        tally += a * b;
                    }
                }
                _ => {}
            }
        }

        tally
    }
}

#[cfg(test)]
mod tests {
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(161, 161)]
    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[aoc_case(161, 48)]
    const INPUT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
}
