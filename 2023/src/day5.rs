use std::ops::Sub;
use std::{cmp::max, cmp::min, ops::RangeInclusive};
use std::{mem::take, ops::Add};

use itertools::Itertools;

use aoc_zen_runner_macros::{aoc, solution};


fn range_overlaps<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where
    T: Ord,
{
    !(a.end() < b.start() || b.end() < a.start())
}

fn range_adjacent<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where
    T: Ord + Add<T> + From<u64> + Copy,
    <T as std::ops::Add>::Output: PartialEq<T>,
{
    *a.end() + 1.into() == *b.start() || *b.end() + 1.into() == *a.start()
}

fn range_intersect<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> Option<RangeInclusive<T>>
where
    T: Ord,
    T: Clone,
    T: Copy,
{
    if !range_overlaps(a, b) { return None; }
    if a == b { return Some(a.clone()); }

    let rv_start: T = *max(a.start(), b.start());
    let rv_end: T = *min(a.end(), b.end());

    Some(rv_start..=rv_end)
}

fn range_sub<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> Vec<RangeInclusive<T>>
where
    T: Ord,
    T: Clone,
    T: Copy,
    T: Sub<u64, Output = T>
{
    if a == b {
        return vec![];
    }

    if b.start() < a.start() && a.end() < b.end() {
        return vec![];
    }

    if !range_overlaps(a, b) {
        return vec![a.clone()];
    }

    if a.start() < b.start() && b.end() < a.end() {
        panic!("Time for a restructure!");
    }

    let range_start = if a.start() < b.start() {
        *a.start()
    } else {
        *b.end()
    };

    let range_end = if a.start() < b.start() {
        *b.start() - 1
    } else {
        *a.end() - 0
    };

    return vec![range_start..=range_end];
}

#[derive(Debug, Default)]
pub struct RangeLine {
    ranges_set: Vec<RangeInclusive<u64>>,
}

impl RangeLine {
    fn new() -> RangeLine {
        RangeLine {
            ranges_set: Vec::new(),
        }
    }

    fn set_range(&mut self, x: &RangeInclusive<u64>) {
        let mut x = x.clone();
        for idx in 0..self.ranges_set.len() {
            while idx < self.ranges_set.len()
                && (range_overlaps(&x, &self.ranges_set[idx])
                    || range_adjacent(&x, &self.ranges_set[idx]))
            {
                let old = self.ranges_set.remove(idx);
                let existing_min = *old.start();
                let existing_max = *old.end();
                let new_min = min(*x.start(), existing_min);
                let new_max = max(*x.end(), existing_max);
                x = new_min..=new_max;
            }
        }
        self.ranges_set.push(x);
    }

    fn unset_range(&mut self, x: &RangeInclusive<u64>) {
        let mut new_ranges: Vec<RangeInclusive<u64>> = Vec::with_capacity(self.ranges_set.capacity());
        for r in &self.ranges_set {
            if range_intersect(&r, x).is_some() {
                for new_r in range_sub(r, x) {
                    if new_r.is_empty() {
                        continue;
                    } else {
                        new_ranges.push(new_r);
                    }
                }
            } else {
                new_ranges.push(r.clone());
            }
        }
        self.ranges_set = new_ranges;
    }

    fn set_range_line(&mut self, x: &RangeLine) {
        for r in &x.ranges_set {
            self.set_range(r);
        }
    }

    fn unset_point(&mut self, pt: u64) {
        for idx in 0..self.ranges_set.len() {
            if self.ranges_set[idx].contains(&pt) {
                let range_a = *self.ranges_set[idx].start()..=(pt - 1);
                let range_b = (pt + 1)..=*(self.ranges_set[idx].end());
                self.ranges_set[idx] = range_a;
                self.set_range(&range_b);
                return;
            }
        }
    }

    fn len(&self) -> u64 {
        self.ranges_set
            .iter()
            .map(|r| {
                let e = *r.end();
                let s = *r.start();
                (e.abs_diff(s) + 1) as u64
            })
            .sum()
    }

    fn normalize(&mut self) {
        let ranges = take(&mut self.ranges_set);
        for r in ranges {
            self.set_range(&r);
        }
    }

    fn ranges_count(&self) -> usize {
        self.ranges_set.len()
    }

    fn contains(&self, point: u64) -> bool {
        self.ranges_set.iter().any(|r| r.contains(&point))
    }

    fn raw<'a>(&'a self) -> &'a Vec<RangeInclusive<u64>> {
        &self.ranges_set
    }
}


#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Default)]
pub struct InputP1 {
    pub seeds: Vec<u64>,
    pub seed_to_soil: RangesMap,
    pub soil_to_fertilizer: RangesMap,
    pub fertilizer_to_water: RangesMap,
    pub water_to_light: RangesMap,
    pub light_to_temperature: RangesMap,
    pub temperature_to_humidity: RangesMap,
    pub humidity_to_location: RangesMap,
}

#[derive(Default)]
pub struct RangesMap {
    items: Vec<InpRangeLine>,
}

impl RangesMap {
    fn get(&self, item: u64) -> u64 {
        for rl in &self.items {
            if rl.in_start <= item && item < rl.in_start + rl.length {
                return item - rl.in_start + rl.out_start;
            }
        }

        item
    }

    fn get_range(&self, items: RangeInclusive<u64>) -> RangeLine {
        let mut rv = RangeLine { ranges_set: vec![] };
        let mut req_rangeline = RangeLine { ranges_set: vec![items.clone()] };

        // Build up the input line
        'a: while req_rangeline.ranges_count() > 0 {
            let current = req_rangeline.ranges_set[0].to_owned();

            for rl in &self.items {
                let rlr = rl.get_in_range();

                if let Some(r_intersect) = range_intersect(&current, &rlr) {
                    req_rangeline.unset_range(&rlr);
                    let delta = rl.out_start - rl.in_start;
                    let out_range = (r_intersect.start() + delta)..=(r_intersect.end() + delta);
                    rv.set_range(&out_range);
                    continue 'a;
                }
            }

            // If we get all the way here, we didn't intersect at all.
            req_rangeline.unset_range(&(*current.start()..=*current.end()));
            rv.set_range(&current);
        }

        rv
    }

    fn add_range(&mut self, rl: InpRangeLine) {
        self.items.push(rl)
    }

    fn get_range_line(&self, item_line: &RangeLine) -> RangeLine {
        let mut rv = RangeLine::new();
        for r in &item_line.ranges_set {
            let res = self.get_range(r.clone());
            rv.set_range_line(&res);
        }

        rv
    }
}

#[derive(Default)]
pub struct InpRangeLine {
    in_start: u64,
    out_start: u64,
    length: u64,
}

impl InpRangeLine {
    pub fn parse(line: &str) -> InpRangeLine {
        let (dest_start, source_start, count) = line
            .split_whitespace()
            .map(|s| {
                s.parse::<u64>()
                    .expect(&format!("Found a non-number in map entry line: '{}'", s))
            })
            .collect_tuple()
            .expect("Map line contained wrong number of arguments.");

        InpRangeLine {
            in_start: source_start,
            out_start: dest_start,
            length: count,
        }
    }

    pub fn get_in_range(&self) -> RangeInclusive<u64> {
        self.in_start..=(self.in_start + self.length - 1)
    }
}

#[aoc(2023, day5)]
pub mod solutions {
    use super::*;

    pub fn p1_parse_basic(input: &str) -> InputP1 {
        let mut lines = input.lines();
        let seed_line = lines.next().expect("No data found in file.");

        let (_, seeds_str) = seed_line
            .split_once(':')
            .expect("First line didn't contain a colon.");
        let seeds = seeds_str.split_whitespace();

        let _ = lines.next();

        let mut rv = InputP1::default();
        rv.seeds = seeds
            .map(|s| s.parse().expect(&format!("Seed ID wasn't a number: {}", s)))
            .collect();

        let mut current_map = Category::Seed;

        for line in lines {
            if line.contains(":") {
                let (map_hint, _) = line.split_once('-').expect(&format!(
                    "Map header line does not contain a dash: '{}'",
                    &line
                ));

                let map_hint = map_hint.trim();

                current_map = match map_hint {
                    "seed" => Category::Seed,
                    "soil" => Category::Soil,
                    "fertilizer" => Category::Fertilizer,
                    "water" => Category::Water,
                    "light" => Category::Light,
                    "temperature" => Category::Temperature,
                    "humidity" => Category::Humidity,
                    _ => {
                        unreachable!("Found an invalid map-hint: {}", map_hint)
                    }
                };
                //println!("Processing table taking {:?} as its input", &current_map);
            } else {
                let line = line.trim();
                if line == "" {
                    //println!("Blank line found.");
                    continue;
                }

                let rl = InpRangeLine::parse(line);

                //println!("Map line ({}) derived mappings [{:?}]", line, rl.iter().map(|t| format!("({} -> {})", t.0, t.1)).join(", "));

                match current_map {
                    Category::Seed => {
                        rv.seed_to_soil.add_range(rl);
                    }
                    Category::Soil => {
                        rv.soil_to_fertilizer.add_range(rl);
                    }
                    Category::Fertilizer => {
                        rv.fertilizer_to_water.add_range(rl);
                    }
                    Category::Water => {
                        rv.water_to_light.add_range(rl);
                    }
                    Category::Light => {
                        rv.light_to_temperature.add_range(rl);
                    }
                    Category::Temperature => {
                        rv.temperature_to_humidity.add_range(rl);
                    }
                    Category::Humidity => {
                        rv.humidity_to_location.add_range(rl);
                    }
                    Category::Location => {
                        unreachable!(
                            "Current category is 'Location', which is not a thing to be mapped."
                        )
                    }
                }
            }
        }

        rv
    }

    // ----------------------- Part 1 -----------------------

    #[solution(part1, draft_soln)]
    pub fn solve_part1(input: &str) -> u64 {
        let mut tally = u64::MAX;

        let input = p1_parse_basic(input);

        for seed in input.seeds {
            let a = input.seed_to_soil.get(seed);
            let b = input.soil_to_fertilizer.get(a);
            let c = input.fertilizer_to_water.get(b);
            let d = input.water_to_light.get(c);
            let e = input.light_to_temperature.get(d);
            let f = input.temperature_to_humidity.get(e);
            let g = input.humidity_to_location.get(f);

            // println!("- {:?}, {:?}, {:?}, {:?}, {:?}, {:?},\n  {:?}, {:?}\n", seed, a, b, c, d, e, f, g);

            tally = min(tally, g);
        }

        tally
    }

    // ----------------------- Part 2 -----------------------

    #[derive(Default)]
    pub struct InputP2 {
        pub seeds: Vec<RangeInclusive<u64>>,
        pub seed_to_soil: RangesMap,
        pub soil_to_fertilizer: RangesMap,
        pub fertilizer_to_water: RangesMap,
        pub water_to_light: RangesMap,
        pub light_to_temperature: RangesMap,
        pub temperature_to_humidity: RangesMap,
        pub humidity_to_location: RangesMap,
    }

    pub fn p2_parse_basic(input: &str) -> InputP2 {
        let mut lines = input.lines();
        let seed_line = lines.next().expect("No data found in file.");

        let (_, seeds_str) = seed_line
            .split_once(':')
            .expect("First line didn't contain a colon.");
        let seeds = seeds_str.split_whitespace();

        let _ = lines.next();

        let mut rv = InputP2::default();
        rv.seeds = seeds
            .map(|s| s.parse().expect(&format!("Seed ID wasn't a number: {}", s)))
            .batching(|it| match it.next() {
                None => None,
                Some(x) => match it.next() {
                    None => None,
                    Some(y) => Some(x..=(x+y-1)),
                },
            })
            .collect();

        let mut current_map = Category::Seed;

        for line in lines {
            if line.contains(":") {
                let (map_hint, _) = line.split_once('-').expect(&format!(
                    "Map header line does not contain a dash: '{}'",
                    &line
                ));

                let map_hint = map_hint.trim();

                current_map = match map_hint {
                    "seed" => Category::Seed,
                    "soil" => Category::Soil,
                    "fertilizer" => Category::Fertilizer,
                    "water" => Category::Water,
                    "light" => Category::Light,
                    "temperature" => Category::Temperature,
                    "humidity" => Category::Humidity,
                    _ => {
                        unreachable!("Found an invalid map-hint: {}", map_hint)
                    }
                };
                //println!("Processing table taking {:?} as its input", &current_map);
            } else {
                let line = line.trim();
                if line == "" {
                    //println!("Blank line found.");
                    continue;
                }

                let rl = InpRangeLine::parse(line);

                //println!("Map line ({}) derived mappings [{:?}]", line, rl.iter().map(|t| format!("({} -> {})", t.0, t.1)).join(", "));

                match current_map {
                    Category::Seed => {
                        rv.seed_to_soil.add_range(rl);
                    }
                    Category::Soil => {
                        rv.soil_to_fertilizer.add_range(rl);
                    }
                    Category::Fertilizer => {
                        rv.fertilizer_to_water.add_range(rl);
                    }
                    Category::Water => {
                        rv.water_to_light.add_range(rl);
                    }
                    Category::Light => {
                        rv.light_to_temperature.add_range(rl);
                    }
                    Category::Temperature => {
                        rv.temperature_to_humidity.add_range(rl);
                    }
                    Category::Humidity => {
                        rv.humidity_to_location.add_range(rl);
                    }
                    Category::Location => {
                        unreachable!(
                            "Current category is 'Location', which is not a thing to be mapped."
                        )
                    }
                }
            }
        }

        rv
    }

    #[solution(part2, draft_solvr)]
    pub fn solve_part2(input: &str) -> u64 {
        let mut tally = u64::MAX;

        let mut input = p2_parse_basic(input);

        for seed in input.seeds.iter_mut().flat_map(|i| i) {
            //println!("Input range: {}..={}", seed.start(), seed.end());
            let a = input.seed_to_soil.get(seed);
            // print!("    {:<20}: ", "Seed to soil");
            // for r in &a.ranges_set {
            //     print!("{}..={}, ",  r.start(), r.end())
            // }
            // println!("");

            let b = input.soil_to_fertilizer.get(a);
            // print!("    {:<20}: ", "Soil to Fertilizer");
            // for r in &b.ranges_set {
            //     print!("{}..={}, ",  r.start(), r.end())
            // }
            // println!("");

            let c = input.fertilizer_to_water.get(b);
            // print!("    {:<20}: ", "Fertilizer to Fater");
            // for r in &c.ranges_set {
            //     print!("{}..={}, ",  r.start(), r.end())
            // }
            // println!("");

            let d = input.water_to_light.get(c);
            // print!("    {:<20}: ", "Water to Light");
            // for r in &d.ranges_set {
            //     print!("{}..={}, ",  r.start(), r.end())
            // }
            // println!("");

            let e = input.light_to_temperature.get(d);
            // print!("    {:<20}: ", "Light to Temperature");
            // for r in &e.ranges_set {
            //     print!("{}..={}, ",  r.start(), r.end())
            // }
            // println!("");

            let f = input.temperature_to_humidity.get(e);
            // print!("    {:<20}: ", "Temperature to Humidity");
            // for r in &f.ranges_set {
            //     print!("{}..={}, ",  r.start(), r.end())
            // }
            // println!("");

            let g = input.humidity_to_location.get(f);

            // print!("    {:<20}: ", "Output ranges");
            // for r in &g.ranges_set {
            //     print!("{}..={}, ",  r.start(), r.end())
            // }
            // println!("");

            //let g_min = g.ranges_set.iter().fold(u64::MAX, |a, b| min(a, *b.start()));

            tally = min(tally, g);
        }

        tally
    }
}

#[cfg(test)]
mod tests {
    use super::solutions::*;
    use super::*;
    use test_case::test_case;
    use aoc_zen_runner_macros::aoc_case;

    #[aoc_case(35, 46)]
    const input1: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4
    ";

    #[test_case(5..=10, 7..=12, Some(7..=10); "overlap second-larger")]
    #[test_case(3..=7, 5..=10, Some(5..=7); "overlap first-larger")]
    #[test_case(3..=12, 5..=10, Some(5..=10); "overlap first-encompassing")]
    #[test_case(7..=14, 3..=122, Some(7..=14); "overlap second-encompassing")]
    #[test_case(7..=14, 22..=122, None; "No intersection")]
    pub fn test_range_intersect(a: RangeInclusive<u64>, b: RangeInclusive<u64>, exp: Option<RangeInclusive<u64>>) {
        assert_eq!(exp, range_intersect(&a, &b));
    }

    #[test_case(5..=10, 7..=12, vec![5..=6]; "overlap second-larger")]
    #[test_case(3..=7, 5..=10, vec![3..=4]; "overlap first-larger")]
    // #[test_case(3..=12, 5..=10, Some(5..=10); "overlap first-encompassing")]
    #[test_case(7..=14, 3..=122, vec![]; "overlap second-encompassing")]
    #[test_case(7..=14, 22..=122, vec![7..=14]; "No intersection")]
    pub fn test_range_sub(a: RangeInclusive<u64>, b: RangeInclusive<u64>, exp: Vec<RangeInclusive<u64>>) {
        assert_eq!(exp, range_sub(&a, &b));
    }
}
