use crate::models::HandBidPair;

pub fn determine_rankings(hands_to_bids: Vec<HandBidPair>) -> Vec<(u32, u32)> {
    hands_to_bids
        .iter()
        .enumerate()
        .map(|(index, card)| ((index + 1).try_into().unwrap(), card.bid))
        .collect()
}
/// returns local rank and bid

pub fn calculate_winnings(rank_to_bids: Vec<(u32, u32)>) -> u32 {
    rank_to_bids.iter().map(|(rank, bid)| rank * bid).sum()
}
