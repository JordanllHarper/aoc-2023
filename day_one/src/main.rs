use std::fs::read_to_string;
fn main() {
    let lines = read_lines("input.txt");
    // let lines = vec![String::from("pppmfmnfourtworxrqrfhbgx8vvxgrjzhvqmztltwo")];

    let mapped_lines = lines
        .into_iter()
        .map(|line| parse_final_num_from_line(line));
    let sum: i32 = mapped_lines.sum();
    println!("{}", sum);
}

fn parse_final_num_from_line(line: String) -> i32 {
    let first = find_any(&line, false);
    println!("Resulting first {first}");
    let last = find_any(&line, true);
    println!("Resulting last {last}");

    let num = (first + &last).parse::<i32>().unwrap();
    println!("Num: {num}");
    num
}

// Iterate through string
// If first is num, return that
// If first is desired word, map to num, then return that
//
//
//
// Simplest form
//
// 11 -> 11
// oneone -> 11
// 1one -> 11
// one1 -> 11
//
//
// Assume we can only see one character
//
//
// o // we need to peek for remaining characters
// n // continue peeking
// e // we found our word
//
//
//

fn find_any(line: &str, inverse: bool) -> String {
    if inverse {
        println!("Backwards");
    } else {
        println!("Forwards");
    }
    let characters = line.chars();
    let characters: Vec<char> = if inverse {
        characters.rev().collect()
    } else {
        characters.collect()
    };

    for (num, character) in characters.clone().into_iter().enumerate() {
        let string_to_compare = characters.split_at(num).1;
        let string_to_compare = string_to_compare.to_vec().into_iter().collect::<String>();

        let find_num = find_num_from_character(&string_to_compare, inverse);
        if let Some(v) = find_num {
            return v;
        }
    }
    String::new()
}

fn find_num_from_character(line: &String, inverse: bool) -> Option<String> {
    println!("Input: {}", line);
    let mut word_accumulator = String::new();
    let chars: Vec<char> = line.chars().collect();

    for character in chars {
        word_accumulator += &character.to_string();
        println!("Character: {character}");
        println!("Accum: {word_accumulator}");
        let remaining_matching_words = remaining_potential_words(&word_accumulator, inverse);

        if remaining_matching_words.len() == 0 && !character.is_numeric() {
            return None;
        }

        if remaining_matching_words.len() >= 1 {
            let potential_matching = if inverse {
                let accum = &word_accumulator.clone();
                accum.chars().rev().collect::<String>()
            } else {
                word_accumulator.clone()
            };
            let opt_potential_word = map_word_to_num(&potential_matching);
            // if one of the words does match - return it
            // if it doesn't match, continue
            match opt_potential_word {
                Some(matching_num) => return Some(matching_num),
                None => {}
            };
        };
        if character.is_numeric() {
            return Some(character.to_string());
        }
    }
    None
}

fn remaining_potential_words(line: &String, inverse: bool) -> Vec<String> {
    let word_list = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    word_list
        .map(|word| {
            let word = word.to_string();
            if inverse {
                word.chars().rev().collect()
            } else {
                word
            }
        })
        .into_iter()
        .filter(|word| word.find(line).is_some())
        .collect()
}

fn map_word_to_num(word: &str) -> Option<String> {
    let mapping = match word {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => return None,
    };
    Some(mapping.to_string())
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
