use crate::models::{Card, Hand, HandBidPair};

pub fn parse_card(character: char) -> Card {
    match character {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::T,
        v => Card::Number(v.to_digit(10).unwrap()),
    }
}
/// given input
pub fn parse_hand(input: &str) -> Hand {
    Hand::new(
        input
            .trim()
            .chars()
            .map(|character| parse_card(character))
            .collect::<Vec<Card>>(),
    )
}
pub fn parse_hand_to_bid(line: &str) -> HandBidPair {
    let sections = line.split_whitespace().collect::<Vec<&str>>();
    let hand = parse_hand(sections.first().unwrap());
    let bid = sections.last().unwrap().parse::<u32>().unwrap();
    HandBidPair::new(hand, bid)
}
