use std::fmt::Display;

use aoc_zen_runner_macros::{aoc, generator, solution, solver};
use colored::Colorize;
use grid::Grid;
use itertools::Itertools;

#[derive(Debug, Default, Clone, Copy)]
pub struct GridNode {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
    is_start: bool,
    is_loop: bool,
}

impl GridNode {
    fn is_vertical(&self) -> bool {
        // Yes for "┐|┌"
        //  No for "┘─└"
        self.south
    }
}

impl Display for GridNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_start {
            return write!(f, "{}", 'S');
        }
        let c = match (self.north, self.south, self.west, self.east) {
            (false, false, false, false) => ".",
            (true,  true,  false, false) => "│",
            (false, false, true,  true)  => "─",
            (true,  false, true,  false) => "┘",
            (true,  false, false, true)  => "└",
            (false, true,  true,  false) => "┐",
            (false, true,  false, true)  => "┌",
            _ => "?"
        };

        if self.is_loop {
            write!(f, "{}", c.bold().yellow())
        } else {
            write!(f, "{}", c)
        }
    }
}

pub struct Input {
    start_loc: (usize, usize),
    pipes: Grid<GridNode>,
}

#[aoc(2023, day10)]
pub mod solutions {
    use grid::Grid;
    use pathfinding::directed::dijkstra::dijkstra_all;
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Input {
        let mut lns = input.lines().peekable();
        let first_line = lns.peek().unwrap();
        let columns = first_line.as_bytes().len();

        let mut rv: Grid<GridNode> = Grid::new(0, columns);
        let mut c_idx: usize = 0;
        let mut r_idx: usize = 0;

        for (l_idx, ln) in lns.enumerate() {
            let row = ln.as_bytes().iter().enumerate().map(|(i, b)| match *b {
                b'|' => GridNode { north: true,  south: true,  east: false, west: false, is_start: false, is_loop: false },
                b'-' => GridNode { north: false, south: false, east: true,  west: true,  is_start: false, is_loop: false },
                b'L' => GridNode { north: true,  south: false, east: true,  west: false, is_start: false, is_loop: false },
                b'J' => GridNode { north: true,  south: false, east: false, west: true,  is_start: false, is_loop: false },
                b'7' => GridNode { north: false, south: true,  east: false, west: true,  is_start: false, is_loop: false },
                b'F' => GridNode { north: false, south: true,  east: true,  west: false, is_start: false, is_loop: false },
                b'.' => GridNode { north: false, south: false, east: false, west: false, is_start: false, is_loop: false },
                b'S' => { 
                    c_idx = i;
                    r_idx = l_idx;
                    GridNode { north: false, south: false, east: false, west: false, is_start: true, is_loop: false }
                },
                _ => unreachable!(),
            });

            let r: Vec<GridNode> = row.collect();

            //println!("Grid row: {}", &r.iter().join(""));

            rv.push_row(r);
        }

        assert_eq!(rv[(r_idx, c_idx)].is_start, true);

        Input {start_loc: (r_idx, c_idx), pipes: rv}
    }

    // ----------------------- Part 1 -----------------------

    #[solver(part1, draft_solvr)]
    pub fn solve_part1(input: Input) -> u32 {
        let Input {mut start_loc, pipes: mut grid} = input;

        fix_start_loc(&start_loc, &mut grid);
        
        let map = dijkstra_all(&start_loc, |&loc| {
            let (r, c) = loc;
            let gn = &grid[loc];
            let mut rv = vec![];

            if gn.north {
                rv.push(((r-1, c), 1));
            }
            if gn.south {
                rv.push(((r+1, c), 1));
            }
            if gn.west {
                rv.push(((r, c-1), 1));
            }
            if gn.east {
                rv.push(((r, c+1), 1));
            }

            rv
        });

        let max_node = map.values().max_by_key(|n| n.1).unwrap();

        // dbg!(max_node);

        max_node.1
    }

    fn fix_start_loc(start_loc: &(usize, usize), grid: &mut Grid<GridNode>) {
        let (r, c) = *start_loc;
        let north = r > 0 && grid[(r-1, c)].south;
        let south = r < grid.rows() - 1 && grid[(r+1, c)].north;
        let west  = c > 0 && grid[(r, c - 1)].east;
        let east  = c < grid.cols() - 1 && grid[(r, c + 1)].west;

        grid[*start_loc] = GridNode { north, south, east, west, is_start: true, is_loop: false};
    }

    // ----------------------- Part 2 -----------------------

    #[solver(part2, draft_solvr)]
    pub fn solve_part2(input: Input) -> u32 {
        let Input { start_loc, pipes: mut grid } = input;
        fix_start_loc(&start_loc, &mut grid);
        flood_fill_loop(&start_loc, &mut grid);

        let mut tally = 0;
        for r in grid.iter_rows() {
            let mut is_in = false;
            for gn in r {
                if gn.is_loop & gn.is_vertical() {
                    is_in = !is_in;
                } else if !gn.is_loop & is_in {
                    tally += 1;
                }
            }
        }

        tally
    }

    #[derive(Debug, Copy, Clone)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    fn flood_fill_loop(start_loc: &(usize, usize), grid: &mut Grid<GridNode>) {
        let (r, c) = *start_loc;
        let mut work_queue: Vec<(Direction, (usize, usize), GridNode)> = vec![];

        grid[*start_loc].is_loop = true;

        let gn = grid[*start_loc].clone();
        if gn.north {
            let next = (Direction::South, (r-1, c), grid[(r-1, c)]);
            work_queue.push(next);
        }
        if gn.south {
            let next = (Direction::North, (r+1, c), grid[(r+1, c)]);
            work_queue.push(next);
        }
        if gn.west {
            let next = (Direction::East, (r, c-1), grid[(r, c-1)]);
            work_queue.push(next);
        }
        if gn.east {
            let next = (Direction::West, (r, c+1), grid[(r, c+1)]);
            work_queue.push(next);
        }

        while work_queue.len() > 0 {
            let (src_dir, curr_loc, curr_node) = work_queue.pop().unwrap();
            let (r, c) = curr_loc;
            grid[curr_loc].is_loop = true;
            let (next_src_dir, next_coords) = match (src_dir, curr_node.north, curr_node.south, curr_node.west, curr_node.east) {
                (Direction::North, true,  true,  false, false) => (Direction::North, (r+1, c)),
                (Direction::North, true,  false, true,  false) => (Direction::East,  (r, c-1)),
                (Direction::North, true,  false, false, true)  => (Direction::West,  (r, c+1)),
                (Direction::South, true,  true,  false, false) => (Direction::South, (r-1, c)),
                (Direction::South, false, true,  true,  false) => (Direction::East,  (r, c-1)),
                (Direction::South, false, true,  false, true)  => (Direction::West,  (r, c+1)),
                (Direction::West,  true,  false, true,  false) => (Direction::South, (r-1, c)),
                (Direction::West,  false, true,  true,  false) => (Direction::North, (r+1, c)),
                (Direction::West,  false, false, true,  true)  => (Direction::West,  (r, c+1)),
                (Direction::East,  true,  false, false, true)  => (Direction::South, (r-1, c)),
                (Direction::East,  false, true,  false, true)  => (Direction::North, (r+1, c)),
                (Direction::East,  false, false, true,  true)  => (Direction::East,  (r, c-1)),
                _ => unreachable!(),
            };
            let next_gn = grid[next_coords];
            if !next_gn.is_loop {
                work_queue.push((next_src_dir, next_coords, next_gn));
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use aoc_zen_runner_macros::aoc_case;
    use super::*;
    use super::solutions::*;

    #[aoc_case(4, 1)]
    const input1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    #[aoc_case(4, 1)]
    const input1a: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[aoc_case(8)]
    const input2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[aoc_case(8)]
    const input2a: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[aoc_case(23, 4)]
    const P2_INPUT_1: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[aoc_case(22, 4)]
    const P2_INPUT_2: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    #[aoc_case(70, 8)]
    const P2_INPUT_3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

#[aoc_case(80, 10)]
    const P2_INPUT_4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
}
