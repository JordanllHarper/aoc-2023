#[derive(Debug, Clone)]
pub struct Card {
    id: i32,
    winning_nums: Vec<i32>,
    held_nums: Vec<i32>,
}

impl Card {
    pub fn new(id: i32, winning_nums: Vec<i32>, held_nums: Vec<i32>) -> Self {
        Self {
            id,
            winning_nums,
            held_nums,
        }
    }
}
pub fn get_num_matching(card: &Card) -> i32 {
    let mut num_matching = 0;
    for winning_num in &card.winning_nums {
        for held_num in &card.held_nums {
            if winning_num == held_num {
                num_matching += 1;
            }
        }
    }
    num_matching
}

pub fn process_card_from_line(input: &str) -> Card {
    let card_split = input.split(':').collect::<Vec<&str>>();
    let id = card_split
        .first()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let nums = card_split.last().unwrap().split('|').collect::<Vec<&str>>();

    let winning_nums = nums
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|each| each.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let held_nums = nums
        .first()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|each| each.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let card = Card::new(id, winning_nums, held_nums);
    card
}

pub fn copy_count(originals: &Vec<Card>, this_iteration_cards: Vec<Card>) -> i32 {
    let mut accum = 0;
    for card in this_iteration_cards {
        accum += 1;
        let ids = ids_to_copy(&card);
        let copy_from_ids = copy_from_ids(ids, originals);
        accum += copy_count(originals, copy_from_ids);
    }
    println!("{accum}");
    accum
}

pub fn copy_from_ids(ids: Vec<i32>, cards: &Vec<Card>) -> Vec<Card> {
    let mut copy_vec = Vec::new();
    for id in ids {
        for card in cards {
            if card.id == id {
                copy_vec.push(card.clone());
            }
        }
    }
    copy_vec
}

pub fn ids_to_copy(card: &Card) -> Vec<i32> {
    let current_id = card.id;

    let num_cards_to_copy = calculate_num_scratch_cards_to_copy(card);

    let num_to_copy = (1..=num_cards_to_copy)
        .into_iter()
        .map(|id| id + current_id)
        .collect::<Vec<i32>>();

    println!("Copying {:?}", num_to_copy);
    num_to_copy
}

pub fn calculate_num_scratch_cards_to_copy(card: &Card) -> i32 {
    let matching_nums = get_num_matching(card);
    (0..matching_nums).into_iter().count().try_into().unwrap() // this will never be less than 0
}

pub fn calculate_winning_score(num_correct_answers: i32) -> i32 {
    (0..num_correct_answers)
        .into_iter()
        .fold(0, |accum, _value| if accum == 0 { 1 } else { accum * 2 })
}
