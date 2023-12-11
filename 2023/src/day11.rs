use aoc_zen_runner_macros::{aoc, generator, solution, solver};
use grid::Grid;
use itertools::Itertools;

pub struct Input {
    map: Grid<char>,
    galaxies: Vec<(usize, usize)>,
}

#[aoc(2023, day11)]
pub mod solutions {
    use std::cmp::{min, max};

    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let mut lns = input.lines().map(|s| s.chars().collect_vec()).peekable();
        let width = lns.peek().unwrap().len();

        let mut rv_grid = Grid::from_vec(lns.flat_map(|i| i).collect(), width);

        // Double the rows

        let mut i = 0;

        while i < rv_grid.rows() {
            let row = rv_grid.iter_row(i).map(|&i| i).collect_vec();
            if row.contains(&'#') {
                i += 1;
            } else {
                rv_grid.insert_row(i, row);
                i += 2;
            }
        }

        // Double the columns
        let mut i = 0;

        while i < rv_grid.cols() {
            let col = rv_grid.iter_col(i).map(|&i| i).collect_vec();
            if col.contains(&'#') {
                i += 1;
            } else {
                rv_grid.insert_col(i, col);
                i += 2;
            }
        }

        let mut rv_galaxies = vec![];
        for (idx, cell) in rv_grid.indexed_iter() {
            if cell == &'#' {
                rv_galaxies.push(idx);
            }
        }

        Input { map: rv_grid, galaxies: rv_galaxies}
    }

    // ----------------------- Part 1 -----------------------

    #[solver(part1, draft_solvr)]
    pub fn solve_part1(input: Input) -> usize {
        let mut tally = 0;
        for (idx1, loc1) in input.galaxies.iter().enumerate() {
            for loc2 in input.galaxies[idx1..].iter() {
                let distance = loc1.0.abs_diff(loc2.0) + loc1.1.abs_diff(loc2.1);
                // dbg!(idx1, idx2, distance);
                tally += distance;
            }
        }

        tally
    }

    // ----------------------- Part 2 -----------------------

    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> usize {
        let mut lns = input.lines().map(|s| s.chars().collect_vec()).peekable();
        let width = lns.peek().unwrap().len();

        let rv_grid = Grid::from_vec(lns.flat_map(|i| i).collect(), width);

        let mut expanded_rows = vec![];
        let mut expanded_cols = vec![];

        // Find the empty rows

        for (idx, mut r) in rv_grid.iter_rows().enumerate() {
            if !r.contains(&'#') {
                expanded_rows.push(idx)
            }
        }

        // dbg!(expanded_rows.len());

        // Find the empty columns
        for (idx, mut c) in rv_grid.iter_cols().enumerate() {
            if !c.contains(&'#') {
                expanded_cols.push(idx)
            }
        }

        // dbg!(expanded_cols.len());

        let mut rv_galaxies = vec![];
        for (idx, cell) in rv_grid.indexed_iter() {
            if cell == &'#' {
                rv_galaxies.push(idx);
            }
        }

        #[cfg(test)]
        const SCALE_FACTOR: usize = 99;

        #[cfg(not(test))]
        const SCALE_FACTOR: usize = 999_999;

        let mut tally = 0;
        for (idx1, loc1) in rv_galaxies.iter().enumerate() {
            for (idx2, loc2) in rv_galaxies[idx1..].iter().enumerate() {
                let mut distance = loc1.0.abs_diff(loc2.0) + loc1.1.abs_diff(loc2.1);
                let r_sr = min(loc1.0, loc2.0);
                let r_st = max(loc1.0, loc2.0);
                let c_sr = min(loc1.1, loc2.1);
                let c_st = max(loc1.1, loc2.1);
                distance += expanded_rows.iter().filter(|ri| (r_sr..r_st).contains(ri)).count() * SCALE_FACTOR;
                distance += expanded_cols.iter().filter(|ci| (c_sr..c_st).contains(ci)).count() * SCALE_FACTOR;
                // dbg!(idx1, idx2, distance);
                tally += distance;
            }
        }

        tally
    }
}

#[cfg(test)]
mod tests {
    use aoc_zen_runner_macros::aoc_case;
    use super::*;
    use super::solutions::*;

    #[aoc_case(374, 8410)]
    const input1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
}
