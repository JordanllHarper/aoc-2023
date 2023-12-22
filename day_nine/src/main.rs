use std::fs::read_to_string;

use parser::parse_to_number_line;

use crate::solve::{work_out_before_first, work_out_differences_in_sequence};

mod models;
mod parser;
mod solve;
fn main() {
    let input = read_lines("input.txt");
    let output = part_two(input);
    println!("{}", output);
}

fn part_one(input: Vec<String>) -> i64 {
    let number_lines = parse_to_number_line(input);
    let finals = number_lines
        .clone()
        .into_iter()
        .map(|line| work_out_differences_in_sequence(line))
        .collect::<Vec<i64>>();
    number_lines
        .iter()
        .zip(finals.iter())
        .map(|(nums, finals)| nums.last().unwrap() + finals)
        .sum()
}

// TODO:
fn part_two(input: Vec<String>) -> i64 {
    let number_lines = parse_to_number_line(input);
    number_lines
        .into_iter()
        .map(|each| work_out_before_first(each))
        .sum()
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
        let data = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let expected = 114;
        let actual = part_one(
            data.lines()
                .map(|each| each.to_string())
                .collect::<Vec<String>>(),
        );
        assert_eq!(expected, actual);
    }
    // TODO:
    #[test]
    fn part_two_test() {
        let data = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let expected = 2;
        let actual = part_two(data.lines().map(|each| each.to_string()).collect());
        assert_eq!(expected, actual);
    }
}
