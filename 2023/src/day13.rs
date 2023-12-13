use aoc_zen_runner_macros::{aoc, generator, solution, solver};
use grid::Grid;
use itertools::Itertools;

#[aoc(2023, day13)]
pub mod solutions {
    use super::*;

    pub fn parse_grid(grid_str: &str) -> Vec<&[u8]> {
        let mut lns = grid_str.lines().peekable();
        let cols = lns.peek().unwrap().as_bytes().len();
        let mut rv: Vec<&[u8]> = vec![];
        for row in lns {
            rv.push(row.as_bytes().into());
        }

        rv
    }

    pub fn brute_find_h_mirror(grid: &[&[u8]]) -> Option<usize> {
        for r in 1..(grid.len()) {
            let (top, bottom) = grid.split_at(r);
            if top.iter().rev().zip(bottom.iter()).all(|(a, b)| a == b) {
                // Found a mirror
                return Some(r);
            }
        }

        None
    }

    // https://stackoverflow.com/a/64499219/1819694
    pub fn transpose<T>(v: &[&[T]]) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        assert!(!v.is_empty());
        let len = v[0].len();
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| *n.next().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }

    pub fn brute_find_v_mirror(grid: &[&[u8]]) -> Option<usize> {
        let grid = transpose(grid);
        let grid2 = grid.iter().map(|v| v.as_slice()).collect_vec();
        brute_find_h_mirror(&grid2)
    }

    // ----------------------- Part 1 -----------------------

    #[solution(part1, draft_soln)]
    pub fn part1_draft(input: &str) -> usize {
        let grids = input.split("\n\n").map(|s| parse_grid(s));

        let mut h_tally = 0;
        let mut v_tally = 0;

        for grid in grids {
            if let Some(h_incr) = brute_find_h_mirror(&grid) {
                h_tally += h_incr;
            }
            if let Some(v_incr) = brute_find_v_mirror(&grid) {
                v_tally += v_incr;
            }
        }

        h_tally * 100 + v_tally
    }

    // ----------------------- Part 2 -----------------------
    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> usize {
        let grids = input.split("\n\n").map(|s| parse_grid(s));

        let mut h_tally = 0;
        let mut v_tally = 0;

        for grid in grids {
            let mut orig_mirror = (0, 0);
            if let Some(h_incr) = brute_find_h_mirror(&grid) {
                orig_mirror.0 = h_incr;
            }

            if let Some(v_incr) = brute_find_v_mirror(&grid) {
                orig_mirror.1 = v_incr;
            }

            let new_mirror = find_smudge(&grid, orig_mirror);

            if new_mirror != (0, 0) {
                if new_mirror.0 != orig_mirror.0 {
                    h_tally += new_mirror.0;
                } else if new_mirror.1 != orig_mirror.1 {
                    v_tally += new_mirror.1;
                }
            }

        }

        h_tally * 100 + v_tally
    }

    pub fn find_smudge(grid: &[&[u8]], orig_mirror: (usize, usize)) -> (usize, usize) {
        let mut mut_grid: Vec<Vec<u8>> = vec![];
        for s in grid.iter() {
            let v = Vec::from(*s);
            mut_grid.push(v);
        }
        
        // Find the smudge, brute-force.
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                flip_cell(&mut mut_grid, r, c);

                let mut new_mirror = (0, 0);

                if let Some(h_incr) = brute_find_h_mirror2(&mut_grid) {
                    new_mirror.0 = h_incr;
                }

                if let Some(v_incr) = brute_find_v_mirror2(&mut_grid) {
                    new_mirror.1 = v_incr;
                }

                flip_cell(&mut mut_grid, r, c);

                assert_eq!(&grid, &mut_grid);

                if new_mirror != (0, 0) {
                    if new_mirror != orig_mirror {
                        return new_mirror;
                    }
                }
            }
        }

        (0, 0)
    }

    pub fn flip_cell(grid: &mut Vec<Vec<u8>>, r: usize, c: usize) {
        if grid[r][c] == b'.' {
            grid[r][c] = b'#';
        } else {
            grid[r][c] = b'.';
        }
    }

    pub fn brute_find_h_mirror2(grid: &Vec<Vec<u8>>) -> Option<usize> {
        for r in 1..(grid.len()) {
            let (top, bottom) = grid.split_at(r);
            if top.iter().rev().zip(bottom.iter()).all(|(a, b)| a == b) {
                // Found a mirror
                return Some(r);
            }
        }

        None
    }

    pub fn brute_find_v_mirror2(grid: &Vec<Vec<u8>>) -> Option<usize> {
        let grid = transpose2(grid);
        let grid2 = grid.iter().map(|v| v.as_slice()).collect_vec();
        brute_find_h_mirror(&grid2)
    }

    // https://stackoverflow.com/a/64499219/1819694
    pub fn transpose2<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        assert!(!v.is_empty());
        let len = v[0].len();
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| *n.next().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::solutions::*;
    use super::*;
    use aoc_zen_runner_macros::aoc_case;
    use test_case::case;

    #[aoc_case(405, 400)]
    const input1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[aoc_case(300, 6)]
    const INPUT_2: &str = "#..#.##.#..
###.####.##
###.####.##
###.####.##
###.####.##
#..#.##.#..
.#..#..#..#
..#.####.#.
####....###
..#......#.
.#.........
#...#..#...
####....###
.##......##
#...####...
###.#..#.##
#.########.";

    #[aoc_case(1, 5)]
    const INPUT_3: &str =
"..#....#.
..######.
..###.##.
####..###
...####..
###.##.##
##.####.#
...#..#..
..######.";

    #[test]
    pub fn test_transpose() {
        let inp = [&[1, 2, 3][..], &[4, 5, 6][..]];
        let outp = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(outp, transpose(&inp[..]));
    }

    #[test]
    pub fn test_find_h_mirror() {
        let inp = parse_grid(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(Some(4), brute_find_h_mirror(&inp));

        let inp2 = parse_grid(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );

        assert_eq!(None, brute_find_h_mirror(&inp2));
    }

    #[test]
    pub fn test_find_v_mirror() {
        let inp = parse_grid(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(None, brute_find_v_mirror(&inp));

        let inp2 = parse_grid(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );

        assert_eq!(Some(5), brute_find_v_mirror(&inp2));
    }

    
}
