use std::fs::read_to_string;

use models::HandBidPair;

use crate::{
    parser::parse_hand_to_bid,
    solve::{calculate_winnings, determine_rankings},
};

mod models;
mod parser;
mod solve;
fn main() {
    let input = read_lines("input.txt");
    let output = part_two(input);
    println!("{}", output);
}

fn part_one(input: Vec<String>) -> u32 {
    let mut hands_to_bids_mapping = input
        .iter()
        .map(|hand| parse_hand_to_bid(hand))
        .collect::<Vec<HandBidPair>>();

    hands_to_bids_mapping.sort_by(|one, two| one.cmp(two));
    let rankings_to_bids = determine_rankings(hands_to_bids_mapping);

    calculate_winnings(rankings_to_bids)
}

// TODO:
fn part_two(input: Vec<String>) -> u32 {
    let mut hands_to_bids_mapping = input
        .iter()
        .map(|hand| parse_hand_to_bid(hand))
        .collect::<Vec<HandBidPair>>();
    for hand_bids in &hands_to_bids_mapping {
        println!(
            "{:?} -> {:?}",
            hand_bids.hand,
            hand_bids.hand.get_hand_type_joker_variant()
        )
    }

    hands_to_bids_mapping.sort_by(|one, two| one.cmp(two));
    let rankings_to_bids = determine_rankings(hands_to_bids_mapping);

    calculate_winnings(rankings_to_bids)
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
#[cfg(test)]
mod tests {

    use crate::part_one;
    use crate::part_two;
    // NOTE: part 2 deprecates this so it won't ever work
    // #[test]
    // fn part_one_test() {
    //     let data = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483"
    //         .lines()
    //         .map(|line| line.to_string())
    //         .collect::<Vec<String>>();
    //     let expected = 6440;
    //     let actual = part_one(data);
    //     assert_eq!(expected, actual);
    // }
    // TODO:
    #[test]
    fn part_two_test() {
        let data = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483"
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let expected = 1;
        let actual = part_two(data);
        assert_eq!(expected, actual);
    }
}
