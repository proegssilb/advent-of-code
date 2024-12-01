use aoc_zen_runner_macros::{aoc, solution};
use bitvec::prelude::*;
use bitvec::vec::BitVec;
use bitvec::{bitvec, slice::BitSlice};
use itertools::Itertools;

#[aoc(2023, day12)]
pub mod solutions {
    use std::collections::HashMap;

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

    pub fn gen_lines(template: &str) -> impl Iterator<Item = String> {
        // TODO: Use number-range, map over each number's bits, translate 0/1 to '.'/'#'

        let len = template.len();

        if len == 1 {
            return match template {
                "." => vec![".".to_owned()],
                "#" => vec!["#".to_owned()],
                "?" => vec![".".to_owned(), "#".to_owned()],
                _ => unreachable!(),
            }
            .into_iter();
        }

        let b = &template[0..=0];
        let rst = &template[1..];
        let rv = match b {
            "." => gen_lines(rst).map(|s| ".".to_owned() + &s).collect_vec(),
            "#" => gen_lines(rst).map(|s| "#".to_owned() + &s).collect_vec(),
            "?" => gen_lines(rst)
                .cartesian_product(vec![".".to_owned(), "#".to_owned()])
                .map(|(a, b)| b + &a)
                .collect_vec(),
            _ => unreachable!(),
        };

        rv.into_iter()
    }

    pub fn line_matches_groups(ln: &str, groups: &[u32]) -> bool {
        let mut starting_index = 0;
        for n in groups {
            if let Some(find_idx) = ln[starting_index..].find('#') {
                // println!("Found match at {} (starting at {})", find_idx, starting_index);
                let group_idx = starting_index + find_idx;
                assert_eq!(&ln[group_idx..=group_idx], "#", ".find found a non-match");
                let mut group_end_idx = group_idx;
                while group_end_idx < ln.len() - 1
                    && &ln[group_end_idx + 1..group_end_idx + 2] == "#"
                {
                    group_end_idx += 1;
                }
                // println!("Found group: {}", &ln[group_idx..=group_end_idx]);
                assert!(
                    ln[group_idx..=group_end_idx].chars().all(|c| c == '#'),
                    "Found group contains mixed springs"
                );

                // println!("Comparing group: {} to {} == {}", group_idx, group_end_idx, n);

                if (group_end_idx - group_idx + 1) != *n as usize {
                    return false;
                }

                starting_index = group_end_idx + 1;
            } else {
                return false;
            }
        }

        if starting_index < ln.len() && ln[starting_index..].find("#").is_some() {
            false
        } else {
            true
        }
    }

    #[solution(part1, draft_soln)]
    pub fn part1_draft(input: &str) -> usize {
        let mut tally = 0;

        for orig_ln in input.lines() {
            let (ln, grps_txt) = orig_ln.split_once(" ").unwrap();
            let grps: Vec<u32> = grps_txt
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let incr = gen_lines(ln)
                //.inspect(|l| println!("Generated line for template {:?}: {:?} (match: {})", &ln, &l, line_matches_groups(l, &grps)))
                .filter(|ln| line_matches_groups(ln, &grps))
                .count();

            // println!("Line {} had {} combos", &ln, &incr);
            tally += incr;
        }

        tally
    }

    // ----------------------- Part 2 -----------------------

    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> usize {
        let mut tally = 0;

        for orig_ln in input.lines() {
            // println!("Attempting to solve line: {}", &orig_ln);
            let (ln, grps_txt) = orig_ln.split_once(" ").unwrap();
            let ln = format!("{0}?{0}?{0}?{0}?{0}", ln);
            let grps_txt = format!("{0},{0},{0},{0},{0}", grps_txt);
            let grps: Vec<usize> = grps_txt
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            // let prob_segments = ln.split('.').map(segment_to_bitvec).collect_vec();

            let mut line_state: HashMap<(usize, usize, usize), usize> = HashMap::new();

            let incr = dynamic_solve(&mut line_state, &ln.as_bytes(), &grps, 0, 0, 0);

            // println!("Line {} had {:?} combos", orig_ln, &incr);

            tally += incr;
        }

        tally
    }

    pub fn dynamic_solve(state: &mut HashMap<(usize, usize, usize), usize>, dots: &[u8], blocks: &[usize], i: usize, bi: usize, current: usize) -> usize {
        // Translated from https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/12.py
        let key = (i, bi, current);
        if state.contains_key(&key) {
            return state[&key];
        }

        if i == dots.len() {
            if bi == blocks.len() && current == 0 {
                return 1;
            } else if bi == blocks.len() - 1 && blocks[bi] == current {
                return 1;
            } else {
                return 0;
            }
        }

        let mut rv = 0;

        for c in [b'.', b'#'] {
            if dots[i] == c || dots[i] == b'?' {
                if c == b'.' && current == 0 {
                    rv += dynamic_solve(state, dots, blocks, i+1, bi, 0);
                } else if c == b'.' && current > 0 && bi < blocks.len() && blocks[bi] == current {
                    rv += dynamic_solve(state, dots, blocks, i+1, bi+1, 0);
                } else if c == b'#' {
                    rv += dynamic_solve(state, dots, blocks, i+1, bi, current+1);
                }
            }
        }

        state.insert(key, rv);

        rv
    }

    pub fn segment_to_bitvec(segment: &str) -> BitVec {
        // TODO: SIMD
        let mut rv = bitvec!(0; 0);

        for b in segment.as_bytes() {
            rv.push(b == &b'?');
        }

        rv
    }

}

#[cfg(test)]
mod tests {
    use super::solutions::*;
    use super::*;
    use aoc_zen_runner_macros::aoc_case;
    use test_case::test_case;

    #[aoc_case(21, 525152)]
    const INPUT1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    // #[test_case("#.#.###", &[1,1,3], true; "Simple case from example")]
    // #[test_case(".##.###", &[1,1,3], false; "Identify group too long")]
    // #[test_case("#.#.#.##", &[1,1,3], false; "Identify group too short")]
    // #[test_case(".###.##.#.##", &[3,2,1], false; "Too many groups")]
    // #[test_case(".###.##.....", &[3,2,1], false; "Too few groups")]
    // fn test_line_matching(line: &str, groups: &[u32], expected: bool) {
    //     assert_eq!(expected, line_matches_groups(line, groups));
    // }

    // #[test_case(3, &[2,2,5], None; "Too big case")]
    // #[test_case(4, &[4], Some(1); "Single group, exact size")]
    // #[test_case(4, &[3], Some(2); "Single group, just one short")]
    // #[test_case(5, &[3], Some(3); "Single group, just two short")]
    // #[test_case(5, &[3,2], None; "Two groups, no space")]
    // #[test_case(7, &[3,3], Some(1); "Two groups, exact size")]
    // #[test_case(7, &[3,2], Some(3); "Two groups, one short")]
    // #[test_case(11, &[2,2,5], Some(1); "Three groups, exact size")]
    // #[test_case(12, &[2,2,5], Some(4); "Three groups, one short")]
    // #[test_case(13, &[2,2,5], Some(10); "Three groups, two short")]
    // fn test_wilds_helper(seg_size: u32, groups: &[u32], expected: Option<u32>) {
    //     assert_eq!(expected, all_wilds_helper(seg_size, groups))
    // }

    // #[test_case(bits![1, 1, 1, 1, 1, 0, 0, 1], 2, Some(1); "single: 1-seg/1-grp/1-opt")]
    // #[test_case(bits![1, 1, 1, 1, 1, 0, 0], 2, Some(1); "single: 1-seg/1-grp/1-opt edgy1")]
    // #[test_case(bits![1, 1, 1, 1, 1, 0, 0], 3, Some(1); "single: 1-seg/1-grp/1-opt edgy2")]
    // #[test_case(bits![1, 1, 1, 1, 1, 0, 0, 1], 3, Some(2); "single: 1-seg/1-grp/2-opt")]
    // #[test_case(bits![1, 1, 1, 1, 1, 0, 0, 1], 1, None; "single: 1-seg/1-grp/no-fit")]
    // #[test_case(bits![1, 0, 1, 1, 1, 0, 0, 1], 3, None; "single: 1-seg/1-grp/split-too-far")]
    // #[test_case(bits![0, 1, 1, 1, 0, 0], 3, None; "single: 1-seg/1-grp/split-too-far2")]
    // #[test_case(bits![1, 1, 1, 1, 0, 1, 1, 1], 4, Some(4); "single: 1-seg/1-grp/4-opt")]
    // fn test_solve_single(segment: &BitSlice, group: u32, expected: Option<u32>) {
    //     assert_eq!(expected, solve_single(segment, group));
    // }

    // #[test_case(&[bits![1, 1, 1, 1, 0, 1, 1, 1]], 3, Some(3); "grp: 1-seg/1-grp/valid")]
    // #[test_case(&[bits![1, 1, 1, 1, 0, 1, 1, 1], bits![1,1,1,1]], 3, Some(3); "grp: 2-seg/1-grp/1-fixed, 1-wild")]
    // #[test_case(&[bits![1, 1, 1, 1, 1, 1, 1, 1], bits![1,1,1,1]], 3, Some(8); "grp: 2-seg/1-grp/all-wild")]
    // #[test_case(&[bits![1, 1, 1, 1, 1, 0, 1, 1], bits![1,1,0,1]], 3, None; "grp: 2-seg/1-grp/2-fixed")]
    // #[test_case(&[bits![1, 1, 1, 1, 1, 1, 1, 0], bits![1,1,1,1]], 3, Some(1); "grp: 2-seg/1-grp/1-fixed,1-wild,edgy")]
    // #[test_case(&[bits![0, 1, 1, 1, 1, 1, 1, 1], bits![1,1,1,1]], 3, Some(1); "grp: 2-seg/1-grp/1-fixed,1-wild,other-edgy")]
    // #[test_case(&[bits![1, 1, 1, 1, 1, 1, 1, 1], bits![0,1,1,1]], 3, Some(1); "grp: 2-seg/1-grp/1-wild,1-fixed,other-edgy")]
    // #[test_case(&[bits![1, 1, 1, 1, 1, 1, 1, 1], bits![1,1,1,0]], 3, Some(1); "grp: 2-seg/1-grp/1-wild,1-fixed,edgy")]
    // fn test_solve_grp(segments: &[&BitSlice], group: u32, expected: Option<u32>) {
    //     assert_eq!(expected, solve_grp(segments, group));
    // }

    // #[test_case(bits![1, 1, 1, 1, 1, 0, 0, 1], &[2], Some(1); "segment: 1-seg/1-grp/1-opt")]
    // #[test_case(bits![1, 1, 1, 1, 1, 0, 0, 1], &[3], Some(2); "segment: 1-seg/1-grp/2-opt")]
    // #[test_case(bits![1, 0, 1, 1, 1, 0, 0, 1], &[1, 3], Some(2); "segment: 1-seg/2-grp/2-opt")]
    // #[test_case(bits![1, 0, 1, 1, 1, 1, 0, 0], &[1, 3], Some(1); "segment: 1-seg/2-grp/1-opt edgy")]
    // #[test_case(bits![1, 1, 1, 0, 0, 0, 1, 1, 0, 0], &[2, 3, 3], Some(1); "segment: 1-seg/3-grp/look-ahead-pass")]
    // #[test_case(bits![1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0], &[2, 3, 3], Some(2); "segment: 1-seg/3-grp/look-ahead-pass-2")]
    // #[test_case(bits![1, 1, 0, 0, 0, 1, 1, 0, 0], &[2, 3, 3], None; "segment: 1-seg/3-grp/look-ahead-fail")]
    // #[test_case(bits![0, 0, 0, 0, 1, 1, 0, 0], &[3, 3], None; "segment: 1-seg/3-grp/lead-zeroes-fail")]
    // #[test_case(bits![0, 0, 0, 1, 1, 0, 0, 1], &[3, 3], Some(2); "segment: 1-seg/3-grp/lead-zeroes-pass")]
    // #[test_case(bits![0, 0, 1, 1, 1, 0, 0, 1], &[3, 3], Some(2); "segment: 1-seg/3-grp/lead-zeroes-double-pass")]
    // #[test_case(bits![0, 1, 1, 1, 1, 1, 0, 0], &[1, 3], Some(1); "segment: 1-seg/2-grp/1-opt double-edgy")]
    // #[test_case(bits![1, 0, 1, 0, 1, 0, 0, 1], &[1, 3], None; "segment: 1-seg/2-grp/too-split")]
    // #[test_case(bits![1, 0, 1, 0, 1, 0, 1, 1], &[1, 3], Some(1); "segment: 1-seg/2-grp/straddle")]
    // #[test_case(bits![1, 0, 1, 0, 1, 0, 1, 1], &[3, 1], Some(1); "segment: 1-seg/2-grp/straddle2")]
    // #[test_case(bits![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1], &[2, 1, 3], Some(6); "segment: 1-seg/2-grp/straddle3")]
    // fn test_solve_seg(segment: &BitSlice, groups: &[u32], expected: Option<u32>) {
    //     assert_eq!(expected, solve_segment(segment, groups));
    // }

    // #[test_case(&[bits![1, 1, 1, 1, 1, 0, 0, 1]], &[2], Some(1); "groups: 1-seg/1-grp/1-opt")]
    // #[test_case(&[bits![1, 1, 1, 1, 1, 0, 0, 1]], &[3], Some(2); "groups: 1-seg/1-grp/2-opt")]
    // #[test_case(&[bits![1, 0, 1, 1, 1, 0, 0, 1]], &[1, 3], Some(2); "groups: 1-seg/2-grp/2-opt")]
    // #[test_case(&[bits![1, 0, 1, 1, 1, 1, 0, 0]], &[1, 3], Some(1); "groups: 1-seg/2-grp/2-opt edgy")]
    // #[test_case(&[bits![1,0,1], bits![1,1], bits![1,1,1,0]], &[1, 3], Some(1); "groups: 3-seg/2-grp/1-opt edgy anchored")]
    // #[test_case(&[bits![1,0,1], bits![1,1], bits![1,1,1,0]], &[2, 3], Some(2); "groups: 3-seg/2-grp/2-opt edgy anchored")]
    // #[test_case(&[bits![1, 1, 1], bits![0, 0, 0], bits![1, 1, 1], bits![0, 0, 0]], &[1, 1, 3, 1, 1, 3], Some(1); "groups: 4-seg/6-grp/1-opt multi-group per wild block")]
    // fn test_solve(segments: &[&BitSlice], groups: &[u32], expected: Option<u32>) {
    //     assert_eq!(expected, solve_groups(segments, groups))
    // }
}
