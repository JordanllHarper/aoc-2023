use std::fs::read_to_string;

use utils::{calculate_winning_score, get_num_matching, process_card_from_line};

use crate::utils::{copy_count, Card};
mod utils;

fn main() {
    let input = read_lines("input.txt");
    let output = part_two(input);
    println!("{}", output);
}

fn part_one(input: Vec<String>) -> i32 {
    input
        .into_iter()
        .map(|line| process_card_from_line(&line))
        .map(|card| get_num_matching(&card))
        .map(|score| calculate_winning_score(score))
        .sum()
}

fn part_two(input: Vec<String>) -> i32 {
    let cards = input
        .into_iter()
        .map(|line| process_card_from_line(&line))
        .collect::<Vec<Card>>();
    copy_count(&cards, cards.clone())
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

    #[test]
    fn part_one_test() {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected = 13;
        let actual = part_one(
            data.lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>(),
        );
        assert_eq!(expected, actual);
    }
    #[test]
    fn part_two_test() {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected = 30;
        let actual = part_two(
            data.lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>(),
        );
        assert_eq!(expected, actual);
    }
}
