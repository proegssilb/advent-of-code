use aoc_zen_runner_macros::{aoc, generator, solution, solver};
use std::io::Write;

#[aoc(2023, day25)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Vec<i32> {
        Vec::new()
    }

    // ----------------------- Part 1 -----------------------

    #[solver(part1, draft_solvr)]
    pub fn solve_part1(input: Vec<i32>) -> u32 {
        0
    }

    #[solution(part1, draft_soln)]
    pub fn part1_draft(input: &str) -> u32 {
        let mut ofile = std::fs::File::create("25_out.txt").unwrap();

        writeln!(&ofile, "graph {{").unwrap();
        let h = input.lines().map(|ln| ln.split_once(":").unwrap());
        for (k, v) in h {
            let v = v.split_ascii_whitespace();
            for elem in v {
                writeln!(&ofile, "{} -- {}", &k, elem).unwrap()
            }
        }
        writeln!(&ofile, "}}").unwrap();

        0
    }

    // ----------------------- Part 2 -----------------------

    #[solver(part2, draft_solvr)]
    pub fn solve_part2(input: Vec<i32>) -> u32 {
        0
    }

    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use aoc_zen_runner_macros::aoc_case;
    use super::solutions::*;

    #[aoc_case(0, 0)]
    const input1: &str = "";
}
