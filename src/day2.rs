use aoc_runner_derive::{aoc_generator, aoc};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameThrow {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SecondColumn {
    X,
    Y,
    Z,
}

pub type RoundInput = (GameThrow, SecondColumn);

pub type Round = (GameThrow, GameThrow);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<RoundInput> {
    let mut results: Vec<RoundInput> = Vec::new();
    for round_str in input.lines() {
        let round_bits: Vec<&str> = round_str.split_whitespace().collect();
        let (opponent_str, self_str) = (round_bits[0], round_bits[1]);
        let opp_move = match opponent_str {
            "A" => GameThrow::Rock,
            "B" => GameThrow::Paper,
            "C" => GameThrow::Scissors,
            _ => panic!("Invalid entry found for opponent move.")
        };
        let self_move = match self_str {
            "X" => SecondColumn::X,
            "Y" => SecondColumn::Y,
            "Z" => SecondColumn::Z,
            _ => panic!("Invalid entry found for second_column.")
        };
        results.push((opp_move, self_move));
    }
    results
}

/// Given an opponent's move, what move will cause us to win?
fn which_wins(act: GameThrow) -> GameThrow {
    match act {
        GameThrow::Rock => GameThrow::Paper,
        GameThrow::Paper => GameThrow::Scissors,
        GameThrow::Scissors => GameThrow::Rock,
    }
}

/// Given an opponent's move, what move will cause us to lose?
fn which_loses(act: GameThrow) -> GameThrow {
    match act {
        GameThrow::Rock => GameThrow::Scissors,
        GameThrow::Paper => GameThrow::Rock,
        GameThrow::Scissors => GameThrow::Paper,
    }
}

fn is_win(opp_move: GameThrow, self_move: GameThrow) -> bool {
    self_move == which_wins(opp_move)
}

fn interpret_round_p1(inr: &RoundInput) -> Round {
    let self_move = match inr {
        (_, SecondColumn::X) => GameThrow::Rock,
        (_, SecondColumn::Y) => GameThrow::Paper,
        (_, SecondColumn::Z) => GameThrow::Scissors,
    };
    (inr.0, self_move)
}

fn interpret_round_p2(inr: &RoundInput) -> Round {
    let self_move = match inr {
        (o, SecondColumn::X) => which_loses(*o),
        (o, SecondColumn::Y) => *o,
        (o, SecondColumn::Z) => which_wins(*o),
    };
    (inr.0, self_move)
}

fn score(round: Round) -> u64 {
    const WIN_POINTS: u64 = 6;
    const DRAW_POINTS: u64 = 3;
    const LOSE_POINTS: u64 = 0;

    let victory_points = match round {
        (o, s) if is_win(o, s) => WIN_POINTS,
        (theirs, mine) if theirs == mine => DRAW_POINTS,
        _ => LOSE_POINTS,
    };
    let move_points = match round.1 {
        GameThrow::Rock => 1,
        GameThrow::Paper => 2,
        GameThrow::Scissors => 3,
    };
    move_points + victory_points
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[RoundInput]) -> u64 {
    input
        .iter()
        .map(interpret_round_p1)
        .map(score)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[RoundInput]) -> u64 {
    input.iter()
        .map(interpret_round_p2)
        .map(score)
        .sum()
}

