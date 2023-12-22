use crate::models::TimeToDistance;

pub fn parse_time_to_distance(input: &Vec<String>) -> Vec<TimeToDistance> {
    let times = parse_line(input.first().unwrap());
    let distances = parse_line(input.last().unwrap());
    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| TimeToDistance::new(*time, *distance))
        .collect()
}
pub fn parse_line(line: &str) -> Vec<i64> {
    line.split(':')
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|each| each.parse::<i64>().unwrap())
        .collect()
}
pub fn calculate_ways_to_win_and_multiply(races: Vec<TimeToDistance>) -> i64 {
    races
        .iter()
        .map(|winning| calculate_one_race_ways_to_win(winning))
        .product()
}
pub fn calculate_one_race_ways_to_win(race: &TimeToDistance) -> i64 {
    let time = race.time;
    let best_distance = race.best_distance;
    let num_ways = (0..=time)
        .map(|hold| calculate_total_distance(hold, time))
        .filter(|result| result > &best_distance)
        .count()
        .try_into()
        .unwrap();

    num_ways
}
pub fn calculate_total_distance(button_held_millis: i64, race_allowed_millis: i64) -> i64 {
    let remaining_movement_millis = race_allowed_millis - button_held_millis;

    let remaining_movement_millis = if remaining_movement_millis.is_negative() {
        0
    } else {
        remaining_movement_millis
    };

    println!(
        "Calculation {}",
        button_held_millis * remaining_movement_millis
    );
    button_held_millis * remaining_movement_millis
}
