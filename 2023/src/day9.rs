use aoc_zen_runner_macros::{aoc, generator, solution, solver};
use itertools::Itertools;

#[aoc(2023, day9)]
pub mod solutions {
    use super::*;

    pub fn extend_series_once(series: &mut impl Iterator<Item = i64>) -> i64 {
        // Returns the next item in the series we're iterating over.
        // DO NOT RETURN THE DELTA.
        let first = series.next().unwrap();
        let mut all_zero = true;
        let mut last = first;

        let series_diffs = series
            .scan(first, |prev, curr| {
                let diff = curr - *prev;
                *prev = curr;
                last = curr;
                all_zero = all_zero && (diff == 0);
                Some(diff)
            })
            .collect_vec();

        if all_zero {
            last
        } else {
            extend_series_once(&mut series_diffs.into_iter()) + last
        }
    }

    // ----------------------- Part 1 -----------------------
    #[solution(part1, draft_soln)]
    pub fn part1_draft(input: &str) -> i64 {
        let mut tally = 0;

        for l in input.lines() {
            let incr = extend_series_once(&mut l.split_whitespace().map(|s| s.parse::<i64>().unwrap()));
            // println!("For line '{}', computed next item: {} ", l, incr);
            tally += incr;
        }

        tally
    }

    // ----------------------- Part 2 -----------------------
    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> i64 {
        let mut tally = 0;

        for l in input.lines() {
            let incr = extend_series_once(&mut l.split_whitespace().map(|s| s.parse::<i64>().unwrap()).rev());
            // println!("For line '{}', computed next item: {} ", l, incr);
            tally += incr;
        }

        tally
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::solutions::*;
    use test_case::test_case;
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(114, 2)]
    const input1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test_case(&[1,1,1,1,1], 1; "all ones")]
    #[test_case(&[1,2,3,4,5], 6; "linear series")]
    pub fn test_extender(items: &[i64], expected: i64) {
        assert_eq!(expected, extend_series_once(&mut items.into_iter().map(|&i| i)));
    }
}
