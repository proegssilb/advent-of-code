use aoc_zen_runner_macros::{aoc, generator, solution, solver};
use bstr::ByteSlice;
use std::cmp::min;
use std::{cmp::Reverse, collections::BinaryHeap, ops::Neg};
use itertools::Itertools;

#[aoc(2023, day17)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Vec<&[u8]> {
        let mut parsed_grid = Vec::new() ;
        for ln in input.as_bytes().lines() {
            parsed_grid.push(ln)
        }

        parsed_grid
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    pub enum Direction {
        North,
        South,
        East,
        West,
    }

    impl Neg for Direction {
        type Output = Direction;

        fn neg(self) -> Self::Output {
            match self {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            }
        }
    }

    // ----------------------- Part 1 -----------------------

    pub type AnswerType = u16;

    fn to_idx((row, column): (usize, usize)) -> u16 {
        (((row as u16) & 0xFF) << 8) | ((column as u16) & 0xFF)
    }

    fn neighbors(
        grid: &Vec<&[u8]>,
        coords: &(usize, usize),
        exclude: Option<Direction>,
    ) -> Vec<(usize, usize, Direction)> {
        let (src_r, src_c) = coords;

        let mut rv = Vec::with_capacity(4);

        if src_r > &0 && exclude != Some(Direction::North) {
            rv.push((src_r - 1, *src_c, Direction::North));
        }

        if src_r < &&(grid.len() - 1) && exclude != Some(Direction::South) {
            rv.push((src_r + 1, *src_c, Direction::South));
        }

        if src_c < &(grid[0].len() - 1) && exclude != Some(Direction::East) {
            rv.push((*src_r, src_c + 1, Direction::East));
        }

        if src_c > &0 && exclude != Some(Direction::West) {
            rv.push((*src_r, src_c - 1, Direction::West));
        }

        rv
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct VisitRecordPart1 {
        cost: AnswerType,
        history: Vec<(usize, usize)>,
        last_dir: Direction,
        dir_count: u8,
        coords: (usize, usize),
    }

    #[solver(part1, draft_solvr)]
    pub fn solve_part1(input: Vec<&[u8]>) -> AnswerType {
        // All coordinates are in zero-indexed (row, column) format.
        let src_coords = (0usize, 0usize);
        let targ_coords = (input.len(), input[0].len());
        let mut work_queue: BinaryHeap<Reverse<VisitRecordPart1>> = BinaryHeap::new();

        let mut result: AnswerType = AnswerType::MAX;

        for n in neighbors(&input, &src_coords, None) {
            let (targ_r, targ_c, dir) = n;
            println!("Initialization direction: {:?}", &dir);
            let visit = VisitRecordPart1 {
                coords: (targ_r, targ_c),
                cost: input[targ_r][targ_c] as AnswerType,
                history: vec![src_coords, (targ_r, targ_c)],
                last_dir: dir,
                dir_count: 1,
            };
            work_queue.push(Reverse(visit))
        }

        while work_queue.len() > 0 {
            let Reverse(visit) = work_queue
                .pop()
                .expect("Work queue has items, but pop() returned None.");

            let current_cost = visit.cost + input[visit.coords.0][visit.coords.1] as u16;

            if visit.coords == targ_coords {
                // We should have found the shortest path!
                let current_cost = current_cost - (b'0' as u16) * (visit.history.len() + 1) as u16;
                result = min(result, current_cost);
                continue;
            }

            // We need to find the next nodes to visit.
            let mut next_history = visit.history.clone();
            next_history.push(visit.coords);

            // println!("Checking node with cost {}: {}", &current_cost, &next_history.iter().map(|t| format!("({}, {})", t.0, t.1)).join(" -> "));

            let neighbors = match visit.dir_count {
                3 => neighbors(&input, &visit.coords, Some(visit.last_dir)),
                _ => neighbors(&input, &visit.coords, None),
            };

            for (n_r, n_c, n_dir) in neighbors {
                // Don't loop
                if next_history.contains(&(n_r, n_c)) {
                    continue;
                }
                let n_count = if &n_dir == &visit.last_dir {
                    visit.dir_count + 1
                } else {
                    0
                };
                let n_visit = VisitRecordPart1 {
                    coords: (n_r, n_c),
                    cost: current_cost,
                    history: next_history.clone(),
                    last_dir: n_dir,
                    dir_count: n_count,
                };

                work_queue.push(Reverse(n_visit));
            }
        }

        result
    }

    #[solution(part1, draft_soln)]
    pub fn part1_draft(input: &str) -> AnswerType {
        0
    }

    // ----------------------- Part 2 -----------------------

    #[solver(part2, draft_solvr)]
    pub fn solve_part2(input: Vec<&[u8]>) -> u32 {
        0
    }

    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::solutions::*;
    use super::*;
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(102)]
    const INPUT1: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
}
