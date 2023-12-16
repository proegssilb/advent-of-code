use std::num::Wrapping;

use aoc_zen_runner_macros::{aoc, generator, solution, solver};

pub fn hash_char(state: &Wrapping<u8>, next_char: &u8) -> Wrapping<u8> {
    let mut state = state + Wrapping(*next_char);
    state *= 17;
    // There's a mod 256 step. This happens automatically with Wrapping<u8>.
    state
}

pub fn hash(data: &[u8]) -> u8 {
    data.iter().fold(Wrapping::default(), |state, char| hash_char(&state, char)).0
}

#[aoc(2023, day15)]
pub mod solutions {
    use std::collections::VecDeque;

    use bstr::ByteSlice;

    use super::*;

    // #[generator(gen)]
    // pub fn input_generator(input: &str) -> Vec<i32> {
    //     Vec::new()
    // }

    // ----------------------- Part 1 -----------------------

    #[solution(part1, draft_soln)]
    pub fn part1_draft(input: &str) -> u64 {
        input.split(',').map(|s| hash(s.trim().as_bytes()) as u64).sum()
    }

    // ----------------------- Part 2 -----------------------

    // #[solver(part2, draft_solvr)]
    // pub fn solve_part2(input: Vec<i32>) -> u32 {
    //     0
    // }

    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> usize {
        let mut boxes: Vec<VecDeque<(&str, u8)>> = vec![Default::default(); 256];
        let instrs = input.split(',').map(|s| parse_instr(s.trim()));

        for (op, label, focal) in instrs {
            let box_num = hash(label.as_bytes()) as usize;
            match op {
                '-' => {
                    if let Some((idx, _)) = boxes[box_num].iter().enumerate().find(|i| i.1.0 == label) {
                        boxes[box_num].remove(idx);
                    }
                }
                '=' => {
                    if let Some((idx, _)) = boxes[box_num].iter().enumerate().find(|i| i.1.0 == label) {
                        boxes[box_num][idx] = (label, focal);
                    } else {
                        boxes[box_num].push_back((label, focal));
                    }
                }
                _ => { unreachable!() }
            }
        }

        let mut tally = 0;

        for (b_idx, lens_box) in boxes.into_iter().enumerate() {
            for (idx, (label, focal)) in lens_box.into_iter().enumerate() {
                tally += (1+b_idx) * (idx + 1) * (focal as usize);
            }
        }

        tally
    }

    fn parse_instr(instr: &str) -> (char, &str, u8) {
        if instr.chars().last() == Some('-') {
            ('-', &instr[0..instr.len()-1], 0)
        } else {
            let (label, focal) = instr.split_once("=").expect(&format!("Instr {} contained neither - nor = ", &instr));
            ('=', label, focal.parse().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_zen_runner_macros::aoc_case;
    use super::*;
    use super::solutions::*;

    #[aoc_case(1320, 145)]
    const input1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
}
