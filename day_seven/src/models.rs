use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum Card {
    A,
    K,
    Q,
    J,
    T,
    Number(u32),
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank_of_self = self.get_rank_of_card_joker_variant();
        let rank_of_other = other.get_rank_of_card_joker_variant();
        if rank_of_self > rank_of_other {
            return Ordering::Greater;
        }
        if rank_of_self < rank_of_other {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

impl Card {
    #[deprecated]
    pub fn get_rank_of_card(&self) -> u32 {
        match self {
            Card::A => 14, // most
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::Number(v) => *v, // least
        }
    }

    pub fn get_rank_of_card_joker_variant(&self) -> u32 {
        match self {
            Card::A => 13, // most
            Card::K => 12,
            Card::Q => 11,
            Card::T => 10,
            Card::Number(v) => *v,
            Card::J => 1, // least
        }
    }
}

/// Ranked from most to least valuable
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    Five = 7, // most
    Four = 6,
    Full = 5,
    Three = 4,
    Two = 3,
    One = 2,
    High = 1, // least
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct HandBidPair {
    pub hand: Hand,
    pub bid: u32,
}
impl Ord for HandBidPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl HandBidPair {
    pub fn new(hand: Hand, bid: u32) -> Self {
        Self { hand, bid }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Hand {
    pub cards: Vec<Card>,
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this_type = self.get_hand_type_joker_variant();
        let other_type = other.get_hand_type_joker_variant();

        let ordering = this_type.cmp(&other_type);
        match ordering {
            Ordering::Equal => {
                for (card_one, card_two) in &mut self.cards.iter().zip(other.cards.iter()) {
                    match card_one.cmp(card_two) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    };
                }
                Ordering::Equal
            }
            v => v,
        }
    }
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }
    pub fn get_hand_type_joker_variant(&self) -> HandType {
        let mut card_number_count = HashMap::new();
        for card in &self.cards {
            *card_number_count.entry(card).or_insert(0) += 1;
        }
        let card_number_count_length = card_number_count.len();
        let has_joker = card_number_count.contains_key(&Card::J);
        let sizes = card_number_count
            .iter()
            .map(|each| each.1)
            .collect::<Vec<&i32>>();
        match card_number_count_length {
            1 => HandType::Five, // no differences
            2 => {
                // this is correct
                // AA8AA -> AAAA 8 -> 2 differences and one has 4 cards

                // JJ8JJ -> JJJJ 8 -> 88888 -> Five
                // or...
                // JJJ88 or 88 JJJ
                if has_joker {
                    return HandType::Five;
                }
                for size in sizes {
                    if *size == 4 {
                        return HandType::Four;
                    }
                }
                // AAA88 -> 2 differences and one has 3 cards
                HandType::Full
            }
            3 => {
                // TTT98 -> TTT 9 8 -> 3 differences and one has 3 cards
                // JJJ98 -> JJJ 9 8 -> 9999 8 -> Four
                // TTTJ8 -> TTT J 8 -> TTTT 8 -> Four
                //
                for size in sizes {
                    if *size == 3 {
                        if has_joker {
                            return HandType::Four; // jokers in either case lead to a 4
                        }
                        return HandType::Three;
                    }
                }
                //

                // 2J4J2 -> 22 JJ 4 -> 2222 4 -> Four
                // 23J32 -> 22 33 J -> 333 22 -> Three
                if has_joker {
                    let joker_count = card_number_count.get(&Card::J).unwrap(); // we know we can
                                                                                // access this here
                    if *joker_count == 2 {
                        return HandType::Four;
                    } else {
                        return HandType::Full;
                    }
                }

                // 23432 -> 22 33 4 -> 3 differences and 2 have 2 cards
                HandType::Two
            }

            //A23A4 -> AA 2 3 4  -> 4 differences and one has 2 cards
            4 => {
                // J23J4 -> JJ 2 3 4 -> 222 3 4 -> 3
                // AJ3A4 -> AA J 3 4 -> AAA 3 4 -> 3
                if has_joker {
                    HandType::Three
                } else {
                    HandType::One
                }
            }
            _ => {
                if has_joker {
                    HandType::One
                } else {
                    HandType::High
                }
            }
        }
    }
    #[deprecated]
    pub fn get_hand_type(&self) -> HandType {
        let mut card_number_count = HashMap::new();
        for card in &self.cards {
            *card_number_count.entry(card).or_insert(0) += 1;
        }
        let card_number_count_length = card_number_count.len();
        let sizes = card_number_count
            .into_iter()
            .map(|each| each.1)
            .collect::<Vec<i32>>();
        match card_number_count_length {
            1 => HandType::Five, // no differences
            2 => {
                let first = sizes.first().unwrap();
                let last = sizes.last().unwrap();
                // AA8AA -> AAAA 8 -> 2 differences and one has 4 cards
                if *first == 4 || *last == 4 {
                    HandType::Four
                } else {
                    HandType::Full
                }
            }
            3 => {
                // TTT98 -> TTT 9 8 -> 3 differences and one has 3 cards
                for size in sizes {
                    if size == 3 {
                        return HandType::Three;
                    }
                }

                // 23432 -> 22 33 4 -> 3 differences and 2 have 2 cards
                HandType::Two
            }

            //A23A4 -> AA 2 3 4  -> 4 differences and one has 2 cards
            4 => HandType::One,
            _ => HandType::High,
        }
    }
}
