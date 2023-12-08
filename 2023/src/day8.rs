use std::collections::HashMap;

use aoc_zen_runner_macros::{aoc, generator, solution, solver};

pub struct Input {
    instructions: String,
    nodes: HashMap<String, (String, String)>
}

#[aoc(2023, day8)]
pub mod solutions {
    use num::integer::lcm;

    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let mut lines = input.lines();
        let instructions = lines.next().unwrap().to_owned();

        let mut nodes = HashMap::new();

        for l in lines {
            if l.trim().len() == 0 {
                continue;
            }

            let (label, opt_l, opt_r) = (&l[0..3], &l[7..10], &l[12..15]);

            nodes.insert(label.to_owned(), (opt_l.to_owned(), opt_r.to_owned()) );
        }

        Input { instructions, nodes}
    }

    // ----------------------- Part 1 -----------------------

    #[solver(part1, draft_solvr)]
    pub fn solve_part1(input: Input) -> usize {
        let instrs = input.instructions.chars().cycle().enumerate();
        let mut current = "AAA";

        for (idx, step) in instrs {
            // println!("Step {}, Node: {}, instr: {}", idx, &current, &step);
            if current == "ZZZ" {
                return idx;
            }

            let opts = &input.nodes[current];
            match step {
                'L' => { current = &opts.0; }
                'R' => { current = &opts.1; }
                _ => { unreachable!(); }
            }
        }

        unreachable!()
    }

    // ----------------------- Part 2 -----------------------

    #[solver(part2, draft_solvr)]
    pub fn solve_part2(input: Input) -> usize {
        let mut current = vec![];

        for node in input.nodes.keys() {
            match node.as_bytes() {
                [_, _, b'A'] => {current.push(node)}
                _ => {}
            }
        }

        // println!("Node count: {}", current.len());

        let mut path_lens: Vec<usize> = vec![];
        
        'a: for mut node in current {
            for (idx, step) in input.instructions.chars().cycle().enumerate() {
                // println!("Step {}, Node: {}, instr: {}", idx, &current, &step);
                if node.ends_with("Z") {
                    path_lens.push(idx);
                    continue 'a;
                }
    
                let opts = &input.nodes[node];
                match step {
                    'L' => { node = &opts.0; }
                    'R' => { node = &opts.1; }
                    _ => { unreachable!(); }
                }
            }
        }

        path_lens.iter().fold(1, |a, &b| lcm(a, b))
    }

}

#[cfg(test)]
mod tests {
    use super::solutions::*;

    const INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    pub fn p1_test1() {
        assert_eq!(2, solve_part1(input_generator(INPUT1)));
    }

    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    pub fn p1_test2() {
        assert_eq!(6, solve_part1(input_generator(INPUT2)));
    }

    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    pub fn p2_test1() {
        assert_eq!(6, solve_part2(input_generator(INPUT2)));
    }
}
