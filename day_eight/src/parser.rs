use std::collections::HashMap;

use crate::models::{Destination, Direction, Map};

pub fn parse_to_models(input: &str) -> Map {
    let split = input.trim().split("\n\n").collect::<Vec<&str>>();

    let directions = split
        .first()
        .unwrap()
        .trim()
        .chars()
        .map(|character| Direction::from_character(character))
        .collect::<Vec<Direction>>();
    let nodes: HashMap<String, Destination> = split
        .last()
        .unwrap()
        .trim()
        .lines()
        .map(|line| produce_single_node(line))
        .collect::<HashMap<String, Destination>>();

    Map::new(directions, nodes)
}
pub fn produce_single_node(input: &str) -> (String, Destination) {
    let split = input.split('=').collect::<Vec<&str>>();
    let key = split.first().unwrap().trim().to_string();
    let node = split.last().unwrap().split(',').collect::<Vec<&str>>();
    let left = node
        .first()
        .unwrap()
        .trim()
        .chars()
        .filter(|character| character.is_alphanumeric())
        .collect::<String>();

    let right = node
        .last()
        .unwrap()
        .trim()
        .chars()
        .filter(|character| character.is_alphanumeric())
        .collect::<String>();

    (key, Destination::new(left, right))
}
