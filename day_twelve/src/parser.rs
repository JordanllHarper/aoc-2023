use crate::models::{Record, SpringType};

pub fn parse_to_models(input: Vec<String>) -> Vec<Record> {
    input.into_iter().map(|each| parse_record(&each)).collect()
}
pub fn parse_record(input: &str) -> Record {
    let split = input.split_whitespace().collect::<Vec<&str>>();
    let springs: Vec<SpringType> = split
        .first()
        .unwrap()
        .chars()
        .map(|c| SpringType::from_char(c))
        .collect();

    let numbers = split
        .last()
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|each| each.parse::<i32>().unwrap())
        .collect();
    Record::new(springs, numbers)
}
