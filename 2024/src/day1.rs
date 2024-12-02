use std::str::FromStr;

use aoc_zen_runner_macros::{aoc, generator, solution, solver};

#[aoc(2024, day1)]
pub mod solutions {
    use std::collections::{HashMap, HashSet};

    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> (Vec<i32>, Vec<i32>) {
        let mut lst_a = vec![];
        let mut lst_b = vec![];

        for ln in input.lines()
        {
            let [a, b] = ln.split_whitespace().collect::<Vec<&str>>().try_into().unwrap();

            lst_a.push(i32::from_str(a).unwrap());
            lst_b.push(i32::from_str(b).unwrap());
        }

        (lst_a, lst_b)
    }

    #[generator(fixed_width)]
    pub fn input_generator_fixed_index(input: &str) -> (Vec<i32>, Vec<i32>) {
        let mut lst_a = vec![];
        let mut lst_b = vec![];

        for ln in input.lines()
        {
            let (a, b) = (&ln[0..5], &ln[8..13]);

            lst_a.push(i32::from_str(a).unwrap());
            lst_b.push(i32::from_str(b).unwrap());
        }

        (lst_a, lst_b)
    }

    // ----------------------- Part 1 -----------------------

    #[solver(part1, draft_solvr)]
    pub fn solve_part1(input: (Vec<i32>, Vec<i32>)) -> u32 {
        let (mut a_lst, mut b_lst) = input;
        a_lst.sort();
        b_lst.sort();

        let mut tally = 0;

        for (a, b) in a_lst.iter().zip(b_lst.iter()) {
            tally += a.abs_diff(*b);
        }

        tally
    }

    // #[solution(part1, draft_soln)]
    // pub fn part1_draft(input: &str) -> u32 {
    //     0
    // }

    // ----------------------- Part 2 -----------------------

    #[solver(part2, draft_solvr)]
    pub fn solve_part2(input: (Vec<i32>, Vec<i32>)) -> i32 {
        let (a_lst, b_lst) = input;

        let mut count_map = HashMap::new();

        for b in b_lst {
            count_map.entry(b).and_modify(|ent| *ent += 1).or_insert(1);
        }

        let mut tally = 0;

        for n in a_lst {
            tally += n * count_map.get(&n).unwrap_or(&0);
        }

        tally
    }

    // #[solution(part2, draft_soln)]
    // pub fn part2_draft(input: &str) -> u32 {
    //     0
    // }
}

#[cfg(test)]
mod tests {
    use aoc_zen_runner_macros::aoc_case;
    use super::solutions::*;

    #[aoc_case(11, 31)]
    const input1: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
}
