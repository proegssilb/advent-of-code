use aoc_zen_runner_macros::{aoc, generator, solution, solver};
use itertools::Itertools;

#[derive(Debug)]
pub struct Race(u64, u64);

#[aoc(2023, day6)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Vec<Race> {
        let (time_line, distance_line) = input.split_once("\n").unwrap();
        let (_, time_line) = time_line.split_once(":").unwrap();
        let (_, distance_line) = distance_line.split_once(":").unwrap();

        let times = time_line.trim().split_whitespace().map(|s| s.parse::<u64>().unwrap());
        let distances = distance_line.trim().split_whitespace().map(|s| s.parse::<u64>().unwrap());

        times.zip(distances).map(|(a, b)| Race(a, b)).collect()
    }

    pub fn compute_race_distance(race: &Race, time_held: u64) -> u64 {
        let speed = time_held;
        let time_remaining = race.0 - time_held;

        speed * time_remaining
    }

    // ----------------------- Part 1 -----------------------

    #[solver(part1, draft_solvr)]
    pub fn solve_part1(input: Vec<Race>) -> u64 {
        let mut tally = 1;
        for r in input {
            let max_time = r.0;
            let options = (1..max_time).map(|t| compute_race_distance(&r, t)).filter(|d| d > &r.1);
            let options_count = options.count();

            // println!("Race: {:?} has winning options: {}", &r, options_count);

            tally *= options_count as u64;
        }

        tally
    }

    // ----------------------- Part 2 -----------------------
    #[solution(part2, draft_soln)]
    pub fn part2_draft(input: &str) -> u64 {
        let (time_line, distance_line) = input.split_once("\n").expect("Did not find second line.");
        let (_, time_line) = time_line.split_once(":").expect("Did not find colon in first line");
        let (_, distance_line) = distance_line.split_once(":").expect("Did not find colon in second line");

        let max_time = time_line.split_whitespace().filter(|s| !s.is_empty()).join("").parse::<u64>().expect("Could not parse max time.");
        let min_distance = distance_line.split_whitespace().filter(|s| !s.is_empty()).join("");
        let min_distance = min_distance.parse::<u64>().expect("Could not parse minimum distance");

        let r = Race(max_time, min_distance);

        let options = (1..max_time).map(|t| compute_race_distance(&r, t)).filter(|d| d > &r.1);
        let options_count = options.count() as u64;

        options_count
    }
}

#[cfg(test)]
mod tests {
    use aoc_zen_runner_macros::aoc_case;
    use super::*;
    use super::solutions::*;

    #[aoc_case(288, 71503)]
    const input1: &str = "Time:      7  15   30
    Distance:  9  40  200";
}
