use aoc_zen_runner_macros::{aoc, solution};
use bitvec::prelude::*;
use bitvec::vec::BitVec;
use bitvec::{bitvec, slice::BitSlice};
use itertools::Itertools;

#[aoc(2023, day12)]
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
    pub fn part2_draft(input: &str) -> u32 {
        let mut tally = 0;

        for orig_ln in input.lines() {
            let (ln, grps_txt) = orig_ln.split_once(" ").unwrap();
            let ln = format!("{0}?{0}?{0}?{0}?{0}", ln);
            let grps_txt = format!("{0},{0},{0},{0},{0}", grps_txt);
            let grps: Vec<u32> = grps_txt
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            let prob_segments = ln.split('.').map(segment_to_bitvec).collect_vec();

            let incr = solve(&prob_segments, &grps);

            println!("Line {} had {:?} combos", orig_ln, &incr);

            tally += incr;
        }

        tally
    }

    pub fn segment_to_bitvec(segment: &str) -> BitVec {
        // TODO: SIMD
        let mut rv = bitvec!(0; 0);

        for b in segment.as_bytes() {
            rv.push(b == &b'?');
        }

        rv
    }

    pub fn solve(segments: &[BitVec], grps: &[u32]) -> u32 {
        let mut slices = vec![];

        for seg in segments {
            slices.push(seg.as_bitslice());
        }

        solve_groups(&slices[..], grps).unwrap()
    }

    pub fn solve_groups(segments: &[&BitSlice], grps: &[u32]) -> Option<u32> {
        // Convenience dispatch table
        match (segments.len(), grps.len()) {
            (0, _) => {
                return None;
            }
            (_, 0) => {
                return None;
            }
            (1, 1) => {
                return solve_single(segments[0], grps[0]);
            }
            (_, 1) => {
                return solve_grp(segments, grps[0]);
            }
            (1, _) => {
                return solve_segment(segments[0], grps);
            }
            (_, _) => {}
        };

        let mut tally = 0;

        let pivot_size = grps.iter().max().unwrap();

        for (idx, seg) in segments.iter().enumerate() {
            if seg.len() >= *pivot_size as usize {
                // Any of these could be empty, but none of this should raise exceptions
                let a_grps = &grps[0..idx];
                let pivot_grp = grps[idx];
                let b_grps = &grps[idx + 1..];

                let a_segs = &segments[0..idx];
                let pivot_seg = seg;
                let b_segs = &segments[idx + 1..];

                // If we fail to match at this pivot, another pivot may yet work.
                let Some(a_incr) = solve_groups(a_segs, a_grps) else {
                    continue;
                };
                let Some(pivot_incr) = solve_single(pivot_seg, pivot_grp) else {
                    continue;
                };
                let Some(b_incr) = solve_groups(b_segs, b_grps) else {
                    continue;
                };

                // If we succeed in this pivot, other pivots may yet yield more options
                tally += a_incr * pivot_incr * b_incr
            }
        }

        Some(tally)
    }

    pub fn solve_single(seg: &BitSlice, grp: u32) -> Option<u32> {
        if seg.count_zeros() == 0 {
            return all_wilds_helper(seg.len() as u32, &[grp]);
        }

        if seg.len() < seg.leading_ones() + seg.trailing_ones() + grp as usize {
            return None;
        }

        //OK, so all fixed-blocks are definitively within the range of the one group at this point.

        let block_span: u32 = (seg.len() - seg.leading_ones() - seg.trailing_ones()) as u32;

        return Some(grp - block_span + 1);
    }

    pub fn solve_grp(segments: &[&BitSlice], grp: u32) -> Option<u32> {
        // If there's multiple segments with fixed-placements, it's impossible.
        let hash_count = segments
            .iter()
            .filter_map(|&bs| if bs.count_zeros() > 0 { Some(bs) } else { None })
            .count();

        if hash_count > 1 {
            return None;
        } else if hash_count == 1 {
            let seg = segments
                .iter()
                .filter_map(|&bs| if bs.count_zeros() > 0 { Some(bs) } else { None })
                .next()
                .unwrap();
            return solve_single(seg, grp);
        } else {
            // Free to place wherever fits.
            let mut tally = 0;
            for seg in segments {
                let Some(incr) = solve_single(seg, grp) else {
                    continue;
                };
                tally += incr;
            }
            return if tally == 0 { None } else { Some(tally) };
        }
    }

    pub fn solve_segment(seg: &BitSlice, grps: &[u32]) -> Option<u32> {
        // 1's are wild, 0's are fixed-hash. Match by groups of zeroes?
        if seg.count_zeros() == 0 {
            // Oh, we can do whatever.
            all_wilds_helper(seg.len() as u32, grps);
        }

        if grps.len() == 1 && seg.len() == grps[0] as usize {
            return Some(1);
        }

        let lead_blocks = seg.leading_zeros();
        if lead_blocks > 1 {
            if lead_blocks < grps[0] as usize {
                return None;
            } else if seg[lead_blocks..].leading_ones() < grps[0] as usize - lead_blocks {
                return None;
            } else {
                let seg_start = grps[0] as usize;
                let rest_seg = &seg[seg_start..];
                let rest_grps = &grps[1..];
                if rest_grps.len() > 0 {
                    // There's only one way to place this segment, but if the rest doesn't work, doesn't count
                    return solve_segment(rest_seg, rest_grps);
                } else {
                    return Some(1);
                }
            }
        }
        let leading_ones = seg.leading_ones();
        let next_block = &seg[leading_ones..];
        let next_zeroes = next_block.leading_zeros();
        if next_zeroes > grps[0] as usize {
            if leading_ones <= grps[0] as usize {
                // There has to be a divider between sections, which can't happen if the end of a group is in the 
                // middle of a block of '#'s
                return None;
            } else {
                // The next group fits entirely inside the block of 1's
                let combos = solve_single(&seg[..leading_ones], grps[0]).unwrap_or(0) * solve_segment(next_block, &grps[1..]).unwrap_or(0);
                if combos == 0 {
                    return None;
                } else {
                    return Some(combos);
                }
            }
        } else if next_zeroes == grps[0] as usize {
            // Only one path forward, so hopefully it works.
            return solve_segment(&next_block[next_zeroes..], &grps[1..]);
        } else {
            // There's a predictable number of ways we can place the next group, but the number of spots we pass to the
            // next recursion varies with placement
            let min_start: usize = leading_ones + next_zeroes - grps[0] as usize;
            let max_start: usize = leading_ones + grps[0] as usize;

            let mut tally = 0;
            for i in min_start..max_start {     // May need ..=
                let next_start = i + grps[0] as usize + 1; // Maybe +2
                let Some(incr) = solve_segment(&seg[next_start..], &grps[1..]) else {
                    continue;
                };

                tally += incr;
            }
            
            if tally == 0 {
                return None;
            } else {
                return Some(tally as u32);
            }
        }
    }

    pub fn all_wilds_helper(seg_size: u32, grps: &[u32]) -> Option<u32> {
        if seg_size < grps.iter().sum() {
            return None;
        }

        if grps.len() == 1 {
            return Some(seg_size - grps[0] + 1);
        }

        let grp_len = grps[0];

        let mut tally = 0;
        for i in 0..seg_size {
            let remaining_size = seg_size - i - grp_len - 1;
            // If the recursive call fails, we're probably done placing this group
            let Some(incr) = all_wilds_helper(remaining_size, &grps[1..]) else {
                break;
            };
            tally += incr;
        }

        if tally == 0 {
            None
        } else {
            Some(tally)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solutions::*;
    use super::*;
    use aoc_zen_runner_macros::aoc_case;
    use test_case::test_case;

    #[aoc_case(21, 506250)]
    const input1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test_case("#.#.###", &[1,1,3], true; "Simple case from example")]
    #[test_case(".##.###", &[1,1,3], false; "Identify group too long")]
    #[test_case("#.#.#.##", &[1,1,3], false; "Identify group too short")]
    #[test_case(".###.##.#.##", &[3,2,1], false; "Too many groups")]
    #[test_case(".###.##.....", &[3,2,1], false; "Too few groups")]
    fn test_line_matching(line: &str, groups: &[u32], expected: bool) {
        assert_eq!(expected, line_matches_groups(line, groups));
    }

    #[test_case(3, &[2,2,5], None; "Too big case")]
    #[test_case(4, &[4], Some(1); "Single group, exact size")]
    #[test_case(4, &[3], Some(2); "Single group, just one short")]
    #[test_case(5, &[3], Some(3); "Single group, just two short")]
    #[test_case(5, &[3,2], None; "Two groups, no space")]
    #[test_case(7, &[3,3], Some(1); "Two groups, exact size")]
    #[test_case(7, &[3,2], Some(3); "Two groups, one short")]
    #[test_case(11, &[2,2,5], Some(1); "Three groups, exact size")]
    #[test_case(12, &[2,2,5], Some(4); "Three groups, one short")]
    #[test_case(13, &[2,2,5], Some(10); "Three groups, two short")]
    fn test_wilds_helper(seg_size: u32, groups: &[u32], expected: Option<u32>) {
        assert_eq!(expected, all_wilds_helper(seg_size, groups))
    }

    #[test_case(&[bits![0b0000_0110]], &[2], Some(1); "1-seg/1-grp/1-opt")]
    #[test_case(&[bits![0b0000_0110]], &[3], Some(2); "1-seg/1-grp/2-opt")]
    #[test_case(&[bits![0b0100_0110]], &[1, 3], Some(2); "1-seg/2-grp/2-opt")]
    #[test_case(&[bits![0b0100_0011]], &[1, 3], Some(2); "1-seg/2-grp/2-opt edgy")]
    #[test_case(&[bits![1,0,1], bits!(1,1), bits!(1,1,1,0)], &[1, 3], Some(2); "3-seg/2-grp/2-opt edgy anchored")]
    fn test_solve(segments: &[&BitSlice], groups: &[u32], expected: Option<u32>) {
        assert_eq!(expected, solve_groups(segments, groups))
    }
}
