use aoc_zen_runner_macros::{aoc, generator, solution, solver};
use gridly::prelude::*;
use gridly_grids::VecGrid;

#[aoc(2023, day3)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> VecGrid<char> {
        let grid = VecGrid::new_from_rows(input.lines().map(|s| s.chars()));
        grid.expect("Failed to parse grid.")
    }

    // ----------------------- Part 1 -----------------------

    #[solver(part1, draft_solvr)]
    pub fn solve_part1(input: VecGrid<char>) -> u32 {
        let mut bit_grid =
            VecGrid::new_fill_copied(input.dimensions(), false).expect("Failed to create grid.");
        for r in input.rows().iter() {
            for (loc, c_val) in r.iter_with_locations() {
                if c_val.is_ascii_punctuation() && *c_val != '.' {
                    set_neighbors(&input, &mut bit_grid, loc);
                }
            }
        }

        // println!("Grid:\n{}", input.display_with(|&c| c));
        // println!("Summing:\n{}", bit_grid.display_with(|&b| if b { 'x' } else {'.'}));

        let mut sum = 0;

        for r in input.rows().iter() {
            let mut num = 0;
            for (loc, c_val) in r.iter_with_locations() {
                if c_val.is_ascii_digit() && bit_grid[loc] {
                    num = num * 10
                        + c_val
                            .to_digit(10)
                            .expect("Failed to parse what should be a digit.");
                } else {
                    if num != 0 {
                        sum += num;
                        num = 0;
                    }
                }
            }
            if num != 0 {
                sum += num;
            }
        }

        sum
    }

    pub fn set_neighbors<'a>(grid: &VecGrid<char>, bit_grid: &mut VecGrid<bool>, loc: Location) {
        for c in [-1isize, 0isize, 1isize] {
            for r in [-1isize, 0isize, 1isize] {
                if r == 0 && c == 0 {
                    continue;
                }

                let targ_loc = loc + (r, c);

                if targ_loc.row < Row(0)
                    || targ_loc.row >= Row(grid.num_rows().0)
                    || targ_loc.column < Column(0)
                    || targ_loc.column >= Column(grid.num_columns().0)
                {
                    continue;
                }

                let do_set = grid[targ_loc].is_ascii_digit();
                let was_set = bit_grid[targ_loc];

                if do_set && !was_set {
                    bit_grid[targ_loc] = bit_grid[targ_loc] || do_set;
                    set_neighbors(&grid, bit_grid, targ_loc);
                }
            }
        }
    }

    // ----------------------- Part 2 -----------------------

    #[solver(part2, draft_solvr)]
    pub fn solve_part2(input: VecGrid<char>) -> u32 {
        //println!("---- ---- ----");
        let mut bit_grid =
            VecGrid::new_fill_copied(input.dimensions(), false).expect("Failed to create grid.");

        let mut tally = 0u32;

        for r in input.rows().iter() {
            for (loc, c_val) in r.iter_with_locations() {
                if *c_val == '*' {
                    if let Some(ratio) = check_gear(&input, &mut bit_grid, loc) {
                        tally += ratio;
                    }
                }
            }
        }

        tally
    }

    pub fn check_gear<'a>(grid: &VecGrid<char>, bit_grid: &mut VecGrid<bool>, loc: Location) -> Option<u32> {
        let target_locations = extract_adjacent_numbers(grid, bit_grid, loc);

        if let [loc_a, loc_b] = target_locations[..] {
            let num_a = get_number(grid, loc_a).expect("Found no number where digit was found.");
            let num_b = get_number(grid, loc_b).expect("Found no number where digit was found.");
            Some( num_a * num_b)
        } else {
            None
        }
    }

    pub fn extract_adjacent_numbers(grid: &VecGrid<char>, bit_grid: &mut VecGrid<bool>, loc: Location) -> Vec<Location> {
        let mut target_locations: Vec<Location> = vec![];
        let mut locations_found: Vec<Location> = vec![];
        for c in [-1isize, 0isize, 1isize] {
            for r in [-1isize, 0isize, 1isize] {
                if r == 0 && c == 0 {
                    continue;
                }

                let targ_loc = loc + (r, c);

                if targ_loc.row < Row(0)
                    || targ_loc.row >= Row(grid.num_rows().0)
                    || targ_loc.column < Column(0)
                    || targ_loc.column >= Column(grid.num_columns().0)
                {
                    continue;
                }

                let do_set = grid[targ_loc].is_ascii_digit()
                    && locations_found.iter().all(|lo| !locations_same_number(&targ_loc, &lo));
                let was_set = bit_grid[targ_loc];

                if grid[targ_loc].is_ascii_digit() {
                    // Even if we're not keeping a given location, if it has a digit, we have to use it for same-number checking.
                    locations_found.push(targ_loc);
                }

                if do_set && !was_set {
                    target_locations.push(targ_loc);
                }
            }
        }

        target_locations
    }

    pub fn locations_same_number(loc1: &Location, loc2: &Location) -> bool {
        loc1.row == loc2.row && loc1.column.0.abs_diff(loc2.column.0) <= 1
    }

    pub fn get_number(grid: &VecGrid<char>, loc: Location) -> Option<u32> {
        if loc.row < Row(0)
            || loc.row >= Row(grid.num_rows().0)
            || loc.column < Column(0)
            || loc.column >= Column(grid.num_columns().0)
            || !grid[loc].is_ascii_digit()
        {
            return None;
        }

        let mut ll = loc.clone();

        // Seek left as far as we can
        while ll.column.0 >= 0 && grid[ll].is_ascii_digit() {
            ll += (0, -1);
        }

        // We're now one off.
        ll += (0, 1);

        // Read out a number
        let mut tally = 0u32;
        while ll.column.0 < grid.num_columns().0 && grid[ll].is_ascii_digit() {
            let d = grid[ll].to_digit(10).unwrap();
            //println!("Tallying: was {}, adding {}", tally, d);
            tally = tally*10 + d;
            ll += (0, 1);
        }

        //println!("Tally found {} at {:?}.", tally, loc);

        Some(tally)
    }
}

#[cfg(test)]
mod tests {
    use super::solutions::*;
    use super::*;
    use aoc_zen_runner_macros::aoc_case;
    use test_case::test_case;

    #[aoc_case(4361, 467835)]
    const input1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[aoc_case(352667, 215302299)]
    const input2: &str = "467..114..
...*...#..
..35..633.
..........
617*348192
..........
..592.....
.+....755.
...$.*....
.664.598..";

    #[aoc_case(352667, 215302299)]
    const input3: &str = "467..114..
...*...#..
..35..633.
617.......
...*348192
..........
..592.....
.+....755.
...$.*....
.664.598..";

#[aoc_case(352667, 215302299)]
const input4: &str = "467..114..
...*...#..
..35..633.
..617.....
...*348192
..........
..592.....
.+....755.
...$.*....
.664.598..";

#[test_case(input4, Location::new(1, 3), Some(16345) ; "input4, upper-left")]
#[test_case(input4, Location::new(8, 5), Some(451490) ; "input4, bottom-right")]
#[test_case(input4, Location::new(4, 3), Some(214834464) ; "input4, middle")]
pub fn test_check_gear(grid_str: &str, loc: Location, expected: Option<u32>) {
    let grid = input_generator(grid_str);
    let mut bit_grid = VecGrid::new_fill_copied(grid.dimensions(), false).expect("Failed to create grid.");
    println!("Cell contents: {}", grid[&loc]);
    assert_eq!(expected, check_gear(&grid, &mut bit_grid, loc));
}

#[test_case(input4, Location::new(1, 3), vec![Location::new(0,2), Location::new(2,2)] ; "input4, upper-left")]
#[test_case(input4, Location::new(8, 5), vec![Location::new(9,5), Location::new(7,6)] ; "input4, bottom-right")]
#[test_case(input4, Location::new(4, 3), vec![Location::new(3,2), Location::new(4,4)] ; "input4, middle")]
pub fn test_extract_adjacent_numbers(grid_str: &str, loc: Location, expected: Vec<Location>) {
    let grid = input_generator(grid_str);
    let mut bit_grid = VecGrid::new_fill_copied(grid.dimensions(), false).expect("Failed to create grid.");
    println!("Cell contents: {}", grid[&loc]);
    assert_eq!(expected, extract_adjacent_numbers(&grid, &mut bit_grid, loc));
}

#[test_case(input4, Location::new(2,3), Some(35) ; "input4 - R2C3 - 35")]
#[test_case(input4, Location::new(1,2), None ; "input4 - R2C2 - None")]
#[test_case(input4, Location::new(0,1), Some(467) ; "input4 - R0C1 - 467")]
#[test_case(input4, Location::new(0,7), Some(114) ; "input4 - R0C7 - 114")]
#[test_case(input4, Location::new(4,7), Some(348192) ; "input4 - R4C7 - 348192")]
#[test_case(input4, Location::new(4,4), Some(348192) ; "input4 - R4C4 - 348192")]
#[test_case(input4, Location::new(5,3), None ; "input4 - R3C3 - None")]
pub fn test_get_number(grid_str: &str, loc: Location, expected: Option<u32>) {
    let grid = input_generator(grid_str);
    println!("Cell contents: {}", grid[&loc]);
    assert_eq!(expected, get_number(&grid, loc))
}

#[test_case(Location::new(5,5), Location::new(5,3), false; "b away-left of a")]
#[test_case(Location::new(5,5), Location::new(5,4), true; "b left of a")]
#[test_case(Location::new(5,5), Location::new(4,4), false; "b uleft of a")]
#[test_case(Location::new(5,5), Location::new(4,5), false; "b up of a")]
#[test_case(Location::new(5,5), Location::new(4,6), false; "b uright of a")]
#[test_case(Location::new(5,5), Location::new(5,6), true; "b right of a")]
#[test_case(Location::new(5,5), Location::new(5,7), false; "b away-right of a")]
#[test_case(Location::new(5,5), Location::new(6,6), false; "b dright of a")]
#[test_case(Location::new(5,5), Location::new(6,5), false; "b down of a")]
#[test_case(Location::new(5,5), Location::new(6,4), false; "b dleft of a")]
pub fn test_adjacency(l1: Location, l2: Location, expected: bool) {
    assert_eq!(expected, locations_same_number(&l1, &l2));
}
    
}
