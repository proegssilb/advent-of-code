use aoc_zen_runner_macros::{aoc, generator, solver};

#[aoc(2024, day2)]
pub mod solutions {

    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
        input.lines().map(|ln| ln.split_ascii_whitespace().map(|i| i.parse().unwrap()).collect()).collect()
    }

    // ----------------------- Part 1 -----------------------

    #[solver(part1, draft_solvr)]
    pub fn solve_part1(input: Vec<Vec<i32>>) -> u32 {
        let mut tally = 0;
        for ln in input {
            let mut prev = ln[0];
            let mut increasing = true;
            let mut decreasing = true;
            let mut gentle = true;
            for item in ln[1..].iter() {
                increasing = increasing && prev < *item;
                decreasing = decreasing && prev > *item;
                let diff = prev.abs_diff(*item);
                gentle = gentle && (1 <= diff && diff <= 3);

                prev = *item;
            }

            if (increasing || decreasing) && gentle {
                tally += 1
            }
        }

        tally
    }

    // #[solution(part1, draft_soln)]
    // pub fn part1_draft(input: &str) -> u32 {
    //     0
    // }

    // ----------------------- Part 2 -----------------------

    fn test_line(ln: &Vec<i32>) -> bool {
        let mut prev = ln[0];
        let mut increasing = true;
        let mut decreasing = true;
        let mut gentle = true;
        for item in ln[1..].iter() {
            increasing = increasing && prev < *item;
            decreasing = decreasing && prev > *item;
            let diff = prev.abs_diff(*item);
            gentle = gentle && (1 <= diff && diff <= 3);

            prev = *item;
        }

        (increasing || decreasing) && gentle
    }

    #[solver(part2, draft_solvr)]
    pub fn solve_part2(input: Vec<Vec<i32>>) -> u32 {
        let mut tally = 0;
        for ln in input 
        {
            if test_line(&ln) {
                tally += 1;
            } 
            else 
            {
                let end = ln.len();
                for i in 0..end
                {
                    let test_list = [&ln[0..i], &ln[i+1..]].concat();

                    if test_line(&test_list) {
                        tally += 1;
                        break;
                    }
                }
            }
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

    #[aoc_case(2, 4)]
    const INPUT1: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
}
