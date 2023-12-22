use std::fs::read_to_string;

use crate::solve::{calculate_ways_to_win_and_multiply, parse_time_to_distance};

mod models;
mod solve;
fn main() {
    let input = read_lines("input.txt");
    let output = part_two(input);
    println!("{}", output);
}

fn part_one(input: Vec<String>) -> i64 {
    let pairs = parse_time_to_distance(&input);
    let result = calculate_ways_to_win_and_multiply(pairs);
    result
}

// TODO:
fn part_two(input: Vec<String>) -> i64 {
    let pairs = parse_time_to_distance(&input);
    let result = calculate_ways_to_win_and_multiply(pairs);
    result
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
        let data = "Time:      7  15   30\nDistance:  9  40  200"
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        let expected: i64 = 288;
        let actual = part_one(data);

        assert_eq!(expected, actual);
    }
    // TODO:
    #[test]
    fn part_two_test() {}
}
