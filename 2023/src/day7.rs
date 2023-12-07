use core::cmp::Ordering;
use std::collections::HashMap;

use aoc_zen_runner_macros::{aoc, generator, solution, solver};
use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
pub enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CardsHand {
    rank: HandType,
    cards: String,
    bid: u32,
}

pub fn count_cards(hand: &str) -> HashMap<char, u32> {
    let mut map: HashMap<char, u32> = HashMap::new();
    for c in hand.chars() {
        map.entry(c)
            .and_modify(|e| {
                *e += 1;
            })
            .or_insert(1);
    }

    map
}

impl CardsHand {
    pub fn new(hand: &str, bid: u32) -> CardsHand {
        let ct = CardsHand::hand_rank(hand);
        CardsHand {
            cards: hand.to_owned(),
            bid: bid,
            rank: ct,
        }
    }

    pub fn hand_rank(hand: &str) -> HandType {
        let card_count = count_cards(hand)
            .iter()
            .sorted_by_key(|(_, &v)| v)
            .rev()
            .map(|(&c, &v)| (c, v))
            .collect_vec();
        match card_count[..] {
            [(_, 5)] => HandType::FiveOfAKind,
            [(_, 4), (_, 1)] => HandType::FourOfAKind,
            [(_, 3), (_, 2)] => HandType::FullHouse,
            [(_, 3), (_, 1), (_, 1)] => HandType::ThreeOfAKind,
            [(_, 2), (_, 2), (_, 1)] => HandType::TwoPair,
            [(_, 2), ..] => HandType::OnePair,
            [(_, 1), ..] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for CardsHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => {}
            ord => return ord,
        }

        for (c1, c2) in self.cards.chars().zip(other.cards.chars()) {
            if c1 == c2 {
                continue;
            }

            // If we reach here, we must be able to determine the inequality this very card.

            if c1.is_ascii_digit() && c2.is_ascii_digit() {
                return c1.cmp(&c2);
            } else if c1.is_ascii_digit() {
                return Ordering::Less;
            } else if c2.is_ascii_digit() {
                return Ordering::Greater;
            }

            //At this point, they're both letters.
            let order = "TJQKA";
            let self_idx = order.find(c1).unwrap();
            let oth_idx = order.find(c2).unwrap();

            return self_idx.cmp(&oth_idx);
        }

        self.bid.cmp(&other.bid)
    }
}

impl PartialOrd for CardsHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CardsHand2 {
    rank: HandType,
    cards: String,
    bid: u32,
}

impl CardsHand2 {
    pub fn new(hand: &str, bid: u32) -> CardsHand2 {
        let ct = CardsHand2::hand_rank(hand);
        CardsHand2 {
            cards: hand.to_owned(),
            bid: bid,
            rank: ct,
        }
    }

    pub fn hand_rank(hand: &str) -> HandType {
        let card_counts = count_cards(hand);
        let mut card_count = card_counts
            .iter()
            .filter(|(&c, _)| c != 'J')
            .sorted_by_key(|(_, &v)| v)
            .rev()
            .map(|(&c, &v)| (c, v))
            .collect_vec();

        if let Some(j_count) = card_counts.get(&'J') {
            if card_count.len() > 0 {
                card_count[0] = (card_count[0].0, card_count[0].1 + j_count);
            }
        }

        match card_count[..] {
            [(_, 5)] => HandType::FiveOfAKind,
            [(_, 4), ..] => HandType::FourOfAKind,
            [(_, 3), (_, 2)] => HandType::FullHouse,
            [(_, 3), ..] => HandType::ThreeOfAKind,
            [(_, 2), (_, 2), ..] => HandType::TwoPair,
            [(_, 2), ..] => HandType::OnePair,
            [(_, 1), ..] => HandType::HighCard,
            [] => HandType::FiveOfAKind, // This is specifically the five-jokers case
            _ => unreachable!("Hand did not match categorizations: {}, {}", &hand, card_count.len()),
        }
    }
}

impl Ord for CardsHand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => {}
            ord => return ord,
        }

        for (c1, c2) in self.cards.chars().zip(other.cards.chars()) {
            if c1 == c2 {
                continue;
            }

            // If we reach here, we must be able to determine the inequality this very card.

            if c1 == 'J' {
                return Ordering::Less;
            } else if c2 == 'J' {
                return Ordering::Greater
            } else if c1.is_ascii_digit() && c2.is_ascii_digit() {
                return c1.cmp(&c2);
            } else if c1.is_ascii_digit() {
                return Ordering::Less;
            } else if c2.is_ascii_digit() {
                return Ordering::Greater;
            }

            //At this point, they're both letters.
            let order = "TJQKA";
            let self_idx = order.find(c1).unwrap();
            let oth_idx = order.find(c2).unwrap();

            return self_idx.cmp(&oth_idx);
        }

        self.bid.cmp(&other.bid)
    }
}

impl PartialOrd for CardsHand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(2023, day7)]
pub mod solutions {
    use super::*;

    #[generator(gen)]
    pub fn input_generator(input: &str) -> Vec<CardsHand> {
        let mut rv: Vec<CardsHand> = Vec::new();
        for ln in input.lines() {
            let (hand, bid_str) = ln.split_once(' ').unwrap();

            rv.push(CardsHand::new(hand, bid_str.parse().expect(&format!("Invalid bid: {:?}", &bid_str))));
        }

        rv
    }

    #[generator(gen2)]
    pub fn input_generator2(input: &str) -> Vec<CardsHand2> {
        let mut rv: Vec<CardsHand2> = Vec::new();
        for ln in input.lines() {
            let (hand, bid_str) = ln.split_once(' ').unwrap();

            rv.push(CardsHand2::new(hand, bid_str.parse().expect(&format!("Invalid bid: {:?}", &bid_str))));
        }

        rv
    }

    // ----------------------- Part 1 -----------------------

    #[solver(part1, draft_solvr)]
    pub fn solve_part1(input: Vec<CardsHand>) -> u64 {
        let t = input.iter().sorted().collect_vec();
        // println!("Sorted hand:");
        // for ch in &t {
        //     println!("    {:?}, ", ch);
        // }
        t.iter().enumerate().map(|(idx, ch)| (idx + 1) as u64 * (ch.bid as u64)).sum()
    }

    // ----------------------- Part 2 -----------------------

    #[solver(part2, draft_solvr)]
    pub fn solve_part2(input: Vec<CardsHand2>) -> u64 {
        let t = input.iter().sorted().collect_vec();
        //println!("Sorted hand:");
        // for ch in &t {
        //     println!("    {:?}, ", ch);
        // }
        t.iter().enumerate().map(|(idx, ch)| (idx + 1) as u64 * (ch.bid as u64)).sum()
    }

}

#[cfg(test)]
mod tests {
    use super::solutions::*;
    use super::*;
    use aoc_zen_runner_macros::aoc_case;
    use test_case::test_case;

    #[test_case("32T3K", HandType::OnePair ; "example - 32T3K")]
    #[test_case("T55J5", HandType::ThreeOfAKind ; "example - T55J5")]
    #[test_case("KK677", HandType::TwoPair ; "example - KK677")]
    #[test_case("KTJJT", HandType::TwoPair ; "example - KTJJT")]
    #[test_case("QQQJA", HandType::ThreeOfAKind ; "example - QQQJA")]
    pub fn test_hand_ranking(hand: &str, expected_rank: HandType) {
        assert_eq!(expected_rank, CardsHand::hand_rank(hand));
    }

    #[aoc_case(6440, 5905)]
    const input1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
}
